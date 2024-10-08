//! This module implements custom AST transforms.

use swc_core::ecma::{
    ast::ImportDecl,
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

/// An AST transformer that redirects widget APIs imports to the specified blob URL.
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
    use super::*;
    use swc_core::ecma::{transforms::testing::test_inline, visit::as_folder};

    // Test that the `ApisImportRenamer` transformer correctly renames the imports of
    // `@deskulpt-test/apis` to the specified blob URL
    test_inline!(
        Default::default(),
        |_| as_folder(ApisImportRenamer("blob://dummy-url".into())),
        test_transform_apis_import_renamer,
        r#"import "@deskulpt-test/apis";"#,
        r#"import "blob://dummy-url";"#
    );
}
