//! Keyboard shortcut management.

use anyhow::{bail, Result};
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcut, GlobalShortcutExt, ShortcutState};

use crate::settings::{Settings, ShortcutKey};
use crate::states::StatesExtCanvasClickThrough;
use crate::WindowExt;

/// Trait that defines the handling of a keyboard shortcut.
trait ShortcutHandler<R: Runtime> {
    /// Register the keyboard shortcut.
    fn register(manager: &GlobalShortcut<R>, shortcut: &str) -> Result<()>;
}

/// Handler for [`ShortcutKey::ToggleCanvas`].
struct ToggleCanvasHandler;

impl<R: Runtime> ShortcutHandler<R> for ToggleCanvasHandler {
    fn register(manager: &GlobalShortcut<R>, shortcut: &str) -> Result<()> {
        manager.on_shortcut(shortcut, |app_handle, _, event| {
            if event.state == ShortcutState::Pressed {
                if let Err(e) = app_handle.toggle_canvas_click_through() {
                    eprintln!("Failed to toggle canvas click-through: {e}");
                }
            }
        })?;
        Ok(())
    }
}

/// Handler for [`ShortcutKey::OpenManager`].
struct OpenManagerHandler;

impl<R: Runtime> ShortcutHandler<R> for OpenManagerHandler {
    fn register(manager: &GlobalShortcut<R>, shortcut: &str) -> Result<()> {
        manager.on_shortcut(shortcut, |app_handle, _, _| {
            if let Err(e) = app_handle.open_manager() {
                eprintln!("Failed to open the manager window: {e}");
            }
        })?;
        Ok(())
    }
}

/// Extension trait for keyboard shortcuts.
pub trait ShortcutsExt<R: Runtime>: Manager<R> + GlobalShortcutExt<R> {
    /// Initialize keyboard shortcuts according to the initial settings.
    ///
    /// If any shortcut fails to be registered, the initial settings will be
    /// modified to remove that shortcut. This is to prevent the application
    /// from panicking only due to non-critical failures, and also sync this
    /// information to the frontend on startup.
    fn init_shortcuts(&self, settings: &mut Settings) {
        settings.shortcuts_mut().retain(|key, shortcut| {
            match self.update_shortcut(key, None, Some(shortcut)) {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("{:?}: {}", key, e);
                    false
                },
            }
        });
    }

    /// Update a shortcut registered in the application.
    ///
    /// This function will compare the old and new shortcuts and perform an
    /// update only if it has changed. In that case, the old shortcut (if
    /// exists) will be unregistered and the new shortcut (if exists) will be
    /// registered.
    fn update_shortcut(
        &self,
        key: &ShortcutKey,
        old_shortcut: Option<&str>,
        new_shortcut: Option<&str>,
    ) -> Result<()> {
        if old_shortcut == new_shortcut {
            return Ok(());
        }
        let manager = self.global_shortcut();

        if let Some(shortcut) = old_shortcut {
            if !manager.is_registered(shortcut) {
                bail!("Cannot unregister '{shortcut}': not registered yet");
            }
            manager.unregister(shortcut)?;
        }

        if let Some(shortcut) = new_shortcut {
            if manager.is_registered(shortcut) {
                bail!("Cannot register '{shortcut}': already registered");
            }
            match key {
                ShortcutKey::ToggleCanvas => ToggleCanvasHandler::register(manager, shortcut)?,
                ShortcutKey::OpenManager => OpenManagerHandler::register(manager, shortcut)?,
            }
        }

        Ok(())
    }
}

impl<R: Runtime> ShortcutsExt<R> for App<R> {}
impl<R: Runtime> ShortcutsExt<R> for AppHandle<R> {}
