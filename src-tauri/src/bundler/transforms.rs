//! This module implements AST transformers and utilities.

use super::{EXTERNAL_BUNDLE, EXTERNAL_BUNDLE_BRIDGE};
use crate::utils::run_shell_command;
use anyhow::{bail, Error};
use std::{borrow::Cow, collections::HashMap, fs::remove_file, path::Path};
use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
    ExportNamedSpecifier, ExportSpecifier, ImportDecl, ImportNamedSpecifier,
    ImportSpecifier, Module, ModuleDecl, ModuleExportName, ModuleItem, NamedExport,
};
use swc_ecma_visit::{
    as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith,
};
use tauri::{AppHandle, Runtime};

/// An AST transformer that resolves imports of widget APIs.
///
/// It replaces the imports from `@deskulpt-test/apis` with the given blob URL.
pub(super) struct ApisImportRenamer(
    /// The APIs blob URL to be used as the import source.
    pub(super) String,
);

impl VisitMut for ApisImportRenamer {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        if n.src.value.as_str() == "@deskulpt-test/apis" {
            n.src.value = Atom::from(self.0.clone());
        }
    }
}

/// An AST transformer that records information of external imports.
///
/// It does **not** actually transform the AST but instead inspects information from it.
/// The information it collects is useful when we construct the external dependencies
/// bridge.
struct ExternalImportInspector<'a> {
    /// The external dependencies to inspect.
    external_dependencies: &'a HashMap<String, String>,
    /// Collection of module items constructed from the external imports.
    ///
    /// Items will be added in place. In particular, each import declaration will be
    /// wrapped into a module item and added to the collection. This will help construct
    /// the import part of the external dependencies bridge.
    imports: &'a mut Vec<ModuleItem>,
    /// Collection of export specifiers constructed from the external imports.
    ///
    /// Items will be added in place. In particular, each specifier in an import
    /// declaration, whether named, default, or namespace, will be converted into a
    /// named export specifier and added to the collection.
    ///
    /// ```typescript
    /// import { foo1, a as foo2 } from "bar";  // -> foo1, foo2
    /// import foo3 from "bar";                 // -> foo3
    /// import * as foo4 from "bar";            // -> foo4
    /// ```
    ///
    /// In other words, this does not care about where they are imported from, but only
    /// what they are imported as.  This will help construct the export part of the
    /// external dependencies bridge.
    export_specifiers: &'a mut Vec<ExportSpecifier>,
}

impl VisitMut for ExternalImportInspector<'_> {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        let import_source = n.src.value.as_str();
        if self.external_dependencies.contains_key(import_source)
            && !import_source.starts_with("@deskulpt-test/")
        {
            // Wrap each import declaration as is
            self.imports.push(ModuleItem::ModuleDecl(ModuleDecl::Import(n.clone())));

            for specifier in &n.specifiers {
                let (span, local) = match specifier {
                    ImportSpecifier::Named(spec) => (spec.span, spec.local.clone()),
                    ImportSpecifier::Default(spec) => (spec.span, spec.local.clone()),
                    ImportSpecifier::Namespace(spec) => (spec.span, spec.local.clone()),
                };

                // All specifiers are converted to named export specifiers by their
                // local names
                self.export_specifiers.push(ExportSpecifier::Named(
                    ExportNamedSpecifier {
                        span,
                        orig: ModuleExportName::Ident(local.clone()),
                        exported: None,
                        is_type_only: false,
                    },
                ));
            }
        }
    }
}

/// An AST transformer that redirects imports of external dependencies.
///
/// It replaces the import statements of external dependencies with named imports
/// pointing to the bundle of external dependencies [`EXTERNAL_BUNDLE`]. For instance,
///
/// ```typescript
/// import { foo1, a as foo2 } from "bar";
/// import foo3 from "bar";
/// import * as foo4 from "bar";
/// ```
///
/// will be transformed into
///
/// ```typescript
/// import { foo1, foo2, foo3, foo4 } from "./${EXTERNAL_BUNDLE}";
/// ```
pub(super) struct ExternalImportRedirector<'a> {
    /// The external dependencies of the widget.
    pub(super) external_dependencies: &'a HashMap<String, String>,
}

