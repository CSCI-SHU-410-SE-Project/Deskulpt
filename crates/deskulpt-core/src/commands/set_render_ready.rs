use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::StatesExtRenderReady;

/// Set the canvas as ready to render widgets.
///
/// ### Errors
///
/// - Failed to emit the render event to the canvas.
#[command]
pub async fn set_render_ready<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    Ok(app_handle.set_render_ready()?)
}
