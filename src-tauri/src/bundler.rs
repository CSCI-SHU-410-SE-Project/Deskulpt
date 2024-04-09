//! The module implements the Deskulpt bundler based on SWC.
//!
//! Note that this is not a general-purpose bundler; it is specifically designed for
//! the use case of bundling Deskulpt widgets which has a custom set of dependency
//! rules and are (at least recommended to be) small.

use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Component, Path, PathBuf},
};

use anyhow::{bail, Context, Error};
use path_clean::PathClean;
use swc_atoms::Atom;
use swc_bundler::{Bundler, Hook, Load, ModuleData, ModuleRecord, Resolve};
use swc_common::{
    comments::SingleThreadedComments, errors::Handler, pass::Repeat, sync::Lrc,
    FileName, FilePathMapping, Globals, Mark, SourceMap, Span, GLOBALS,
};
use swc_ecma_ast::KeyValueProp;
use swc_ecma_codegen::{
    text_writer::{JsWriter, WriteJs},
    Emitter,
};
use swc_ecma_loader::resolve::Resolution;
use swc_ecma_parser::{parse_file_as_module, EsConfig, Syntax};
use swc_ecma_transforms_optimization::simplify::dce;
use swc_ecma_transforms_react::jsx;
use swc_ecma_visit::FoldWith;
use tempfile::NamedTempFile;

#[cfg(windows)]
use normpath::BasePath;

/// The file extensions to try when an import is given without an extension
static EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];

