//! Event system for IPC between Deskulpt frontend and backend.

use anyhow::Result;
use serde::Serialize;
use tauri::{App, AppHandle, Emitter, Runtime};

/// Payload of the `show-toast` event.
#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "content", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShowToastPayload {
    /// Show a [success](https://sonner.emilkowal.ski/toast#success) toast.
    Success(String),
    /// Show an [error](https://sonner.emilkowal.ski/toast#error) toast.
    Error(String),
}

/// Extension trait for event-related operations.
pub trait EventsExt<R: Runtime>: Emitter<R> {
    /// Emit the `show-toast` event to the canvas.
    fn emit_show_toast_to_canvas(&self, payload: ShowToastPayload) -> Result<()> {
        Ok(self.emit_to("canvas", "show-toast", payload)?)
    }

    /// Emit the `exit-app` event to the manager.
    fn emit_exit_app_to_manager(&self) -> Result<()> {
        Ok(self.emit_to("manager", "exit-app", ())?)
    }

    /// Emit the `render-widgets` event to the canvas.
    fn emit_render_widgets_to_canvas(&self, payload: serde_json::Value) -> Result<()> {
        Ok(self.emit_to("canvas", "render-widgets", payload)?)
    }
}

impl<R: Runtime> EventsExt<R> for AppHandle<R> {}
impl<R: Runtime> EventsExt<R> for App<R> {}
