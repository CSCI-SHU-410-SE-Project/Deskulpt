use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::ShortcutKey;
use crate::states::SettingsStateExt;

/// Wrapper of [`update_shortcut`](ShortcutsExt::update_shortcut).
///
/// ### Errors
///
/// - The old shortcut needs to be unregistered but is not registered.
/// - The new shortcut needs to be registered but is already registered.
/// - Error registering or unregistering shortcuts.
#[command]
#[specta::specta]
pub async fn update_shortcut<R: Runtime>(
    app_handle: AppHandle<R>,
    key: ShortcutKey,
    shortcut: Option<String>,
) -> CmdResult<()> {
    app_handle.update_settings_shortcut(key, shortcut)?;
    Ok(())
}
