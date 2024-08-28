//! This module implements custom AST transforms.

use std::collections::HashMap;

use swc_atoms::Atom;
use swc_ecma_ast::ModuleDecl;
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

/// An AST transformer that renames import module specifiers.
///
/// This should be wrapped within [`as_folder`].
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
