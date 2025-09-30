#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/src/public/deskulpt.svg"
)]

mod error;
mod helpers;
mod template;

use std::path::Path;

use handlebars::Handlebars;
use regex::Regex;
use tauri_specta::{ExportContext, LanguageExt};

/// Exporter for Deskulpt TypeScript bindings.
#[derive(Default)]
pub struct TypeScript(specta_typescript::Typescript);

impl LanguageExt for TypeScript {
    type Error = error::Error;

    fn render(&self, cfg: &ExportContext) -> Result<String, Self::Error> {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);
        handlebars.register_helper("indent", Box::new(helpers::indent));
        handlebars.register_template_string("bindings", include_str!("bindings.ts.hbs"))?;

        let data = template::BindingsTemplate::from(&self.0, cfg)?;
        let rendered = handlebars.render("bindings", &data)?;

        // TODO: Remove when specta > 2.0.0-rc.22
        let re =
            Regex::new(r"Partial\s*<\s*(\{\s*\[\s*key\s+in\s+string\s*\][^}]*\})\s*>").unwrap();
        let rendered = re.replace_all(&rendered, "$1").to_string();
        Ok(rendered)
    }

    fn format(&self, path: &Path) -> Result<(), Self::Error> {
        if let Some(formatter) = self.0.formatter {
            formatter(path)?;
        }
        Ok(())
    }
}
