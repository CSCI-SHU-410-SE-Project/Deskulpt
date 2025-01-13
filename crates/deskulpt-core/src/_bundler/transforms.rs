//! This module implements custom AST transforms.

use swc_core::ecma::ast::ImportDecl;
use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

/// An AST transformer that redirects `@deskulpt-test/*` imports.
pub struct ImportRenamer {
    /// The base URL to resolve local path imports.
    pub base_url: String,
    /// The blob URL to redirect `@deskulpt-test/apis` imports to.
    pub apis_blob_url: String,
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
