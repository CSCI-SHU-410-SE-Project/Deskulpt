use std::{collections::HashMap, path::{Component, Path, PathBuf}};

use anyhow::{bail, Error};
use path_clean::PathClean;
use serde::Serialize;
use swc_atoms::Atom;
use swc_bundler::{Bundler, Hook, Load, ModuleData, ModuleRecord, Resolve};
use swc_common::{
    comments::SingleThreadedComments,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    FileName, FilePathMapping, Globals, Mark, SourceMap, Span, GLOBALS,
};
use swc_ecma_ast::KeyValueProp;
use swc_ecma_codegen::{text_writer::{JsWriter, WriteJs}, Emitter};
use swc_ecma_loader::resolve::Resolution;
use swc_ecma_parser::{parse_file_as_module, EsConfig, Syntax};
use swc_ecma_transforms_react::jsx;
use swc_ecma_visit::FoldWith;

#[cfg(windows)]
use normpath::BasePath;

// The file extensions that are recognized by the bundler
static EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];

pub(crate) enum BundlerOutput {
    Code(String),
    Error(String),
}

pub(crate) fn bundle(
    target: &PathBuf,
    dependency_map: Option<&HashMap<String, String>>,
) -> BundlerOutput {
    match bundle_internal(target, dependency_map) {
        Ok(code) => BundlerOutput::Code(code),
        Err(err) => BundlerOutput::Error(format!("{:#?}", err)),
    }
}

// Treat the target file as an entry point and bundle it into a single module that can
// be recognized by the <script> tag in a browser; `dependency_map` is the mapping that
// should be read from the configuration file
fn bundle_internal(
    target: &PathBuf,
    dependency_map: Option<&HashMap<String, String>>,
) -> Result<String, Error> {
    let globals = Globals::default();
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

    // Get the external modules that should not be resolved and bundled, but rather be
    // left as `import` statements as is
    let external_modules = match dependency_map {
        Some(map) => map.keys().map(|k| Atom::from(k.clone())).collect(),
        None => vec![],
    };

    let mut bundler = Bundler::new(
        &globals,
        cm.clone(),
        PathLoader(cm.clone()),
        PathResolver,
        swc_bundler::Config { external_modules, ..Default::default() },
        Box::new(NoopHook),
    );

    // Bundle the target file into bundles, each containing the merged module
    let mut entries = HashMap::new();
    entries
        .insert(target.to_string_lossy().to_string(), FileName::Real(target.clone()));
    let mut bundles = bundler.bundle(entries)?;

    // Since we provide a single entry point, there should be only one generated
    // bundle (as long as there are no dynamic/conditional imports)
    if bundles.len() != 1 {
        bail!("Expected a single bundle; try to remove dynamic/conditional imports");
    }
    let module = bundles.pop().unwrap().module;

    let code = GLOBALS.set(&globals, || {
        // Notes on the JSX transform
        // ==========================
        //
        // There are two types of JSX transforms ("classic" and "automatic"), see
        // https://legacy.reactjs.org/blog/2020/09/22/introducing-the-new-jsx-transform.html
        //
        // The "automatic" transform automatically imports from "react/jsx-runtime", but
        // this module is not available when runnning the bundled code in the browser,
        // so we have to use the "classic" transform instead. The "classic" transform
        // requires `React` to be in scope, which we can require users to bring into
        // scope by assigning `const React = window.__DESKULPT__.React`.
        //
        // Note, however, that this puts constraints on how we can minify the bundled
        // code, e.g., we cannot mangle the `React` identifier, we cannot remove `React`
        // even if it is unused, etc.

        let mut jsx_transform = jsx::<SingleThreadedComments>(
            cm.clone(),
            None,
            Default::default(), // options, where runtime defaults to "classic"
            Mark::new(),        // top level mark
            Mark::new(),        // unresolved mark
        );

        // TODO: maybe need to chain more transforms, e.g., TypeScript transform
        let module = module.fold_with(&mut jsx_transform);

        // Emit the bundled module as string into a buffer
        let mut buf = vec![];
        {
            let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
            let mut emitter = Emitter {
                cfg: swc_ecma_codegen::Config::default().with_minify(true),
                cm: cm.clone(),
                comments: None,
                wr: Box::new(wr) as Box<dyn WriteJs>,
            };
            emitter.emit_module(&module).unwrap();
        }
        String::from_utf8_lossy(&buf).to_string()
    });

    Ok(code)
}

struct PathLoader(Lrc<SourceMap>);

impl Load for PathLoader {
    fn load(&self, file: &FileName) -> Result<ModuleData, Error> {
        let fm = match file {
            FileName::Real(path) => self.0.load_file(path)?,
            _ => unreachable!(),
        };

        // TODO: maybe need to use Syntax::TypeScript based on file extension
        let syntax = Syntax::Es(EsConfig { jsx: true, ..Default::default() });

        // Parse the file as a module; note that transformations are not applied here,
        // because per-file transformations may lead to unexpected results when bundled
        // together; instead, transformations are postponed until the bundling phase
        let module =
            parse_file_as_module(&fm, syntax, Default::default(), None, &mut vec![])
                .unwrap_or_else(|err| {
                    let handler = Handler::with_tty_emitter(
                        ColorConfig::Auto,
                        true,  // allow emitting warnings
                        false, // do not treat errors as bugs
                        Some(self.0.clone()),
                    );
                    err.into_diagnostic(&handler).emit();
                    panic!("FATAL: Failed to parse module");
                });

        Ok(ModuleData { fm, module, helpers: Default::default() })
    }
}

