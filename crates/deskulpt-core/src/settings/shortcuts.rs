//! Keyboard shortcut management.

use anyhow::Result;
use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcut, ShortcutState};

use super::{Settings, ShortcutKey};
use crate::states::CanvasImodeStateExt;
use crate::window::WindowExt;

impl Settings {
    pub fn init_shortcuts<R: Runtime>(&mut self, gs: &GlobalShortcut<R>) {
        for (key, current) in self.app.shortcuts.clone() {
            if let Err(e) = self.update_shortcut(gs, &key, Some(current)) {
                eprintln!("Failed to register shortcut for {key:?}: {e}");
                self.app.shortcuts.remove(&key);
            }
        }
    }

    pub fn update_shortcut<R: Runtime>(
        &mut self,
        gs: &GlobalShortcut<R>,
        key: &ShortcutKey,
        new: Option<String>,
    ) -> Result<()> {
        let old = self.app.shortcuts.get(key);
        if old == new.as_ref() {
            return Ok(());
        }
        if let Some(shortcut) = old {
            gs.unregister(shortcut.as_str())?;
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
            gs.on_shortcut(shortcut.as_str(), move |app_handle, _, event| {
                if event.state == ShortcutState::Pressed {
                    handler(app_handle);
                }
            })?;
        }

        Ok(())
    }
}
