use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::SettingsUpdate;
use crate::states::StatesExtSettings;

#[command]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    updates: Vec<SettingsUpdate>,
) -> CmdResult<()> {
    app_handle.update_settings(updates)?;
    Ok(())
}
