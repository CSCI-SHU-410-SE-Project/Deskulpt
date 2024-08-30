//! This module implements custom AST transforms.

use std::collections::HashMap;

use swc_core::{
    atoms::Atom,
    ecma::{
        ast::ModuleDecl,
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

/// An AST transformer that renames import module specifiers.
pub(super) struct ImportRenamer(pub(super) HashMap<String, String>);

impl VisitMut for ImportRenamer {
    noop_visit_mut_type!();

    fn visit_mut_module_decl(&mut self, n: &mut ModuleDecl) {
        n.visit_mut_children_with(self);

        if let ModuleDecl::Import(import_decl) = n {
            let src = import_decl.src.value.to_string();
            if let Some(new_src) = self.0.get(&src) {
                import_decl.src.value = Atom::from(new_src.clone());
            }
        }
    }
}
