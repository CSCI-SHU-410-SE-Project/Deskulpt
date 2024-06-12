//! The module implements the Deskulpt bundler based on SWC.
//!
//! Note that this is not a general-purpose bundler; it is specifically designed for
//! the use case of bundling Deskulpt widgets and their external dependencies.

use crate::utils::run_shell_command;
use anyhow::{bail, Error};
use std::{
    collections::HashMap,
    fs::{remove_file, File},
    path::Path,
};
use swc_common::{sync::Lrc, FilePathMapping, Globals, SourceMap, DUMMY_SP, GLOBALS};
use swc_ecma_ast::{ExportSpecifier, Module, ModuleDecl, ModuleItem, NamedExport};
use swc_ecma_visit::{as_folder, FoldWith};
use tauri::{AppHandle, Runtime};

mod common;
mod path_loader;
mod path_resolver;
mod transforms;

/// The supported file extensions to import.
const EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];

/// Name of the bridge file for external dependencies.
///
/// The file is meant to hold the import statements of external dependencies and export
/// them as named exports. This can then be bundled with node modules resolution into
/// [`EXTERNAL_BUNDLE`].
const EXTERNAL_BUNDLE_BRIDGE: &str = ".deskulpt--external-imports.js";

/// Name of the bundle file of external dependencies.
const EXTERNAL_BUNDLE: &str = ".deskulpt--bundle.js";

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
        true, // Replace external imports
        &globals,
        cm.clone(),
    )?;

    let code = GLOBALS.set(&globals, || {
        let module = common::apply_common_transforms(module, cm.clone());

        // We need to rename the imports of `@deskulpt-test/apis` to the blob URL which
        // wraps the widget-specific APIs
        let mut wrap_apis = as_folder(transforms::ImportRenamer {
            rename_mapping: [("@deskulpt-test/apis".to_string(), apis_blob_url)].into(),
        });
        let module = module.fold_with(&mut wrap_apis);

        // Emit the bundled module as string into a buffer
        let mut buf = vec![];
        common::emit_module_to_buf(module, cm.clone(), &mut buf);
        String::from_utf8_lossy(&buf).to_string()
    });

    Ok(code)
}

/// Bundle the external dependencies of a widget.
///
/// This is not designed for bundling widget source code. Instead, through bundling the
/// widget source code it obtains information necessary to produce a tree-shaked bundle
/// of external dependencies. It produces a bridge module based on these information and
/// the actual bundling is done via [`rollup`](https://rollupjs.org/). It would thus
/// require proper setup of `node` and `npm` in the environment (i.e., the requirements
/// for widget developers).
pub(crate) fn bundle_external<R: Runtime>(
    app_handle: &AppHandle<R>,
    root: &Path,
    target: &Path,
    dependency_map: &HashMap<String, String>,
) -> Result<(), Error> {
    assert!(!dependency_map.is_empty());

    let globals = Globals::default();
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

    let module = common::bundle_into_raw_module(
        root,
        target,
        dependency_map,
        false, // Keep external imports
        &globals,
        cm.clone(),
    )?;

    let bridge_path = root.join(EXTERNAL_BUNDLE_BRIDGE);
    GLOBALS.set(&globals, || {
        let module = common::apply_common_transforms(module, cm.clone());

        // Inspect the import statements of external dependencies and record them
        let mut external_imports: Vec<ModuleItem> = vec![];
        let mut export_specifiers: Vec<ExportSpecifier> = vec![];
        let mut inspector = as_folder(transforms::ExternalImportInspector {
            external_dependencies: &dependency_map,
            imports: &mut external_imports,
            export_specifiers: &mut export_specifiers,
        });
        module.fold_with(&mut inspector);

        // Build the bridge module for bundling external dependencies; this module
        // will hold those original import statements and their corresponding named
        // export statements
        let export_decl =
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(NamedExport {
                span: DUMMY_SP,
                specifiers: export_specifiers,
                src: None,
                type_only: false,
                with: None,
            }));
        let bridge_module = Module {
            span: DUMMY_SP,
            body: external_imports
                .into_iter()
                .chain(std::iter::once(export_decl))
                .collect(),
            shebang: None,
        };

        // Write the bridge module
        let bridge_file = File::create(&bridge_path);
        if bridge_file.is_err() {
            return;
        }
        let mut bridge_file = bridge_file.unwrap(); // Safe because not error
        common::emit_module_to_buf(bridge_module, cm.clone(), &mut bridge_file);
    });

    // Install rollup dependencies
    let result = run_shell_command(
        app_handle,
        root,
        "npm install --save-dev rollup @rollup/plugin-commonjs @rollup/plugin-node-resolve @rollup/plugin-terser",
    );
    if !result.success {
        let _ = remove_file(&bridge_path);
        bail!("Failed to install rollup; make sure node and npm are properly set up");
    }

    // Bundle the external dependencies via rollup
    let result = run_shell_command(
        app_handle,
        root,
        &format!("npx rollup {EXTERNAL_BUNDLE_BRIDGE} --file {EXTERNAL_BUNDLE} --format esm --plugin commonjs --plugin node-resolve --plugin terser"),
    );
    if !result.success {
        let _ = remove_file(&bridge_path);
        bail!("Failed to execute rollup");
    }

    let _ = remove_file(&bridge_path);
    Ok(())
}
