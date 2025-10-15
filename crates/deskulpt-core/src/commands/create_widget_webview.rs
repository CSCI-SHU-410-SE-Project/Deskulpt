use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::window::WindowExt;

#[command]
#[specta::specta]
pub async fn create_widget_webview<R: Runtime>(
    app_handle: AppHandle<R>,
    id: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> CmdResult<()> {
    app_handle.create_widget_webview(id, x, y, width, height)?;
    Ok(())
}
