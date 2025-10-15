//! Common utilities for Deskulpt windows.

/// Deskulpt window enum.
#[derive(Clone, Debug, specta::Type)]
#[specta(rename_all = "camelCase")]
pub enum DeskulptWindow {
    /// The manager window.
    Manager,
    /// The canvas window.
    Canvas,
}

impl AsRef<str> for DeskulptWindow {
    fn as_ref(&self) -> &str {
        match self {
            DeskulptWindow::Manager => "manager",
            DeskulptWindow::Canvas => "canvas",
        }
    }
}

impl From<DeskulptWindow> for String {
    fn from(window: DeskulptWindow) -> Self {
        window.as_ref().to_owned()
    }
}

impl std::fmt::Display for DeskulptWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
