#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use anyhow::Error;
use serde::Serialize;
use tauri::{App, AppHandle, Emitter, Runtime};

/// Payload of the "show-toast" event.
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShowToastPayload {
    /// Show a [success](https://sonner.emilkowal.ski/toast#success) toast.
    Success(String),
    /// Show an [error](https://sonner.emilkowal.ski/toast#error) toast.
    Error(String),
}

/// Extension trait for event-related operations in Deskulpt.
pub trait EventsExt<R: Runtime>: Emitter<R> {
    /// Emit a "show-toast" event to the canvas.
    fn emit_show_toast_to_canvas(&self, payload: ShowToastPayload) -> Result<(), Error> {
        self.emit_to("canvas", "show-toast", payload)
            .map_err(|e| e.into())
    }
}

impl<R: Runtime> EventsExt<R> for AppHandle<R> {}

impl<R: Runtime> EventsExt<R> for App<R> {}
