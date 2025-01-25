use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::settings::Shortcuts;
use crate::shortcuts::ShortcutsExt;

/// Update the shortcuts registered in the application.
///
/// This command will compare the old and new shortcuts and update only the ones
/// that have changed. For each changed shortcut, the old one (if exists) will
/// be unregistered and the new one (if exists) will be registered.
///
/// ### Errors
///
/// - Some old shortcut that needs to be unregistered is not registered.
/// - Some new shortcut that needs to be registered is already registered.
/// - Error registering or unregistering shortcuts.
#[command]
pub async fn update_shortcuts<R: Runtime>(
    app_handle: AppHandle<R>,
    old_shortcuts: Shortcuts,
    new_shortcuts: Shortcuts,
) -> CmdResult<()> {
    app_handle.update_canvas_toggle_shortcut(
        old_shortcuts.canvas_toggle.as_deref(),
        new_shortcuts.canvas_toggle.as_deref(),
    )?;

    Ok(())
}