impl VisitMut for ExternalImportRedirector<'_> {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);
        let import_source = n.src.value.as_str();

        // Import source is an external dependency
        if self.external_dependencies.contains_key(import_source)
            && !import_source.starts_with("@deskulpt-test/")
        {
            let mut new_specifiers: Vec<ImportSpecifier> = vec![];

            for specifier in &n.specifiers {
                let (span, local) = match specifier {
                    ImportSpecifier::Named(spec) => (spec.span, spec.local.clone()),
                    ImportSpecifier::Default(spec) => (spec.span, spec.local.clone()),
                    ImportSpecifier::Namespace(spec) => (spec.span, spec.local.clone()),
                };

                // Each specifier in the import declaration is replaced by a named
                // import specifier pointing to the external bundle
                new_specifiers.push(ImportSpecifier::Named(ImportNamedSpecifier {
                    span,
                    local,
                    imported: None,
                    is_type_only: false,
                }));
            }

            // Replace the import source and specifiers; this transformer will be used
            // to produce a temporary bundle of widget source code which will be placed
            // at the root, so it is safe to use a relative path
            n.src = Box::new(format!("./{EXTERNAL_BUNDLE}").into());
            n.specifiers = new_specifiers;
        }
    }
}

/// Utility for generating a bridge module for bundling external dependencies.
///
/// This function makes use of the [`ExternalImportInspector`] transform and creates a
/// bridge module that imports all external dependencies and exports them as named
/// exports. Hence once resolved via node modules resolution, the bridge module will
/// contain the source code of all external dependencies and export according to how
/// they appear locally in the widget source code.
pub(super) fn build_bridge_module(
    module: Module,
    dependency_map: &HashMap<String, String>,
) -> Module {
    let mut external_imports: Vec<ModuleItem> = vec![];
    let mut export_specifiers: Vec<ExportSpecifier> = vec![];

    // Inspect the module to collect information of external imports
    let mut inspector = as_folder(ExternalImportInspector {
        external_dependencies: dependency_map,
        imports: &mut external_imports,
        export_specifiers: &mut export_specifiers,
    });
    module.fold_with(&mut inspector);

    // The import declarations are already stored in `external_imports` as module items;
    // we further construct the exporting part of the bridge module into a module item
    // that contains the named export specifiers
    let export_decl = ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(NamedExport {
        span: DUMMY_SP,
        specifiers: export_specifiers,
        src: None,
        type_only: false,
        with: None,
    }));

    // Assemble the import and export parts into a module
    Module {
        span: DUMMY_SP,
        body: external_imports
            .into_iter()
            .chain(std::iter::once(export_decl))
            .collect(),
        shebang: None,
    }
}

/// Utility for resolving a bridge module via rollup.
///
/// This function makes use of the [`ExternalImportInspector`] transform and creates a
/// bridge module that imports all external dependencies and exports them as named
/// exports. Hence once resolved via node modules resolution, the bridge module will
/// contain the source code of all external dependencies and export according to how
/// they appear locally in the widget source code.
pub(super) async fn resolve_bridge_module<R: Runtime>(
    app_handle: &AppHandle<R>,
    root: &Path,
) -> Result<(), Error> {
    let command = format!(
        concat!(
            // Install rollup and necessary plugins
            "npm install --save-dev",
            " rollup",
            " @rollup/plugin-alias",
            " @rollup/plugin-replace",
            " @rollup/plugin-commonjs",
            " @rollup/plugin-node-resolve",
            " @rollup/plugin-terser",
            " && ",
            // Run rollup to convert the bridge module to the external bundle
            "npx rollup {}",
            " --file {}",
            " --format esm",
            // Externalize `@deskulpt-test/react` because rollup cannot resolve it and
            // it should be resolved during our runtime
            " --external @deskulpt-test/react",
            // Alias `react` to `@deskulpt-test/react` so we avoid duplicating React
            // source code in the bundle which is already available in the runtime
            " --plugin {}",
            // Replace `process.env.NODE_ENV` with `production` because `process` is not
            // available without a node environment, and we are sure to eliminate any
            // non-production code paths
            " --plugin {}",
            // Convert CommonJS external dependencies to ESM
            " --plugin commonjs",
            // Node modules resolution
            " --plugin node-resolve",
            // Minify the bundle
            " --plugin terser",
        ),
        EXTERNAL_BUNDLE_BRIDGE,
        EXTERNAL_BUNDLE,
        shell_escape::escape(Cow::Borrowed("alias={entries:{react:'@deskulpt-test/react'}}")),
        shell_escape::escape(Cow::Borrowed("replace={'process.env.NODE_ENV':JSON.stringify('production'),preventAssignment:true}"))
    );

    let bridge_path = root.join(EXTERNAL_BUNDLE_BRIDGE);
    let result = run_shell_command(app_handle, root, &command).await;
    if !result.success {
        let _ = remove_file(&bridge_path);
        bail!("Failed to install or execute rollup\n\n{}", result.stderr);
    }
    let _ = remove_file(&bridge_path);
    Ok(())
}
