//! Helper functions for the Handlebars template engine.
//!
//! The helpers should be registered via [`Handlebars::register_helper`].

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

/// Indent text by a certain number of spaces.
///
/// If the text is multi-line, each line will be indented.
///
/// Usage example: `{{ indent block_of_text 4 }}` indents the `block_of_text`
/// template variable by 4 spaces.
pub fn indent(
    h: &Helper<'_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    let text = h.param(0).unwrap().value().as_str().unwrap_or("");
    let spaces = h
        .param(1)
        .map(|p| p.value().as_u64().unwrap_or(0))
        .unwrap_or(0);
    let pad = " ".repeat(spaces as usize);
    let indented = text
        .lines()
        .map(|line| format!("{pad}{line}"))
        .collect::<Vec<_>>()
        .join("\n");
    out.write(&indented)?;
    Ok(())
}
