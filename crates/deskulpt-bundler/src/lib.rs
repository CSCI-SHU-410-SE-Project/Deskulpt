//! Bundling utilities for Deskulpt widgets.

#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::{bail, Error};
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
    root: PathBuf,
    entry: PathBuf,
    apis_blob_url: String,
    external_deps: HashSet<String>,
}

impl WidgetBundler {
    /// Create a new bundler instance.
    ///
    /// Note that `root` and `entry` should be absolute paths, and `entry` must
    /// be within the `root` directory.
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
    fn bundle_into_raw_module(
        &self,
        globals: &Globals,
        cm: Lrc<SourceMap>,
    ) -> Result<Module, Error> {
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
    pub fn bundle(&self) -> Result<String, Error> {
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

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, read_to_string};

    use deskulpt_test_testing::assert::{assert_eq, assert_err_eq, ChainReason};
    use deskulpt_test_testing::fixture_path;
    use rstest::rstest;
    use tempfile::tempdir;

    use super::*;

    #[rstest]
    // Use correct runtime for `jsx`, `jsxs`, and `Fragment`
    #[case::jsx_runtime("jsx_runtime", "index.jsx")]
    // Correctly resolve JS/JSX imports with and without extensions, or as index files
    // of a directory
    #[case::import("import", "index.jsx")]
    // Correctly strip off TypeScript syntax
    #[case::strip_types("strip_types", "index.tsx")]
    // Do not resolve imports from default dependencies, and that `@deskulpt-test/apis`
    // should be replaced with the blob URL
    #[case::default_deps("default_deps", "index.js")]
    fn test_bundle_ok(#[case] case: &str, #[case] entry: &str) {
        let case_dir = fixture_path("deskulpt-bundler/widgets").join(case);
        let bundle_root = case_dir.join("input");
        let bundler = WidgetBundler::new(
            bundle_root.clone(),
            bundle_root.join(entry),
            "blob://dummy-url".to_string(),
            Default::default(),
        );

        let result = bundler.bundle().expect("Expected bundling to succeed");
        let expected = read_to_string(case_dir.join("output.js")).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    // Node modules import that are not specified as external dependencies
    #[case::import_node_modules(
        "import_node_modules",
        vec![
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Regex("failed to resolve os-name from".to_string()),
            ChainReason::Exact(
                "node_modules imports should be explicitly included in package.json to \
                avoid being bundled at runtime; URL imports are not supported, one \
                should vendor its source to local and use a relative import instead"
                .to_string()
            ),
        ]
    )]
    // URL import
    #[case::import_url(
        "import_url",
        vec![
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Regex("failed to resolve https://foo.js from".to_string()),
            ChainReason::Exact(
                "node_modules imports should be explicitly included in package.json to \
                avoid being bundled at runtime; URL imports are not supported, one \
                should vendor its source to local and use a relative import instead"
                .to_string()
            ),
        ]
    )]
    // Relative import that goes beyond the root
    #[case::import_beyond_root(
        "import_beyond_root",
        vec![
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Regex("failed to resolve ../../foo from".to_string()),
            ChainReason::Regex("Relative imports should not go beyond the root".to_string()),
        ]
    )]
    // Entry file does not exist
    #[case::entry_not_exist(
        "entry_not_exist",
        vec![ChainReason::Regex("Entry point does not exist".to_string())],
    )]
    // Bad syntax that cannot be parsed
    #[case::bad_syntax(
        "bad_syntax",
        vec![
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Regex("error: Expected ';', '}' or <eof>".to_string()),
        ]
    )]
    fn test_bundle_error(#[case] case: &str, #[case] expected_error: Vec<ChainReason>) {
        let case_dir = fixture_path("deskulpt-bundler/widgets").join(case);
        let bundle_root = case_dir.join("input");
        let bundler = WidgetBundler::new(
            bundle_root.clone(),
            bundle_root.join("index.jsx"),
            Default::default(),
            Default::default(),
        );

        let error = bundler.bundle().expect_err("Expected bundling error");
        assert_err_eq(error, expected_error);
    }

    #[rstest]
    #[should_panic]
    fn test_bundle_import_meta_panic() {
        // Test that accessing `import.meta` is not supported
        let bundle_root = fixture_path("deskulpt-bundler/widgets/import_meta/input");
        let bundler = WidgetBundler::new(
            bundle_root.clone(),
            bundle_root.join("index.jsx"),
            Default::default(),
            Default::default(),
        );
        let _ = bundler.bundle();
    }

    #[rstest]
    fn test_bundle_absolute_import_error() {
        // Test that an absolute import raises a proper error
        let temp_dir = tempdir().unwrap();
        create_dir(temp_dir.path().join("input")).unwrap();

        // Create the following structure in the temporary directory:
        //     input/
        //       ├─ index.jsx  (imports utils.js via absolute path)
        //       └─ utils.js
        // Note that the absolute path we used the debugging format otherwise the
        // backslashes on Windows would not be escaped properly
        let bundle_root = temp_dir.path().join("input");
        let index_path = bundle_root.join("index.jsx");
        let utils_path = bundle_root.join("utils.js");
        std::fs::write(
            &index_path,
            format!("import {{ foo }} from {utils_path:?}; console.log(foo);"),
        )
        .unwrap();
        std::fs::write(&utils_path, "export const foo = 42;").unwrap();

        // Test the bundling error
        let bundler = WidgetBundler::new(
            bundle_root,
            index_path,
            Default::default(),
            Default::default(),
        );
        let error = bundler.bundle().expect_err("Expected bundling error");
        let expected_error = vec![
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Exact(
                "Absolute imports are not supported; use relative imports instead".to_string(),
            ),
        ];
        assert_err_eq(error, expected_error);
    }
}
