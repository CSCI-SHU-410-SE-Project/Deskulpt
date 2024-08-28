//! The module implements the Deskulpt bundler based on SWC.
//!
//! Note that this is not a general-purpose bundler; it is specifically designed for
//! the use case of bundling Deskulpt widgets and their external dependencies.

use anyhow::Error;
use std::{collections::HashMap, path::Path};
use swc_common::{sync::Lrc, FilePathMapping, Globals, SourceMap, GLOBALS};
use swc_ecma_visit::{as_folder, FoldWith};

mod common;
mod transforms;

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
    apis_blob_url: String,
    dependency_map: &HashMap<String, String>,
) -> Result<String, Error> {
    let globals = Globals::default();
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

    let module = common::bundle_into_raw_module(
        root,
        target,
        dependency_map,
        &globals,
        cm.clone(),
    )?;

    let code = GLOBALS.set(&globals, || {
        let module = common::apply_basic_transforms(module, cm.clone());

        // We need to rename the imports of `@deskulpt-test/apis` to the blob URL which
        // wraps the widget APIs to avoid exposing the raw APIs that allow specifying
        // widget IDs; note that this transform should be done last to avoid messing up
        // with import resolution
        let mut wrap_apis = as_folder(transforms::ImportRenamer(
            [("@deskulpt-test/apis".to_string(), apis_blob_url)].into(),
        ));
        let module = module.fold_with(&mut wrap_apis);

        // Emit the bundled module as string into a buffer
        let mut buf = vec![];
        common::emit_module_to_buf(module, cm.clone(), &mut buf);
        String::from_utf8_lossy(&buf).to_string()
    });

    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{assert_err_eq, ChainReason};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::{
        fs::{create_dir, read_to_string},
        path::PathBuf,
    };
    use tempfile::{tempdir, TempDir};

    /// Get the absolute path to the fixture directory.
    ///
    /// The paths used within the SWC bundler are all canonicalized (and thus verbatim
    /// with the `\\?\` prefix on Windows), so canonicalize here to match them. Note
    /// that this is not the case elsewhere in the codebase.
    fn fixture_dir() -> PathBuf {
        Path::new("tests/fixtures/bundler").canonicalize().unwrap()
    }

    /// Setup a temporary directory for testing.
    ///
    /// This would create a temporary directory and an `input` directory inside it.
    fn setup_temp_dir() -> TempDir {
        let temp_dir = tempdir().unwrap();
        create_dir(temp_dir.path().join("input")).unwrap();
        temp_dir
    }

    #[rstest]
    // Use correct JSX runtime for `jsx`, `jsxs`, and `Fragment``
    #[case::jsx_runtime("jsx_runtime", "index.jsx")]
    // Correctly resolve JS/JSX imports with and without extensions, or as index files
    // of a directory
    #[case::import("import", "index.jsx")]
    // Correctly strip off TypeScript syntax
    #[case::strip_types("strip_types", "index.tsx")]
    // Replace `@deskulpt-test/apis` with the blob URL
    #[case::replace_apis("replace_apis", "index.js")]
    // Do not resolve imports from default and external dependencies
    #[case::external_deps("external_deps", "index.js")]
    fn test_bundle_ok(#[case] case: &str, #[case] entry: &str) {
        let case_dir = fixture_dir().join(case);
        let bundle_root = case_dir.join("input");
        let result = bundle(
            &bundle_root,
            &bundle_root.join(entry),
            "blob://dummy-url".to_string(),
            &HashMap::from([
                ("os-name".to_string(), "^6.0.0".to_string()),
                ("matcher".to_string(), "^5.0.0".to_string()),
            ]),
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
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve os-name from {}",
                fixture_dir().join("import_node_modules/input/index.jsx").display(),
            )),
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
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve https://cdn.jsdelivr.net/npm/os-name@6.0.0/+esm from {}",
                fixture_dir().join("import_url/input/index.jsx").display(),
            )),
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
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve ../../foo from {}",
                fixture_dir().join("import_beyond_root/input/index.jsx").display(),
            )),
            ChainReason::Exact(format!(
                "Relative imports should not go beyond the root '{}'",
                fixture_dir().join("import_beyond_root/input").display(),
            )),
        ]
    )]
    // Entry file does not exist
    #[case::entry_not_exist(
        "entry_not_exist",
        vec![
            ChainReason::Exact(format!(
                "Entry point does not exist: '{}'",
                fixture_dir().join("entry_not_exist/input/index.jsx").display()
            ))
        ]
    )]
    // Bad syntax that cannot be parsed
    #[case::bad_syntax(
        "bad_syntax",
        vec![
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("Bundler.load() failed".to_string()),
            ChainReason::Exact(format!(
                "Bundler.loader.load({}) failed",
                fixture_dir().join("bad_syntax/input/index.jsx")
                    .canonicalize()
                    .unwrap()
                    .display()
                )),
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
        let bundle_root = temp_dir.path().canonicalize().unwrap().join("input");
        let utils_path = bundle_root.join("utils.js");
        std::fs::write(&utils_path, "export const foo = 42;").unwrap();

        // Create an entry file that imports the absolute path of `utils.js`; note that
        // we use debugging format for the path because otherwise path separator "\" on
        // Windows will not be escaped
        let entry_path = bundle_root.join("index.jsx");
        println!("import {{ foo }} from {utils_path:?};");
        std::fs::write(&entry_path, format!("import {{ foo }} from {utils_path:?};"))
            .unwrap();

        let error =
            bundle(&bundle_root, &entry_path, Default::default(), &Default::default())
                .expect_err("Expected bundling error");
        let expected_error = vec![
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve {} from {}",
                utils_path.display(),
                entry_path.display()
            )),
            ChainReason::Exact(
                "Absolute imports are not supported; use relative imports instead"
                    .to_string(),
            ),
        ];
        assert_err_eq(error, expected_error);
    }
}
