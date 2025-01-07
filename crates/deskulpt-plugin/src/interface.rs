//! Interaction interface.

use std::path::PathBuf;

use tauri::{AppHandle, Manager};

/// The interface for interacting with the Deskulpt engine (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// This is a temporary implementation that directly uses the app handle of the
/// Deskulpt core because the plugin currently runs in the same process. The
/// final implementation should use IPC for communication, and this struct may
/// need to hold the IPC channel, etc.
pub struct EngineInterface {
    app_handle: AppHandle,
}

impl EngineInterface {
    /// Create a new engine interface instance.
    pub(crate) fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    /// Get the directory of a widget (🚧 TODO 🚧).
    ///
    /// ### 🚧 TODO 🚧
    ///
    /// This method is a temporary implementation. The final implementation
    /// should use IPC to communicate with the Deskulpt core to get the widget
    /// directory.
    pub fn widget_dir(&self, widget_id: &str) -> PathBuf {
        self.app_handle
            .path()
            .resource_dir()
            .unwrap()
            .join("widgets")
            .join(widget_id)
    }
}
