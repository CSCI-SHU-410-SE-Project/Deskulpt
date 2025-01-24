//! Window initialization scripts.

use anyhow::Result;
use serialize_to_javascript::{default_template, DefaultTemplate, Template};

use crate::settings::Settings;

/// Template for window initialization scripts.
#[derive(Template)]
#[default_template("init.js")]
pub struct WindowInitJS {
    /// `window.__DESKULPT__.apisWrapper`
    apis_wrapper: &'static str,
    /// `window.__DESKULPT__.initialSettings`
    initial_settings: Settings,
}

impl WindowInitJS {
    /// Generate JavaScript code for initializing the window.
    pub fn generate(initial_settings: Settings) -> Result<String> {
        let template = Self {
            apis_wrapper: include_str!("../../generated/apis.wrapper.js"),
            initial_settings,
        };
        let serialized = template.render_default(&Default::default())?;
        Ok(serialized.into_string())
    }
}
