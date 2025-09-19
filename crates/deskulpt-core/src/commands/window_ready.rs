use tauri::{command, AppHandle, Runtime, WebviewWindow};

use super::error::CmdResult;
use crate::window::DeskulptWindow;

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn window_ready<R: Runtime>(
    _app_handle: AppHandle<R>,
    window: WebviewWindow<R>,
) -> CmdResult<()> {
    if window.label() == DeskulptWindow::Canvas.label() {
        unimplemented!();
    }
    Ok(())
}
