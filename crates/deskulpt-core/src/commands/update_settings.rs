use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::SettingsUpdate;
use crate::states::StatesExtSettings;

/// Wrapper of [`update_settings`](StatesExtSettings::update_settings).
///
/// ### Errors
///
/// - Any error that occurs while updating the settings.
#[command]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    updates: Vec<SettingsUpdate>,
) -> CmdResult<()> {
    app_handle.update_settings(updates)?;
    Ok(())
}
