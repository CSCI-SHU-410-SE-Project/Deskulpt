//! Keyboard shortcut management.

use anyhow::{bail, Result};
use paste::paste;
use serde::Deserialize;
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::settings::Shortcuts;
use crate::states::StatesExtSettings;
use crate::WindowExt;

/// Implement [`ShortcutKey`] and [`ShortcutsExt`] for the given shortcuts.
///
/// This macro takes a list of `key => listener` pairs, where `key` corresponds
/// to the keys of [`Shortcuts`] and `listener` is the corresponding shortcut
/// handler callback.
macro_rules! impl_shortcuts {
    ($($key: ident => $listener: expr),* $(,)?) => {
        paste! {
            /// Keyboard shortcuts registered in the application.
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub enum ShortcutKey {
                $(
                    [<$key:upper:camel>], // lower_snake_case => UpperCamelCase
                )*
            }
        }

        /// Extension trait for keyboard shortcuts.
        pub trait ShortcutsExt<R: Runtime>: Manager<R> + GlobalShortcutExt<R> {
            /// Initialize keyboard shortcuts according to the initial settings.
            ///
            /// If any shortcut fails to be registered, the initial settings will be
            /// modified to remove that shortcut. This is to prevent the application
            /// from panicking only due to non-critical failures, and also sync this
            /// information to the frontend.
            fn init_shortcuts(&self, shortcuts: &mut Shortcuts) {
                paste! {
                    $(
                        if let Err(e) = self.update_shortcut(
                            ShortcutKey::[<$key:upper:camel>],
                            None,
                            shortcuts.$key.as_deref(),
                        ) {
                            eprintln!("{}: {}", stringify!($key), e);
                            shortcuts.$key = None;
                        }
                    )*
                }
            }

            /// Update a shortcut registered in the application.
            ///
            /// This function will compare the old and new shortcuts and perform an update
            /// only if it has changed. In that case, the old shortcut (if exists) will be
            /// unregistered and the new shortcut (if exists) will be registered.
            fn update_shortcut(
                &self,
                key: ShortcutKey,
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
                    paste! {
                        match key {
                            $(
                                ShortcutKey::[<$key:upper:camel>] => {
                                    manager.on_shortcut(shortcut, $listener)?;
                                },
                            )*
                        }
                    }
                }

                Ok(())
            }
        }
    };
}

impl_shortcuts! {
    toggle_canvas_imode => |app_handle, _, event| {
        if event.state == ShortcutState::Pressed {
            if let Err(e) = app_handle.toggle_canvas_imode() {
                eprintln!("Failed to toggle canvas interaction mode: {e}");
            }
        }
    },
    open_manager => |app_handle, _, _| {
        if let Err(e) = app_handle.open_manager() {
            eprintln!("Failed to open the manager window: {e}");
        }
    },
}

impl<R: Runtime> ShortcutsExt<R> for App<R> {}
impl<R: Runtime> ShortcutsExt<R> for AppHandle<R> {}
