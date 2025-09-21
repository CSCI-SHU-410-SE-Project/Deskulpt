use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::commands::load_widgets;
use crate::states::{SetupStateExt, SetupTask};

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn mark_setup<R: Runtime>(app_handle: AppHandle<R>, task: SetupTask) -> CmdResult<()> {
    if let Some(true) = app_handle.mark_setup(task) {
        load_widgets(app_handle.clone()).await?;
    }
    Ok(())
}
