//! The module implements the Deskulpt bundler based on SWC.
//!
//! Note that this is not a general-purpose bundler; it is specifically designed for
//! the use case of bundling Deskulpt widgets and their external dependencies.

use anyhow::{bail, Error};
use std::{
    collections::HashMap,
    fs::{remove_file, File},
    path::Path,
};
use swc_common::{sync::Lrc, FilePathMapping, Globals, SourceMap, GLOBALS};
use swc_ecma_visit::{as_folder, FoldWith};
use tauri::{AppHandle, Runtime};

mod common;
mod transforms;

/// Name of the bridge file for external dependencies.
///
/// The file is meant to hold the import statements of external dependencies and export
/// them as named exports. This can then be bundled with node modules resolution into
/// [`EXTERNAL_BUNDLE`].
const EXTERNAL_BUNDLE_BRIDGE: &str = "__external_bundle_bridge.js";

/// Name of the bundle of external dependencies.
const EXTERNAL_BUNDLE: &str = "__external_bundle.js";

/// Name of the bundle of widget source code without resolving external dependencies.
const TEMP_WIDGET_BUNDLE: &str = "__temp_widget_bundle.js";

/// Bundle a widget and return the bundled code as a string.
///
/// This does not bundle the external dependencies (if any) of the widget. Instead, it
/// assumes that the external dependencies bundle already exists at [`EXTERNAL_BUNDLE`]
/// and resolves the external imports to point to that bundle. The bundling of external
/// dependencies should be done via [`bundle_external`].
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

    let mut apis_renamer = as_folder(transforms::ApisImportRenamer(apis_blob_url));

    if dependency_map.is_empty() {
        // If there are no external dependencies, there is no need to resolve external
        // imports and bundle a second round
        let code = GLOBALS.set(&globals, || {
            let module = common::apply_basic_transforms(module, cm.clone())
                .fold_with(&mut apis_renamer);

            // Directly emit the bundled code into the buffer and return as string
            let mut buf = vec![];
            common::emit_module_to_buf(module, cm.clone(), &mut buf);
            String::from_utf8_lossy(&buf).to_string()
        });
        return Ok(code);
    }

    // There are external dependencies, but the external dependency bundle is not ready
    if !root.join(EXTERNAL_BUNDLE).exists() {
        bail!(
            "External dependencies required: {dependency_map:?}, but the bundle is not \
            found at {EXTERNAL_BUNDLE}; bundle the external dependencies first"
        );
    }

    // We need to first redirect external imports to EXTERNAL_BUNDLE then bundle again;
    // for this purpose we prepare a temporary file to hold the intermediate widget
    // bundle without resolving external imports
    let temp_bundle_path = root.join(TEMP_WIDGET_BUNDLE);
    let mut temp_bundle_file = File::create(&temp_bundle_path)?;

    GLOBALS.set(&globals, || {
        let module = common::apply_basic_transforms(module, cm.clone());

        // Redirect external imports to prepare for the second round of bundling
        let mut resolver = as_folder(transforms::ExternalImportRedirector {
            external_dependencies: dependency_map,
        });
        let module = module.fold_with(&mut resolver);

        // Emit the module to the temporary file
        common::emit_module_to_buf(module, cm.clone(), &mut temp_bundle_file);
    });

    // Bundle a second time to resolve the external imports, using the temporary bundle
    // of widget source code as the entry point
    let module = common::bundle_into_raw_module(
        root,
        &temp_bundle_path,
        dependency_map,
        &globals,
        cm.clone(),
    );

    // Remove the temporary bundle file; avoid bundling error from leaving it behind
    remove_file(&temp_bundle_path)?;
    let module = module?;

    let code = GLOBALS.set(&globals, || {
        // We no longer need to apply the common transforms as they are already applied
        // in the first round of bundling, and the bundle of external dependencies
        // should have already been an ESM module itself; it suffices to redirect the
        // imports of widget APIs
        let module = module.fold_with(&mut apis_renamer);

        // Emit the bundled code into the buffer and return as string
        let mut buf = vec![];
        common::emit_module_to_buf(module, cm.clone(), &mut buf);
        String::from_utf8_lossy(&buf).to_string()
    });

    Ok(code)
}

