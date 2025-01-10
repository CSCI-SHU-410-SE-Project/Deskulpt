use tauri::{command, AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use super::error::{cmdbail, CmdResult};
use crate::states::StatesExtCanvasClickThrough;

/// Update the shortcut for toggling canvas click-through.
///
/// If `old_shortcut` is given, it will be unregistered. If `new_shortcut` is
/// given, it will be registered. It is not an error if both are not given, in
/// which case this is a no-op.
///
/// ### Errors
///
/// - `old_shortcut` is given but not registered.
/// - `new_shortcut` is given but already registered.
/// - Error registering or unregistering shortcuts.
#[command]
pub async fn update_toggle_shortcut<R: Runtime>(
    app_handle: AppHandle<R>,
    old_shortcut: Option<String>,
    new_shortcut: Option<String>,
) -> CmdResult<()> {
    let manager = app_handle.global_shortcut();

    // Unregister the old shortcut if it exists
    if let Some(shortcut) = old_shortcut {
        let shortcut = shortcut.as_str();
        if !manager.is_registered(shortcut) {
            cmdbail!("Failed to unregister '{shortcut}' because it is not registered yet");
        }
        manager.unregister(shortcut)?;
    }

    // Register the new shortcut if it exists
    if let Some(shortcut) = new_shortcut {
        let shortcut = shortcut.as_str();
        if manager.is_registered(shortcut) {
            cmdbail!("Failed to register '{shortcut}' because it is already registered");
        }
        manager.on_shortcut(shortcut, |app_handle, _, event| {
            if event.state == ShortcutState::Pressed {
                // We must only react to press events, otherwise we would toggle
                // back on release
                if let Err(e) = app_handle.toggle_canvas_click_through() {
                    eprintln!("Failed to toggle canvas click-through: {e}");
                }
            }
        })?;
    }

    Ok(())
}
