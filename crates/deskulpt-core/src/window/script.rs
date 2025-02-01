//! Window initialization scripts.
use anyhow::Result;
use serialize_to_javascript::{default_template, DefaultTemplate, Template};

/// Template for the canvas window initialization script.
#[derive(Template)]
#[default_template("canvas.js")]
pub struct CanvasInitJS {
    /// `window.__DESKULPT_CANVAS_INTERNALS__.apisWrapper`
    apis_wrapper: &'static str,
}

impl CanvasInitJS {
    /// Generate JavaScript code for initializing the canvas window.
    pub fn generate() -> Result<String> {
        let template = Self {
            apis_wrapper: include_str!("../../generated/apis.wrapper.js"),
        };
        let serialized = template.render_default(&Default::default())?;
        Ok(serialized.into_string())
    }
}