/// Bundle the external dependencies of a widget.
///
/// This should be done prior to bundling the widget source code for a widget that uses
/// external dependencies. It produces a tree-shaked bundle of external dependencies at
/// the designated location [`EXTERNAL_BUNDLE`]. This is done via creating the bridge
/// file [`bundle_external_bridge`] and then calling [`rollup`](https://rollupjs.org/).
/// It would thus require proper setup of `node` and `npm` in the environment.
pub(crate) async fn bundle_external<R: Runtime>(
    app_handle: &AppHandle<R>,
    root: &Path,
    target: &Path,
    dependency_map: &HashMap<String, String>,
) -> Result<(), Error> {
    assert!(!dependency_map.is_empty());

    {
        // Wrap within a scoped block to limit the lifetime of `cm` so it is dropped
        // before entering the async context; this is necessary because `cm` is not
        // `Send` and cannot be passed to the async context
        let globals = Globals::default();
        let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

        let module = common::bundle_into_raw_module(
            root,
            target,
            dependency_map,
            &globals,
            cm.clone(),
        )?;

        // Generate the bundle bridge of external dependencies
        let mut bridge_file = File::create(root.join(EXTERNAL_BUNDLE_BRIDGE))?;
        GLOBALS.set(&globals, || {
            let module = common::apply_basic_transforms(module, cm.clone());
            let bridge_module = transforms::build_bridge_module(module, dependency_map);
            common::emit_module_to_buf(bridge_module, cm.clone(), &mut bridge_file);
        });
    }

    // Resolve the bundle bridge of external dependencies
    transforms::resolve_bridge_module(app_handle, root).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        testing::{assert_err_eq, setup_mock_env, ChainReason},
        utils::run_shell_command,
    };
    use copy_dir::copy_dir;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::{fs::read_to_string, path::PathBuf};

    /// Get the absolute path to the fixture directory.
    ///
    /// The paths used within the SWC bundler are all canonicalized (and thus verbatim
    /// with the `\\?\` prefix on Windows), so canonicalize here to match them. Note
    /// that this is not the case elsewhere in the codebase.
    fn fixture_dir() -> PathBuf {
        Path::new("tests/fixtures/bundler").canonicalize().unwrap()
    }

    #[rstest]
    // Use correct JSX runtime for `jsx`, `jsxs`, and `Fragment`
    #[case::jsx_runtime("jsx_runtime", "index.jsx")]
    // Correctly resolve JS/JSX imports with and without extensions, or as index files
    // of a directory
    #[case::import("import", "index.jsx")]
    // Correctly strip off TypeScript syntax
    #[case::strip_types("strip_types", "index.tsx")]
    // Replace `@deskulpt-test/apis` with the blob URL
    #[case::replace_apis("replace_apis", "index.js")]
    // Do not resolve imports from default and external dependencies
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
            ChainReason::Regex("failed to resolve lodash from".to_string()),
            ChainReason::Exact(
                "node_modules imports should be explicitly included in package.json to \
                avoid being bundled at runtime; URL imports are not supported, one \
                should vendor its source to local and use a relative import instead"
                .to_string()
            ),
        ],
    )]
    // URL import
    #[case::import_url(
        "import_url",
        vec![
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Regex("failed to resolve https://dummy.js from".to_string()),
            ChainReason::Exact(
                "node_modules imports should be explicitly included in package.json to \
                avoid being bundled at runtime; URL imports are not supported, one \
                should vendor its source to local and use a relative import instead"
                .to_string()
            ),
        ],
    )]
    // Relative import that goes beyond the root
    #[case::import_beyond_root(
        "import_beyond_root",
        vec![
            ChainReason::Skip,
            ChainReason::Skip,
            ChainReason::Regex("failed to resolve ../../foo from".to_string()),
            ChainReason::Regex("Relative imports should not go beyond the root".to_string()),
        ],
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
        ],
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
    async fn test_bundle_external_dependencies() {
        // Test the bundling of widgets that use external dependencies
        let case_dir = fixture_dir().join("external_deps");
        let external_deps =
            HashMap::from([("internal-ip".to_string(), "8.0.0".to_string())]);

        // Set up test environment in a temporary directory to avoid polluting the
        // workspace with node modules, etc.
        let (temp_dir, app_handle) = setup_mock_env();
        let bundle_root = temp_dir.path().join("external_deps");
        copy_dir(case_dir.join("input"), &bundle_root).unwrap();
        app_handle.plugin(tauri_plugin_shell::init()).unwrap();
        run_shell_command(&app_handle, &bundle_root, "npm install").await;

        // Directly bundle the widget and check that we get the proper error
        let error = bundle(
            &bundle_root,
            &bundle_root.join("index.js"),
            Default::default(),
            &external_deps,
        )
        .expect_err("Expected error before external dependencies are not bundled");
        assert_err_eq(
            error,
            vec![ChainReason::Regex(regex::escape(&format!(
                "External dependencies required: {external_deps:?}"
            )))],
        );

        // Create the bundle of external dependencies
        bundle_external(
            &app_handle,
            &bundle_root,
            &bundle_root.join("index.js"),
            &external_deps,
        )
        .await
        .unwrap();

        // Check that the bundle is created and the bundle bridge is removed properly
        assert!(bundle_root.join(EXTERNAL_BUNDLE).exists());
        assert!(!bundle_root.join(EXTERNAL_BUNDLE_BRIDGE).exists());

        let result = read_to_string(bundle_root.join(EXTERNAL_BUNDLE)).unwrap();
        let expected = read_to_string(case_dir.join("output-external.js")).unwrap();
        assert_eq!(result.trim(), expected);

        // Bundle the widget again and check that it succeeds
        let result = bundle(
            &bundle_root,
            &bundle_root.join("index.js"),
            Default::default(),
            &external_deps,
        )
        .expect("Expected success after external dependencies are bundled");

        let expected = read_to_string(case_dir.join("output.js")).unwrap();
        assert_eq!(result, expected);
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
}
