use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::events::RenderWidgetsEvent;
use crate::states::InitialRenderStatesExt;

/// Wrapper of
/// [`emit_on_render_ready`](InitialRenderStatesExt::emit_on_render_ready).
///
/// ### Errors
///
/// - Failed to emit the `render-widgets` event to the canvas.
#[command]
pub async fn emit_on_render_ready<R: Runtime>(
    app_handle: AppHandle<R>,
    event: RenderWidgetsEvent,
) -> CmdResult<()> {
    Ok(app_handle.emit_on_render_ready(event)?)
}
