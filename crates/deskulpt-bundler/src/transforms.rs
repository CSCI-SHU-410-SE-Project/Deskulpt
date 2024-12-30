//! This module implements custom AST transforms.

use swc_core::ecma::ast::ImportDecl;
use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

/// An AST transformer that redirects widget APIs imports to a blob URL.
pub(super) struct ApisImportRenamer(
    /// The blob URL to redirect APIs imports to.
    pub(super) String,
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

#[cfg(test)]
mod tests {
    use swc_core::ecma::transforms::testing::test_inline;
    use swc_core::ecma::visit::visit_mut_pass;

    use super::*;

    // Test that the `ApisImportRenamer` transformer correctly renames the imports
    // of `@deskulpt-test/apis` to the specified blob URL
    test_inline!(
        Default::default(),
        |_| visit_mut_pass(ApisImportRenamer("blob://dummy-url".into())),
        test_transform_apis_import_renamer,
        r#"import "@deskulpt-test/apis";"#,
        r#"import "blob://dummy-url";"#
    );
}
