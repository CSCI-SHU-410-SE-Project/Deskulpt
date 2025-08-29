use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::events::{DeskulptEvent, UpdateSettingsEvent};
use crate::settings::SettingsUpdate;
use crate::states::StatesExtSettings;

/// Update the settings.
///
/// See [`update_settings`](StatesExtSettings::update_settings) for details on
/// how the settings are updated. An [`UpdateSettingsEvent`] is emitted with the
/// updated settings, even if some or all updates fail.
///
/// ### Errors
///
/// - Error emitting the [`UpdateSettingsEvent`].
/// - Some or all updates failed.
#[command]
pub async fn update_settings<R: Runtime>(
    app_handle: AppHandle<R>,
    update: SettingsUpdate,
) -> CmdResult<()> {
    app_handle.update_settings(update)?;
    let settings = app_handle.get_readable_settings().clone();
    UpdateSettingsEvent(settings).emit(&app_handle)?;
    Ok(())
}
