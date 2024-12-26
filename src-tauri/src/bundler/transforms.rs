//! This module implements custom AST transforms.

use swc_core::ecma::ast::ImportDecl;
use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

/// An AST transformer that redirects `@deskulpt-test/*` imports.
pub(super) struct ImportRenamer {
    /// The base URL to resolve local path imports.
    pub(super) base_url: String,
    /// The blob URL to redirect `@deskulpt-test/apis` imports to.
    pub(super) apis_blob_url: String,
}

impl VisitMut for ImportRenamer {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        if n.src.value.as_str() == "@deskulpt-test/apis" {
            n.src = Box::new(self.apis_blob_url.clone().into());
        } else if n.src.value.as_str() == "@deskulpt-test/emotion/jsx-runtime" {
            n.src = Box::new(format!("{}/.scripts/jsx-runtime.js", self.base_url).into());
        } else if n.src.value.as_str() == "@deskulpt-test/raw-apis" {
            n.src = Box::new(format!("{}/.scripts/raw-apis.js", self.base_url).into());
        } else if n.src.value.as_str() == "@deskulpt-test/react" {
            n.src = Box::new(format!("{}/.scripts/react.js", self.base_url).into());
        } else if n.src.value.as_str() == "@deskulpt-test/ui" {
            n.src = Box::new(format!("{}/.scripts/ui.js", self.base_url).into());
        }
    }
}

#[cfg(test)]
mod tests {
    use swc_core::ecma::transforms::testing::test_inline;
    use swc_core::ecma::visit::visit_mut_pass;

    use super::*;

    // Test that the `ImportRenamer` transformer correctly renames the imports
    // of `@deskulpt-test/*`
    test_inline!(
        Default::default(),
        |_| visit_mut_pass(ImportRenamer {
            base_url: "http://tauri.localhost".into(),
            apis_blob_url: "blob://dummy-url".into()
        }),
        test_transform_apis_import_renamer,
        concat!(
            r#"import "@deskulpt-test/apis";"#,
            r#"import "@deskulpt-test/emotion/jsx-runtime";"#,
            r#"import "@deskulpt-test/raw-apis";"#,
            r#"import "@deskulpt-test/react";"#,
            r#"import "@deskulpt-test/ui";"#
        ),
        concat!(
            r#"import "blob://dummy-url";"#,
            r#"import "http://tauri.localhost/.scripts/jsx-runtime.js";"#,
            r#"import "http://tauri.localhost/.scripts/raw-apis.js";"#,
            r#"import "http://tauri.localhost/.scripts/react.js";"#,
            r#"import "http://tauri.localhost/.scripts/ui.js";"#
        )
    );
}
