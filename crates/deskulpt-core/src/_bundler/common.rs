//! This module contains common bundling routines and utilities.

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use anyhow::{bail, Context, Result};
use path_clean::PathClean;
use swc_core::bundler::{Hook, Load, ModuleData, ModuleRecord, Resolve};
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::errors::Handler;
use swc_core::common::sync::Lrc;
use swc_core::common::{FileName, Mark, SourceMap, Span};
use swc_core::ecma::ast::{KeyValueProp, Module, Program};
use swc_core::ecma::codegen::text_writer::{JsWriter, WriteJs};
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::loader::resolve::Resolution;
use swc_core::ecma::parser::{parse_file_as_module, EsSyntax, Syntax, TsSyntax};
use swc_core::ecma::transforms::react::react;
use swc_core::ecma::transforms::typescript::typescript;
use tempfile::NamedTempFile;

/// The file extensions to try when an import is given without an extension
static EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];

pub trait ModuleExt {
    /// Emit a module into a buffer.
    fn emit_to_buf<W: Write>(&self, cm: Lrc<SourceMap>, buf: W);
}

impl ModuleExt for Module {
    fn emit_to_buf<W: Write>(&self, cm: Lrc<SourceMap>, buf: W) {
        let wr = JsWriter::new(cm.clone(), "\n", buf, None);
        let mut emitter = Emitter {
            cfg: swc_core::ecma::codegen::Config::default().with_minify(true),
            cm: cm.clone(),
            comments: None,
            wr: Box::new(wr) as Box<dyn WriteJs>,
        };
        emitter.emit_module(self).unwrap();
    }
}

/// Deskulpt-customized path loader for SWC bundler.
///
/// It is in charge of parsing the source file into a module AST. TypeScript
/// types are stripped off and JSX syntax is transformed during the parsing.
pub struct PathLoader(pub Lrc<SourceMap>);

impl Load for PathLoader {
    fn load(&self, file: &FileName) -> Result<ModuleData> {
        let path = match file {
            FileName::Real(path) => path,
            _ => unreachable!(),
        };
        let fm = self.0.load_file(path)?;

        let syntax = match path.extension() {
            Some(ext) if ext == "ts" || ext == "tsx" => Syntax::Typescript(TsSyntax {
                tsx: true,
                ..Default::default()
            }),
            _ => Syntax::Es(EsSyntax {
                jsx: true,
                ..Default::default()
            }),
        };

        // Parse the file as a module
        match parse_file_as_module(&fm, syntax, Default::default(), None, &mut vec![]) {
            Ok(module) => {
                let unresolved_mark = Mark::new();
                let top_level_mark = Mark::new();

                // Strip off TypeScript types
                let ts_transform =
                    typescript::typescript(Default::default(), unresolved_mark, top_level_mark);

                // We use the automatic JSX transform (in contrast to the classic
                // transform) here so that there is no need to bring anything into scope
                // just for syntax which could be unused; to enable the `css` prop from
                // Emotion, we specify the import source to be `@deskulpt-test/emotion`,
                // so that the JSX runtime utilities will be automatically imported from
                // `@deskulpt-test/emotion/jsx-runtime`
                let jsx_transform = react::<SingleThreadedComments>(
                    self.0.clone(),
                    None,
                    swc_core::ecma::transforms::react::Options {
                        runtime: Some(swc_core::ecma::transforms::react::Runtime::Automatic),
                        import_source: Some("@deskulpt-test/emotion".to_string()),
                        ..Default::default()
                    },
                    top_level_mark,
                    unresolved_mark,
                );

                match Program::Module(module)
                    .apply(ts_transform)
                    .apply(jsx_transform)
                    .module()
                {
                    Some(module) => Ok(ModuleData {
                        fm,
                        module,
                        helpers: Default::default(),
                    }),
                    None => bail!("Failed to parse the file as a module"),
                }
            },
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
                        "Parsing error occurred but failed to emit the formatted error analysis; \
                         falling back to raw version: {err:?}"
                    );
                    let buffer = NamedTempFile::new().context(context.clone())?;
                    let buffer_path = buffer.path().to_path_buf();
                    let handler =
                        Handler::with_emitter_writer(Box::new(buffer), Some(self.0.clone()));
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
/// It is in charge of resolving the module specifiers in the import statements.
/// Note that module specifiers that are ignored in the first place will not go
/// through this resolver at all.
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
pub struct PathResolver(pub PathBuf);

impl PathResolver {
    /// Helper function for resolving a path by treating it as a file.
    ///
    /// If `path` refers to a file then it is directly returned. Otherwise,
    /// `path` with each extension in [`EXTENSIONS`] is tried in order.
    fn resolve_as_file(&self, path: &Path) -> Result<PathBuf> {
        if path.is_file() {
            // Early return if `path` is directly a file
            return Ok(path.to_path_buf());
        }

        if let Some(name) = path.file_name() {
            let mut ext_path = path.to_path_buf();
            let name = name.to_string_lossy();

            // Try all extensions we support for importing
            for ext in EXTENSIONS {
                ext_path.set_file_name(format!("{name}.{ext}"));
                if ext_path.is_file() {
                    return Ok(ext_path);
                }
            }
        }
        bail!("File resolution failed")
    }

    /// Helper function for the [`Resolve`] trait.
    ///
    /// Note that errors emitted here do not need to provide information about
    /// `base` and `module_specifier` because the call to this function
    /// should have already been wrapped in an SWC context that provides
    /// this information.
    fn resolve_filename(&self, base: &FileName, module_specifier: &str) -> Result<FileName> {
        let base = match base {
            FileName::Real(v) => v,
            _ => bail!("Invalid base for resolution: '{base}'"),
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
            let path = base_dir.join(module_specifier).clean();

            // Try to resolve by treating `path` as a file first, otherwise try by
            // looking for an `index` file under `path` as a directory
            let resolved_path = self
                .resolve_as_file(&path)
                .or_else(|_| self.resolve_as_file(&path.join("index")))?;

            // Reject if the resolved path goes beyond the root
            if !resolved_path.starts_with(&self.0) {
                bail!(
                    "Relative imports should not go beyond the root '{}'",
                    self.0.display(),
                );
            }
            return Ok(FileName::Real(resolved_path));
        }

        bail!(
            "node_modules imports should be explicitly included in package.json to avoid being \
             bundled at runtime; URL imports are not supported, one should vendor its source to \
             local and use a relative import instead"
        )
    }
}

impl Resolve for PathResolver {
    fn resolve(&self, base: &FileName, module_specifier: &str) -> Result<Resolution> {
        self.resolve_filename(base, module_specifier)
            .map(|filename| Resolution {
                filename,
                slug: None,
            })
    }
}

/// A no-op hook for SWC bundler.
pub struct NoopHook;

impl Hook for NoopHook {
    fn get_import_meta_props(&self, _: Span, _: &ModuleRecord) -> Result<Vec<KeyValueProp>> {
        // XXX: figure out a better way than panicking
        unimplemented!();
    }
}
