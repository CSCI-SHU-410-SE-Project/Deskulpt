use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::shortcuts::{ShortcutKey, ShortcutsExt};

/// Wrapper of [`update_shortcut`](ShortcutsExt::update_shortcut).
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
    app_handle.update_shortcut(key, old_shortcut.as_deref(), new_shortcut.as_deref())?;
    Ok(())
}
