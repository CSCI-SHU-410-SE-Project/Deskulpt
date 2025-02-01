//! Keyboard shortcut registration.

use anyhow::{bail, Result};
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::settings::Settings;
use crate::states::StatesExtCanvasClickThrough;

/// Implement a shortcut update function in [`ShortcutsExt`].
///
/// The first argument is the name of the function. The second argument is the
/// top-level docstring. The third argument is the listener to be registered for
/// the shortcut.
macro_rules! impl_update_shortcut {
    ($name: ident, $doc: expr, $listener: expr) => {
        #[doc = $doc]
        ///
        /// This will compare the old and new shortcut, and update only when they
        /// are different. For each changed shortcut, the old one (if exists) will
        /// be unregistered and the new one (if exists) will be registered.
        fn $name(&self, old_shortcut: Option<&str>, new_shortcut: Option<&str>) -> Result<()> {
            if old_shortcut == new_shortcut {
                return Ok(());
            }
            let manager = self.global_shortcut();

            if let Some(shortcut) = old_shortcut {
                if !manager.is_registered(shortcut) {
                    bail!("Failed to unregister '{shortcut}' because it is not registered yet");
                }
                manager.unregister(shortcut)?;
            }

            if let Some(shortcut) = new_shortcut {
                if manager.is_registered(shortcut) {
                    bail!("Failed to register '{shortcut}' because it is already registered");
                }
                manager.on_shortcut(shortcut, $listener)?;
            }

            Ok(())
        }
    };
}

/// Extension trait for keyboard shortcuts.
pub trait ShortcutsExt<R: Runtime>: Manager<R> + GlobalShortcutExt<R> {
    /// Initialize keyboard shortcuts according to the initial settings.
    ///
    /// If any shortcut fails to be registered, the initial settings will be
    /// modified to remove that shortcut. This is to prevent the application
    /// from panicking only due to non-critical failures, and also sync this
    /// information to the frontend.
    fn init_shortcuts(&self, settings: &mut Settings) {
        let shortcuts = settings.shortcuts_mut();

        if let Err(e) = self.update_toggle_canvas_shortcut(None, shortcuts.toggle_canvas.as_deref())
        {
            eprintln!("{e}");
            shortcuts.toggle_canvas = None;
        }
    }

    impl_update_shortcut!(
        update_toggle_canvas_shortcut,
        "Update the keyboard shortcut for toggling canvas click-through.",
        |app_handle, _, event| {
            if event.state == ShortcutState::Pressed {
                if let Err(e) = app_handle.toggle_canvas_click_through() {
                    eprintln!("Failed to toggle canvas click-through: {e}");
                }
            }
        }
    );
}

impl<R: Runtime> ShortcutsExt<R> for App<R> {}
impl<R: Runtime> ShortcutsExt<R> for AppHandle<R> {}
