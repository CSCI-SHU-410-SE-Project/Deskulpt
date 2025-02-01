use serde::Deserialize;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::shortcuts::ShortcutsExt;

/// The key of the shortcut to update.
///
/// This corresponds to `keyof Shortcuts` in TypeScript.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShortcutKey {
    ToggleCanvas,
}

/// Update a shortcut registered in the application.
///
/// This command will compare the old and new shortcuts and perform an update
/// only if it has changed. In that case, the old shortcut (if exists) will be
/// unregistered and the new shortcut (if exists) will be registered.
///
/// ### Errors
///
/// - The old shortcut needs to be unregistered but is not registered.
/// - The new shortcut needs to be registered but is already registered.
/// - Error registering or unregistering shortcuts.
#[command]
pub async fn update_shortcut<R: Runtime>(
    app_handle: AppHandle<R>,
    key: ShortcutKey,
    old_shortcut: Option<String>,
    new_shortcut: Option<String>,
) -> CmdResult<()> {
    match key {
        ShortcutKey::ToggleCanvas => {
            app_handle
                .update_toggle_canvas_shortcut(old_shortcut.as_deref(), new_shortcut.as_deref())?;
        },
    }

    Ok(())
}
