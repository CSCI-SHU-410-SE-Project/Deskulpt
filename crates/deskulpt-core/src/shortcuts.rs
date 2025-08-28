//! Keyboard shortcut management.

use anyhow::Result;
use paste::paste;
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::settings::{SettingsUpdate, Shortcuts, ShortcutsUpdate};
use crate::states::StatesExtSettings;
use crate::WindowExt;

/// Implement [`ShortcutsExt`] for the given shortcuts.
///
/// This macro takes a list of `key => listener` pairs, where `key` corresponds
/// to the keys of [`Shortcuts`] and `listener` is the corresponding shortcut
/// handler callback.
macro_rules! impl_shortcuts {
    ($($key: ident => $listener: expr),* $(,)?) => {

/// Extension trait for keyboard shortcuts.
pub trait ShortcutsExt<R: Runtime>: Manager<R> + GlobalShortcutExt<R> {
    /// Initialize keyboard shortcuts.
    ///
    /// If any shortcut fails to be registered, the initial settings will be
    /// modified to remove that shortcut. This is to prevent the application
    /// from panicking only due to non-critical failures.
    fn init_shortcuts(&self, shortcuts: &mut Shortcuts) {
        let default_shortcuts = Shortcuts::default();
        paste! {
            $(
                if let Err(e) = self.reregister_shortcut(
                    &default_shortcuts,
                    &ShortcutsUpdate::[<$key:upper:camel>](shortcuts.$key.clone()),
                ) {
                    eprintln!("{}: {}", stringify!($key), e);
                    shortcuts.$key = None;
                }
            )*
        }
    }

    /// Re-register keyboard shortcuts.
    ///
    /// The new shortcut will be registered according to the provided update. If
    /// the corresponding key already has a registered shortcut, it will be
    /// unregistered first. If the new shortcut and the original shortcut are
    /// the same, this method is a no-op.
    fn reregister_shortcut(
        &self,
        shortcuts: &Shortcuts,
        update: &ShortcutsUpdate
    ) -> Result<()> {
        let manager = self.global_shortcut();

        paste! {
            match update {
                $(
                    ShortcutsUpdate::[<$key:upper:camel>](value) => {
                        if (*value == shortcuts.$key) {
                            return Ok(());
                        }
                        if let Some(old) = &shortcuts.$key {
                            manager.unregister(old.as_ref())?;
                        }
                        if let Some(new) = value {
                            manager.on_shortcut(new.as_ref(), |app_handle, _, event| {
                                if event.state == ShortcutState::Pressed {
                                    ($listener)(app_handle);
                                }
                            })?;
                        }
                    },
                )*
            }
        }

        Ok(())
    }
}

    };
}

impl_shortcuts! {
    toggle_canvas_imode => |app_handle: &AppHandle<R>| {
        let current = app_handle.get_readable_settings().app.canvas_imode.clone();
        if let Err(e) = app_handle.update_settings([
            SettingsUpdate::canvas_imode(!current)
        ]) {
            eprintln!("Failed to toggle canvas interaction mode: {e}");
        }
    },
    open_manager => |app_handle: &AppHandle<R>| {
        if let Err(e) = app_handle.open_manager() {
            eprintln!("Failed to open the manager window: {e}");
        }
    },
}

impl<R: Runtime> ShortcutsExt<R> for App<R> {}
impl<R: Runtime> ShortcutsExt<R> for AppHandle<R> {}
