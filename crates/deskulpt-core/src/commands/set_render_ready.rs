use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::StatesExtRenderReady;

/// Set the `render` listener as ready.
///
/// This is a wrapper command for
/// [`set_render_ready`](StatesExtRenderReady::set_render_ready) to be invoked
/// by the frontend.
///
/// ### Errors
///
/// - Failed to emit the `render` event to the canvas.
#[command]
pub async fn set_render_ready<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    Ok(app_handle.set_render_ready()?)
}
