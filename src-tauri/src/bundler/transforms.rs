//! This module implements custom AST transforms.

use super::EXTERNAL_BUNDLE;
use std::{collections::HashMap, path::Path};
use swc_atoms::Atom;
use swc_ecma_ast::{
    ExportNamedSpecifier, ExportSpecifier, ImportDecl, ImportNamedSpecifier,
    ImportSpecifier, ModuleDecl, ModuleExportName, ModuleItem,
};
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

/// An AST transformer that records information of external imports.
///
/// It does **not** actually transform the AST but instead inspects information from it.
pub(super) struct ExternalImportInspector<'a> {
    /// The external dependencies to be recorded and removed.
    pub(super) external_dependencies: &'a HashMap<String, String>,
    /// Collection of module items constructed from the external imports.
    ///
    /// This collection is meant to be modified in place.
    pub(super) imports: &'a mut Vec<ModuleItem>,
    /// Collection of export specifiers constructed from the external imports.
    ///
    /// In particular, each export specifier would be named as the local identifier of
    /// the corresponding import specifier. This collection is meant to be modified in
    /// place.
    pub(super) export_specifiers: &'a mut Vec<ExportSpecifier>,
}

impl VisitMut for ExternalImportInspector<'_> {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        let import_source = n.src.value.as_str();
        if self.external_dependencies.contains_key(import_source)
            && !import_source.starts_with("@deskulpt-test/")
        {
            // Each import declaration is recorded as is to the collection of imports
            self.imports.push(ModuleItem::ModuleDecl(ModuleDecl::Import(n.clone())));

            for specifier in &n.specifiers {
                let (span, local) = match specifier {
                    ImportSpecifier::Named(spec) => (spec.span, spec.local.clone()),
                    ImportSpecifier::Default(spec) => (spec.span, spec.local.clone()),
                    ImportSpecifier::Namespace(spec) => (spec.span, spec.local.clone()),
                };

                // Each specifier in the import declaration is recorded as a named
                // export specifier
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
/// pointing to the bundle of external dependencies [`EXTERNAL_BUNDLE`].
pub(super) struct ExternalImportRedirector<'a> {
    /// The root directory of the widget to bundle.
    pub(super) root: &'a Path,
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

            // Rpleace the import source and specifiers
            n.src = Box::new(self.root.join(EXTERNAL_BUNDLE).to_string_lossy().into());
            n.specifiers = new_specifiers;
        }
    }
}

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
