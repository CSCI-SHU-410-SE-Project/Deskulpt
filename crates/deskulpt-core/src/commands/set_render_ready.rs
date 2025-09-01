use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::StatesExtInitialRender;

/// Wrapper of [`set_render_ready`](StatesExtInitialRender::set_render_ready).
///
/// ### Errors
///
/// - Failed to emit the `render-widgets` event to the canvas.
#[command]
#[specta::specta]
pub async fn set_render_ready<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    Ok(app_handle.set_render_ready()?)
}
