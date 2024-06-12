//! This module implements the path loader for the bundler.

use super::transforms::ExternalImportResolver;
use anyhow::{bail, Context, Error};
use std::{collections::HashMap, fs::File, io::Read, path::Path};
use swc_bundler::{Load, ModuleData};
use swc_common::{errors::Handler, pass::Optional, sync::Lrc, FileName, SourceMap};
use swc_ecma_parser::{parse_file_as_module, EsConfig, Syntax, TsConfig};
use swc_ecma_visit::{as_folder, FoldWith};
use tempfile::NamedTempFile;

/// Deskulpt-customized path loader for SWC bundler.
///
/// It is in charge of loading each source file and parsing them into module ASTs.
pub(super) struct PathLoader<'a> {
    /// The root directory of the widget to bundle.
    pub(super) root: &'a Path,
    /// The source map.
    pub(super) cm: Lrc<SourceMap>,
    /// The external dependencies of the widget.
    pub(super) dependency_map: &'a HashMap<String, String>,
    /// Whether to replace the import statements of external dependencies.
    ///
    /// If true, the import statements will be replaced with named imports from the
    /// bundle of external dependencies, assuming that it exists.
    pub(super) replace_external: bool,
}

impl Load for PathLoader<'_> {
    fn load(&self, file: &FileName) -> Result<ModuleData, Error> {
        let path = match file {
            FileName::Real(path) => path,
            _ => unreachable!(),
        };
        let fm = self.cm.load_file(path)?;

        // Determine language syntax based on file extension; JSX syntax is always
        // enabled since it does not affect the parsing of non-JSX files
        let syntax = match path.extension() {
            Some(ext) if ext == "ts" || ext == "tsx" => {
                Syntax::Typescript(TsConfig { tsx: true, ..Default::default() })
            },
            _ => Syntax::Es(EsConfig { jsx: true, ..Default::default() }),
        };

        // Parse the file as a module; most AST transforms are postponed until the
        // bundling phase to avoid messing up per-file ASTs
        match parse_file_as_module(&fm, syntax, Default::default(), None, &mut vec![]) {
            Ok(module) => {
                // Resolve external imports into named imports from the external bundle
                // if the purpose is to bundle the widget code; when trying to bundle
                // external dependencies we should leave external imports as is
                let mut resolver = Optional {
                    visitor: as_folder(ExternalImportResolver {
                        root: self.root,
                        external_dependencies: self.dependency_map,
                    }),
                    enabled: self.replace_external && !self.dependency_map.is_empty(),
                };
                let module = module.fold_with(&mut resolver);
                Ok(ModuleData { fm, module, helpers: Default::default() })
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
                        "Parsing error occurred but failed to emit the formatted error \
                        analysis; falling back to raw version: {err:?}"
                    );
                    let buffer = NamedTempFile::new().context(context.clone())?;
                    let buffer_path = buffer.path().to_path_buf();
                    let handler = Handler::with_emitter_writer(
                        Box::new(buffer),
                        Some(self.cm.clone()),
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
