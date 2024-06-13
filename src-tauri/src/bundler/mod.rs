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

    let mut apis_renamer = as_folder(transforms::ApisImportRenamer { apis_blob_url });
    let module = common::bundle_into_raw_module(
        root,
        target,
        dependency_map,
        &globals,
        cm.clone(),
    )?;

    if dependency_map.is_empty() {
        // If there are no external dependencies, there is no need to resolve external
        // imports and bundle a second round
        let code = GLOBALS.set(&globals, || {
            let module = common::apply_common_transforms(module, cm.clone())
                .fold_with(&mut apis_renamer);

            // Directly emit the bundled code into the buffer and return as string
            let mut buf = vec![];
            common::emit_module_to_buf(module, cm.clone(), &mut buf);
            String::from_utf8_lossy(&buf).to_string()
        });
        return Ok(code);
    }

    // There are external dependencies so we need to first redirect external imports to
    // local then bundle again; for this purpose we prepare a temporary file to hold the
    // intermediate widget bundle without resolving external imports
    let temp_bundle_path = root.join(TEMP_WIDGET_BUNDLE);
    let mut temp_bundle_file = File::create(&temp_bundle_path)?;

    GLOBALS.set(&globals, || {
        let module = common::apply_common_transforms(module, cm.clone());

        // Redirect external imports to prepare for the second round of bundling
        let mut resolver = as_folder(transforms::ExternalImportRedirector {
            root,
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
        &globals,
        cm.clone(),
    )?;

    let bridge_path = root.join(EXTERNAL_BUNDLE_BRIDGE);
    let mut bridge_file = File::create(&bridge_path)?;
    GLOBALS.set(&globals, || {
        let module = common::apply_common_transforms(module, cm.clone());

        // Inspect the import statements of external dependencies and record them
        let mut external_imports: Vec<ModuleItem> = vec![];
        let mut export_specifiers: Vec<ExportSpecifier> = vec![];
        let mut inspector = as_folder(transforms::ExternalImportInspector {
            external_dependencies: dependency_map,
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
