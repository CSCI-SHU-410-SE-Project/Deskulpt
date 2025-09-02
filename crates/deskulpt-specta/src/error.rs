//! Errors in deskulpt-specta.

/// The error type for all errors in deskulpt-specta.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Handlebars render error: {0}")]
    HandlebarsRender(#[from] handlebars::RenderError),
    #[error("Handlebars template error: {0}")]
    HandlebarsTemplate(#[from] handlebars::TemplateError),
    #[error("Specta TypeScript export error: {0}")]
    SpectaExport(#[from] specta_typescript::ExportError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for deskulpt-specta.
pub type Result<T> = std::result::Result<T, Error>;