/// Bundle a widget into a single ESM string given its entry point.
///
/// The `dependency_map` argument is an optional mapping with keys being the module
/// specifiers to ignore. The import statements with these module specifiers will be
/// left as is in the bundled code without path resolution. This should commonly be the
/// list of external dependencies, since Deskulpt requires widget developers to bundle
/// their external dependencies (if any) to be included directly in the Webview.
pub(crate) fn bundle(
    root: &Path,
    target: &Path,
    dependency_map: Option<&HashMap<String, String>>,
) -> Result<String, Error> {
    let globals = Globals::default();
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

    // Get the list of external modules not to resolve
    let external_modules = match dependency_map {
        Some(map) => map.keys().map(|k| Atom::from(k.clone())).collect(),
        None => vec![],
    };

    let mut bundler = Bundler::new(
        &globals,
        cm.clone(),
        PathLoader(cm.clone()),
        // The path resolver produces paths with the \\?\ prefix on Windows, and since
        // we need to compare paths with the root we canonicalize the root path here to
        // get the same prefix; XXX not sure if there will be symlink issues
        PathResolver { root: root.canonicalize()?.to_path_buf() },
        // We must disabled the default tree-shaking by the SWC bundler, otherwise it
        // will remove unused `React` variables, which is required by the JSX transform
        swc_bundler::Config {
            external_modules,
            disable_dce: true,
            ..Default::default()
        },
        Box::new(NoopHook),
    );

    // SWC bundler requires a map of entries to bundle; we provide a single entry point
    // and expect there to be only one generated bundle, as long as there are no dynamic
    // or conditional imports; we use the target path as the key for convenience
    let mut entries = HashMap::new();
    entries.insert(
        target.to_string_lossy().to_string(),
        FileName::Real(target.to_path_buf()),
    );

    let mut bundles = bundler.bundle(entries)?;
    if bundles.len() != 1 {
        bail!("Expected a single bundle; try to remove dynamic/conditional imports");
    }
    let module = bundles.pop().unwrap().module;

    let code = GLOBALS.set(&globals, || {
        // Tree-shaking optimization in the bundler is disabled, so we need to manually
        // apply the transform; we need to retain the top level mark `React` because it
        // is needed by the JSX transform even if not explicitly used in the code
        let mut tree_shaking = Repeat::new(dce::dce(
            dce::Config { top_retain: vec![Atom::from("React")], ..Default::default() },
            Mark::new(),
        ));

        // There are two types of JSX transforms ("classic" and "automatic"), see
        // https://legacy.reactjs.org/blog/2020/09/22/introducing-the-new-jsx-transform.html
        //
        // The "automatic" transform automatically imports from "react/jsx-runtime", but
        // this module is not available when runnning the bundled code in the browser,
        // so we have to use the "classic" transform instead. The "classic" transform
        // requires `React` to be in scope, which we can require users to bring into
        // scope by assigning `const React = window.__DESKULPT__.React`.
        let mut jsx_transform = jsx::<SingleThreadedComments>(
            cm.clone(),
            None,
            Default::default(), // options, where runtime defaults to "classic"
            Mark::new(),        // top level mark
            Mark::new(),        // unresolved mark
        );

        // Apply the module transformations
        // @Charlie-XIAO: chain more transforms e.g. TypeScript
        let module = module.fold_with(&mut tree_shaking).fold_with(&mut jsx_transform);

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

/// Deskulpt-customized path loader for SWC bundler.
///
/// It is in charge of parsing the source file into a module AST. Note that transforms
/// are not applied here to avoid messing up per-file ASTs that can cause unexpected
/// bundling results.
struct PathLoader(Lrc<SourceMap>);

impl Load for PathLoader {
    fn load(&self, file: &FileName) -> Result<ModuleData, Error> {
        let fm = match file {
            FileName::Real(path) => self.0.load_file(path)?,
            _ => unreachable!(),
        };

        // @Charlie-XIAO: maybe need to use Syntax::TypeScript based on file extension
        let syntax = Syntax::Es(EsConfig { jsx: true, ..Default::default() });

        // Parse the file as a module; note that transformations are not applied here,
        // because per-file transformations may lead to unexpected results when bundled
        // together; instead, transformations are postponed until the bundling phase
        match parse_file_as_module(&fm, syntax, Default::default(), None, &mut vec![]) {
            Ok(module) => Ok(ModuleData { fm, module, helpers: Default::default() }),
            Err(err) => {
                // The error handler requires a destination for the emitter writer that
                // implements `Write`; a buffer implements `Write` but its borrowed
                // reference does not, causing the handler to take ownership of the
                // buffer, making us unable to read from it later (and the buffer is
                // made private in the handler); the workaround is to use a temporary
                // file and access its content later by its path (we require the file to
                // live only for a short time so this is relatively safe)
                let mut err_msg = String::new();
                {
                    let context = format!(
                        "Parsing error occurred but failed to emit the formatted error \
                        analysis; falling back to raw version: {:?}",
                        err
                    );
                    let buffer = NamedTempFile::new().context(context.clone())?;
                    let buffer_path = buffer.path().to_path_buf();
                    let handler = Handler::with_emitter_writer(
                        Box::new(buffer),
                        Some(self.0.clone()),
                    );
                    err.into_diagnostic(&handler).emit();
                    File::open(buffer_path)
                        .context(context.clone())?
                        .read_to_string(&mut err_msg)
                        .context(context.clone())?;
                }
                bail!(err_msg);
            },
        }
    }
}

/// The Deskulpt-customized path resolver for SWC bundler.
///
/// It is in charge of resolving the module specifiers in the import statements. Note
/// that module specifiers that are ignored in the first place will not go through this
/// resolver at all.
///
/// This path resolver intends to resolve the following types of imports:
///
/// - Extension-less relative paths, e.g., `import foo from "./foo"`
/// - Relative paths, e.g., `import foo from "./foo.js"`
///
/// It is not designed to resolve the following types of imports:
///
/// - Absolute path imports, e.g., `import foo from "/foo"`
/// - URL imports, e.g., `import foo from "https://example.com/foo"`
/// - Node resolution imports, e.g., `import globals from "globals"`
/// - Relative imports that go beyond the root
struct PathResolver {
    root: PathBuf,
}

impl PathResolver {
    /// Wrap a resolved module path if specified, otherwise raise an error.
    fn wrap(&self, path: Option<PathBuf>) -> Result<FileName, Error> {
        if let Some(path) = path {
            return Ok(FileName::Real(path.clean()));
        }
        bail!("File resolution failed")
    }

    // Resolve a path as a file; if `path` refers to a file then it is directly
    // returned; otherwise, `path` with each extension is tried

    /// Resolve a path as a file.
    ///
    /// If `path` refers to a file then it is directly returned. Otherwise, `path` with
    /// each extension in [`EXTENSIONS`] is tried in order.
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

    /// Resolve a path as a directory.
    ///
    /// This essentially resolves `${path}/index` as a file. Note that it does not try
    /// any node resolution, e.g., looking for the `main` field in `package.json`.
    fn resolve_as_directory(&self, path: &Path) -> Result<Option<PathBuf>, Error> {
        for ext in EXTENSIONS {
            let ext_path = path.join(format!("index.{}", ext));
            if ext_path.is_file() {
                return Ok(Some(ext_path));
            }
        }
        Ok(None)
    }

    /// Helper function for the [`Resolve`] trait.
    fn resolve_filename(
        &self,
        base: &FileName,
        module_specifier: &str,
    ) -> Result<FileName, Error> {
        let base = match base {
            FileName::Real(v) => v,
            _ => bail!("Invalid base for resolution: {}", base),
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
            bail!("Absolute imports are not supported; use relative imports instead");
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

            // Reject paths that go beyond the root
            if !path.starts_with(&self.root) {
                bail!("Relative imports should not go beyond the root {:?}", self.root);
            }

            return self
                .resolve_as_file(&path)
                .or_else(|_| self.resolve_as_directory(&path))
                .and_then(|p| self.wrap(p));
        }
        bail!(
            "node_modules imports should be explicitly included in package.json to \
            avoid being bundled at runtime; URL imports are not supported, one should \
            vendor its source to local and use a relative import instead"
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

/// A no-op hook for SWC bundler.
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

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;
    use pretty_assertions::assert_eq;
    use std::{
        fs::{read_to_string, remove_file, File},
        io::Write,
    };

    /// Assert that an [`Error`] object has the expected chain of reasons.
    fn assert_err_eq(error: Error, chain: Vec<String>) {
        let mut error_chain = error.chain();
        for expected_msg in chain {
            let reason = error_chain.next().expect("Expected more reasons");
            assert_eq!(format!("{reason}"), expected_msg, "{reason:?}");
        }
        // Assert that the chain of reasons ends here
        assert_eq!(error_chain.next().map(|msg| format!("{msg}")), None);
    }

    #[parameterized(case = {
        "no_react",
        "with_react_hook",
        "import_relative",
        "import_relative_no_ext",
        "import_relative_directory",
        "import_relative_no_ext_jsx",
        "import_no_inner_react_def",
        "import_no_outer_react_def",
    })]
    fn test_bundle_basic(case: &str) {
        // Test the most basic bundler functionalities, with no dependencies and are
        // expected to succeed
        let root = PathBuf::from(format!("tests/fixtures/bundler/{case}/input"))
            .canonicalize()
            .unwrap();
        let result = bundle(&root, root.join("index.jsx").as_path(), None)
            .expect("Failed to bundle");

        let expected =
            read_to_string(format!("tests/fixtures/bundler/{case}/output.js")).unwrap();
        self::assert_eq!(result, expected);
    }

    #[test]
    fn test_bundle_absolute_import_error() {
        // Test that we do not allow absolute path import
        let root = Path::new("tests/fixtures/bundler/import_absolute/input")
            .canonicalize()
            .unwrap();
        let entry = root.join("index.jsx");

        // We need to write the entry file here because absolute paths cannot be
        // determined statically
        let import_file = root.join("utils.js");
        {
            let mut entry_file = File::create(&entry).unwrap();
            // Debug print of the import path will automatically include the quotes
            writeln!(entry_file, "import foo from {import_file:?};").unwrap();
        }

        let error = bundle(&root, &entry, None).expect_err("Expected bundling error");
        let expected = vec![
            "load_transformed failed".to_string(),
            "failed to analyze module".to_string(),
            format!(
                "failed to resolve {} from {}",
                import_file.to_string_lossy(),
                entry.to_string_lossy()
            ),
            "Absolute imports are not supported; use relative imports instead"
                .to_string(),
        ];
        assert_err_eq(error, expected);

        // Clean up the entry file
        remove_file(&entry).unwrap();
    }

    #[test]
    fn test_bundle_beyond_root_error() {
        // Test that we do not allow relative import that goes beyond the root path
        let root = Path::new("tests/fixtures/bundler/import_beyond_root/input")
            .canonicalize()
            .unwrap();
        let entry = root.join("index.jsx");
        let error = bundle(&root, &entry, None).expect_err("Expected bundling error");

        let expected = vec![
            "load_transformed failed".to_string(),
            "failed to analyze module".to_string(),
            format!(
                "failed to resolve ../../../dummy from {}",
                entry.to_string_lossy()
            ),
            format!("Relative imports should not go beyond the root {root:?}"),
        ];
        assert_err_eq(error, expected);
    }
}
