use serde::Deserialize;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::StatesExtWindowReady;

/// Window enum for the [`window_ready`] command.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WindowEnum {
    Canvas,
    Manager,
}

/// Mark a window as ready.
///
/// When both the canvas and manager windows are ready, a `window-ready` event
/// will be emitted to all windows.
///
/// ### Errors
///
/// - Failed to emit the `window-ready` event.
#[command]
pub async fn window_ready<R: Runtime>(
    app_handle: AppHandle<R>,
    window: WindowEnum,
) -> CmdResult<()> {
    match window {
        WindowEnum::Canvas => app_handle.set_canvas_window_ready()?,
        WindowEnum::Manager => app_handle.set_manager_window_ready()?,
    }
    Ok(())
}
