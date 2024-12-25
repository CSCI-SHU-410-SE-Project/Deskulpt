//! The module implements the Deskulpt bundler based on SWC.
//!
//! Note that this is not a general-purpose bundler; it is specifically designed
//! for the use case of bundling Deskulpt widgets and their external
//! dependencies.

use std::collections::HashMap;
use std::path::Path;

use anyhow::Error;
use swc_core::common::sync::Lrc;
use swc_core::common::{FilePathMapping, Globals, SourceMap, GLOBALS};
use swc_core::ecma::visit::{as_folder, FoldWith};

mod common;
mod transforms;

/// Bundle a widget into a single ESM string given its entry point.
///
/// The `dependency_map` argument is an optional mapping with keys being the
/// module specifiers to ignore. The import statements with these module
/// specifiers will be left as is in the bundled code without path resolution.
/// This should commonly be the list of external dependencies, since Deskulpt
/// requires widget developers to bundle their external dependencies (if any) to
/// be included directly in the Webview.
pub(crate) fn bundle(
    root: &Path,
    target: &Path,
    apis_blob_url: String,
    dependency_map: &HashMap<String, String>,
) -> Result<String, Error> {
    let globals = Globals::default();
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

    let module =
        common::bundle_into_raw_module(root, target, dependency_map, &globals, cm.clone())?;

    let code = GLOBALS.set(&globals, || {
        // Redirect widget APIs imports to the APIs blob URL
        let mut rename_apis = as_folder(transforms::ApisImportRenamer(apis_blob_url));
        let module = module.fold_with(&mut rename_apis);

        // Emit the bundled module as string into a buffer
        let mut buf = vec![];
        common::emit_module_to_buf(module, cm.clone(), &mut buf);
        String::from_utf8_lossy(&buf).to_string()
    });

    Ok(code)
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, read_to_string};
    use std::path::PathBuf;

    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use tempfile::{tempdir, TempDir};

    use super::*;
    use crate::testing::{assert_err_eq, ChainReason};

    /// Get the absolute path to the fixture directory.
    ///
    /// The paths used within the SWC bundler are all canonicalized (and thus
    /// verbatim with the `\\?\` prefix on Windows), so canonicalize here to
    /// match them. Note that this is not the case elsewhere in the
    /// codebase.
    fn fixture_dir() -> PathBuf {
        Path::new("tests/fixtures/bundler").canonicalize().unwrap()
    }

    /// Setup a temporary directory for testing.
    ///
    /// This would create a temporary directory and an `input` directory inside
    /// it.
    fn setup_temp_dir() -> TempDir {
        let temp_dir = tempdir().unwrap();
        create_dir(temp_dir.path().join("input")).unwrap();
        temp_dir
    }

    #[rstest]
    // Use correct JSX runtime for `jsx`, `jsxs`, and `Fragment`
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
        let case_dir = fixture_dir().join(case);
        let bundle_root = case_dir.join("input");
        let result = bundle(
            &bundle_root,
            &bundle_root.join(entry),
            "blob://dummy-url".to_string(),
            &Default::default(),
        )
        .expect("Expected bundling to succeed");

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
        let case_dir = fixture_dir().join(case);
        let bundle_root = case_dir.join("input");
        let error = bundle(
            &bundle_root,
            &bundle_root.join("index.jsx"),
            Default::default(),
            &Default::default(),
        )
        .expect_err("Expected bundling error");
        assert_err_eq(error, expected_error);
    }

    #[rstest]
    #[should_panic]
    fn test_bundle_import_meta_panic() {
        // Test that accessing `import.meta` is not supported
        let bundle_root = fixture_dir().join("import_meta/input");
        let _ = bundle(
            &bundle_root,
            &bundle_root.join("index.jsx"),
            Default::default(),
            &Default::default(),
        );
    }

    #[rstest]
    fn test_bundle_absolute_import_error() {
        // Test that an absolute import raises a proper error
        let temp_dir = setup_temp_dir();

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
        let error = bundle(
            &bundle_root,
            &index_path,
            Default::default(),
            &Default::default(),
        )
        .expect_err("Expected bundling error");
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
