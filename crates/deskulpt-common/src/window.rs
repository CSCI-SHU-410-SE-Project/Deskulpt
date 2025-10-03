//! Common utilities for Deskulpt windows.

use anyhow::{anyhow, Result};
use tauri::{Manager, Runtime, WebviewWindow};

/// Deskulpt window enum.
#[derive(Clone, Debug, specta::Type)]
#[specta(rename_all = "camelCase")]
pub enum DeskulptWindow {
    /// The manager window.
    Manager,
    /// The canvas window.
    Canvas,
}

impl DeskulptWindow {
    /// Retrieve the webview window instance.
    pub fn webview_window<R, M>(&self, manager: &M) -> Result<WebviewWindow<R>>
    where
        R: Runtime,
        M: Manager<R> + ?Sized,
    {
        manager
            .get_webview_window(self.as_ref())
            .ok_or_else(|| anyhow!("Window not found: {self}"))
    }
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
