//! Bundler for Deskulpt widgets.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::{bail, Result};
use common::{ModuleExt, NoopHook, PathLoader, PathResolver};
use swc_core::atoms::Atom;
use swc_core::bundler::Bundler;
use swc_core::common::sync::Lrc;
use swc_core::common::{FileName, FilePathMapping, Globals, SourceMap, GLOBALS};
use swc_core::ecma::ast::{Module, Program};
use swc_core::ecma::visit::visit_mut_pass;

mod common;
mod transforms;

/// The Deskulpt widget bundler.
pub struct WidgetBundler {
    /// Absolute path to the widget directory.
    root: PathBuf,
    /// Absolute path to the entry file of the widget.
    ///
    /// This file must be within the `root` directory and share the exact same
    /// path prefix.
    entry: PathBuf,
    /// URL to the widget APIs blob.
    apis_blob_url: String,
    /// External dependencies.
    external_deps: HashSet<String>,
}

impl WidgetBundler {
    /// Create a new bundler instance.
    ///
    /// Note that `root` and `entry` must be absolute paths, and `entry` must
    /// be within the `root` directory and share the exact same path prefix.
    pub fn new(
        root: PathBuf,
        entry: PathBuf,
        apis_blob_url: String,
        external_deps: HashSet<String>,
    ) -> Self {
        Self {
            root,
            entry,
            apis_blob_url,
            external_deps,
        }
    }

    /// Bundle the widget into a raw module.
    ///
    /// This does not apply any AST transforms and also leaves imports of
    /// default and external dependencies as is.
    fn bundle_into_raw_module(&self, globals: &Globals, cm: Lrc<SourceMap>) -> Result<Module> {
        if !self.entry.exists() {
            bail!("Entry point does not exist: '{}'", self.entry.display());
        }

        // Get the list of external modules not to resolve; this should include default
        // dependencies and (if any) external dependencies obtained from the dependency
        // map
        let external_modules = {
            let mut dependencies = HashSet::from([
                Atom::from("@deskulpt-test/apis"),
                Atom::from("@deskulpt-test/emotion/jsx-runtime"),
                Atom::from("@deskulpt-test/react"),
                Atom::from("@deskulpt-test/ui"),
            ]);
            dependencies.extend(self.external_deps.iter().map(|k| Atom::from(k.clone())));
            Vec::from_iter(dependencies)
        };

        // The root of the path resolver will be used to determine whether a resolved
        // import goes beyond the root; the comparison is done via path prefixes so
        // we must be consistent with how SWC resolves paths, see:
        // https://github.com/swc-project/swc/blob/f584ef76d75e86da15d0725ac94be35a88a1c946/crates/swc_bundler/src/bundler/mod.rs#L159-L166
        #[cfg(target_os = "windows")]
        let path_resolver_root = self.root.canonicalize()?;
        #[cfg(not(target_os = "windows"))]
        let path_resolver_root = self.root.to_path_buf();

        let mut bundler = Bundler::new(
            globals,
            cm.clone(),
            PathLoader(cm.clone()),
            PathResolver(path_resolver_root),
            // Do not resolve the external modules
            swc_core::bundler::Config {
                external_modules,
                ..Default::default()
            },
            Box::new(NoopHook),
        );

        // SWC bundler requires a map of entries to bundle; we provide a single entry
        // point and expect there to be only one generated bundle; we use the target
        // path as the key for convenience
        let mut entries = HashMap::new();
        entries.insert(
            self.entry.to_string_lossy().to_string(),
            FileName::Real(self.entry.to_path_buf()),
        );

        let mut bundles = bundler.bundle(entries)?;
        if bundles.len() != 1 {
            bail!("Expected a single bundle, got {}", bundles.len());
        }
        Ok(bundles.pop().unwrap().module)
    }

    /// Bundle the widget into a single ESM code string.
    pub fn bundle(&self) -> Result<String> {
        let globals = Globals::default();
        let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

        let module = self.bundle_into_raw_module(&globals, cm.clone())?;
        let code = GLOBALS.set(&globals, || {
            // Redirect widget APIs imports to the APIs blob URL
            let rename_apis =
                visit_mut_pass(transforms::ApisImportRenamer(self.apis_blob_url.clone()));
            let program = Program::Module(module);
            let module = program.apply(rename_apis).expect_module();

            // Emit the bundled module as string into a buffer
            let mut buf = vec![];
            module.emit_to_buf(cm.clone(), &mut buf);
            String::from_utf8_lossy(&buf).to_string()
        });

        Ok(code)
    }
}