// Based on the implementation of `NodeModulesResolver`; for reference see:
// https://github.com/swc-project/swc/blob/de09c55ffac8610e7128ced2d4b273d9fba1fdd2/crates/swc_ecma_loader/src/resolvers/node.rs
struct PathResolver;

impl PathResolver {
    // Wrap a resolved path if possible or else error out directly
    fn wrap(&self, path: Option<PathBuf>) -> Result<FileName, Error> {
        if let Some(path) = path {
            return Ok(FileName::Real(path.clean()));
        }
        bail!("File resolution failed")
    }

    // Resolve a path as a file; if `path` refers to a file then it is directly
    // returned; otherwise, `path` with each extension is tried
    fn resolve_as_file(&self, path: &Path) -> Result<Option<PathBuf>, Error> {
        if path.is_file() {
            // Early return if `path` is directly a file
            return Ok(Some(path.to_path_buf()));
        }

        if let Some(name) = path.file_name() {
            let mut ext_path = path.to_path_buf();
            let name = name.to_string_lossy();

            // Try all extensions we support for importing
            for ext in EXTENSIONS {
                ext_path.set_file_name(format!("{}.{}", name, ext));
                if ext_path.is_file() {
                    return Ok(Some(ext_path));
                }
            }
        }
        bail!("File resolution failed: {:?}", path)
    }

    // Resolve a path as a directory; normally one should consider using the "main" key
    // from package.json if it exists, but here we do not support this use case; we only
    // try to resolve the index file of the directory with trying each extension
    fn resolve_as_directory(&self, path: &Path) -> Result<Option<PathBuf>, Error> {
        for ext in EXTENSIONS {
            let ext_path = path.join(format!("index.{}", ext));
            if ext_path.is_file() {
                return Ok(Some(ext_path));
            }
        }
        Ok(None)
    }

    // Helper function for the Resolve trait
    fn resolve_filename(
        &self,
        base: &FileName,
        module_specifier: &str,
    ) -> Result<FileName, Error> {
        let base = match base {
            FileName::Real(v) => v,
            _ => bail!("Invalid base for resolution: {:?}", base),
        };

        // Determine the base directory (or `base` itself if already a directory)
        let base_dir = if base.is_file() {
            // If failed to get the parent directory then use the cwd
            base.parent().unwrap_or_else(|| Path::new("."))
        } else {
            base
        };

        let spec_path = Path::new(module_specifier);
        // Absolute paths are not supported
        if spec_path.is_absolute() {
            bail!(
                "Invalid module specifier {:?} in base {:?}; absolute imports are not \
                supported, please use relative imports instead",
                module_specifier,
                base,
            )
        }

        // If not absolute, then it should be either relative, a node module, or a URL;
        // we support only relative import among these types
        let mut components = spec_path.components();
        if let Some(Component::CurDir | Component::ParentDir) = components.next() {
            // Workaround for the fs::canonicalize issue on Windows; normalization is
            // usually a better choice unless one specifically needs a canonical path;
            // note that `normalize_virtually` is an equivalent of `normalize` without
            // accessing the file system
            #[cfg(windows)]
            let path = {
                let base_dir = BasePath::new(base_dir).unwrap();
                base_dir
                    .join(module_specifier)
                    .normalize_virtually()
                    .unwrap()
                    .into_path_buf()
            };
            // Perform a simple join on Unix-like systems; as mentioned canonicalization
            // is not preferred but Rust does not provide a better alternative, and
            // Unix-like systems do not provide canonicalization functionality without
            // file system access
            #[cfg(not(windows))]
            let path = base_dir.join(module_specifier);

            return self
                .resolve_as_file(&path)
                .or_else(|_| self.resolve_as_directory(&path))
                .and_then(|p| self.wrap(p));
        }
        bail!(
            "Invalid module specifier {:?} in base {:?}; node_modules imports should \
            be explicitly included in package.json to avoid being bundled at runtime;\
            URL imports are not supported, one should vendor its source to local and \
            use a relative import instead",
            module_specifier,
            base,
        )
    }
}

impl Resolve for PathResolver {
    fn resolve(
        &self,
        base: &FileName,
        module_specifier: &str,
    ) -> Result<Resolution, Error> {
        self.resolve_filename(base, module_specifier)
            .map(|filename| Resolution { filename, slug: None })
    }
}

struct NoopHook;

impl Hook for NoopHook {
    fn get_import_meta_props(
        &self,
        _: Span,
        _: &ModuleRecord,
    ) -> Result<Vec<KeyValueProp>, Error> {
        unimplemented!();
    }
}