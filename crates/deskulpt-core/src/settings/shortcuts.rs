//! Keyboard shortcut management.

use anyhow::Result;
use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcut, ShortcutState};

use super::{Settings, ShortcutKey};
use crate::states::CanvasImodeStateExt;
use crate::window::WindowExt;

fn reregister_shortcut<R: Runtime>(
    gs: &GlobalShortcut<R>,
    key: &ShortcutKey,
    old: Option<&str>,
    new: Option<&str>,
) -> Result<()> {
    if old == new {
        return Ok(());
    }

    if let Some(shortcut) = old {
        gs.unregister(shortcut)?;
    }

    let handler: fn(&AppHandle<R>) = match key {
        ShortcutKey::ToggleCanvasImode => |app_handle| {
            if let Err(e) = app_handle.toggle_canvas_imode() {
                eprintln!("Failed to toggle canvas interaction mode: {e}");
            }
        },
        ShortcutKey::OpenManager => |app_handle| {
            if let Err(e) = app_handle.open_manager() {
                eprintln!("Failed to open the manager window: {e}");
            }
        },
    };

    if let Some(shortcut) = new {
        gs.on_shortcut(shortcut, move |app_handle, _, event| {
            if event.state == ShortcutState::Pressed {
                handler(app_handle);
            }
        })?;
    }

    Ok(())
}

impl Settings {
    pub fn init_shortcuts<R: Runtime>(&mut self, gs: &GlobalShortcut<R>) {
        for (key, current) in self.app.shortcuts.clone() {
            if let Err(e) = reregister_shortcut(gs, &key, None, Some(&current)) {
                eprintln!("Failed to register shortcut for {key:?}: {e}");
                self.app.shortcuts.remove(&key);
            }
        }
    }

    pub fn update_shortcut<R: Runtime>(
        &mut self,
        gs: &GlobalShortcut<R>,
        key: ShortcutKey,
        new: Option<String>,
    ) -> Result<()> {
        reregister_shortcut(
            gs,
            &key,
            self.app.shortcuts.get(&key).map(|s| s.as_str()),
            new.as_deref(),
        )?;

        if let Some(shortcut) = new {
            self.app.shortcuts.insert(key, shortcut);
        } else {
            self.app.shortcuts.remove(&key);
        }

        Ok(())
    }
}
