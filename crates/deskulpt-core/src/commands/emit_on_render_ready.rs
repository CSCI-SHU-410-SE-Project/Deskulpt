use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::events::RenderWidgetsEvent;
use crate::states::InitialRenderStateExt;

/// Wrapper of
/// [`emit_on_render_ready`](InitialRenderStateExt::emit_on_render_ready).
///
/// ### Errors
///
/// - Failed to emit the [`RenderWidgetsEvent`] to the canvas.
#[command]
#[specta::specta]
pub async fn emit_on_render_ready<R: Runtime>(
    app_handle: AppHandle<R>,
    event: RenderWidgetsEvent,
) -> CmdResult<()> {
    Ok(app_handle.emit_on_render_ready(event)?)
}
