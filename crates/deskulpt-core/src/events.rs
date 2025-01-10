//! Event system for IPC between Deskulpt frontend and backend.

use anyhow::Result;
use serde::Serialize;
use tauri::{App, AppHandle, Emitter, Runtime};

/// Payload of the `show-toast` event.
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
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
        self.emit_to("canvas", "show-toast", payload)
            .map_err(Into::into)
    }

    /// Emit the `exit-app` event to the manager.
    fn emit_exit_app_to_manager(&self) -> Result<()> {
        self.emit_to("manager", "exit-app", ()).map_err(Into::into)
    }
}

impl<R: Runtime> EventsExt<R> for AppHandle<R> {}
impl<R: Runtime> EventsExt<R> for App<R> {}
