use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::StatesExtInitialRender;

/// Emit a `render-widgets` event to the canvas when it is ready.
///
/// ### Errors
///
/// - Failed to emit the `render-widgets` event to the canvas.
#[command]
pub async fn emit_on_render_ready<R: Runtime>(
    app_handle: AppHandle<R>,
    payload: serde_json::Value,
) -> CmdResult<()> {
    Ok(app_handle.emit_on_render_ready(payload)?)
}
