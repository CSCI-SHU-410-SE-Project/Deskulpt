//! This module implements custom AST transforms.

use swc_core::ecma::ast::ImportDecl;
use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

/// An AST transformer that redirects widget APIs imports to a blob URL.
pub struct ApisImportRenamer(
    /// The blob URL to redirect APIs imports to.
    pub String,
);

impl VisitMut for ApisImportRenamer {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        if n.src.value.as_str() == "@deskulpt-test/apis" {
            n.src = Box::new(self.0.clone().into());
        }
    }
}
