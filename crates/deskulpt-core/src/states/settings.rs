//! State management for the settings.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::path::PathExt;
use crate::settings::{Settings, ShortcutKey, Theme, WidgetSettings};

/// Managed state for the settings.
struct SettingsState(RwLock<Settings>);

/// Extension trait for operations on the settings state.
pub trait SettingsStateExt<R: Runtime>: Manager<R> + PathExt<R> + GlobalShortcutExt<R> {
    /// Initialize state management for the settings.
    ///
    /// This will load the settings from the persistence directory and
    /// initialize the shortcuts. If any step fails, it will fall back to a
    /// state that preserves as much persisted data as possible.
    fn manage_settings(&self) {
        let mut settings = self
            .persist_dir()
            .and_then(Settings::load)
            .unwrap_or_else(|e| {
                eprintln!("Failed to load settings: {e}");
                Settings::default()
            });
        settings.init_shortcuts(self.global_shortcut());
        self.manage(SettingsState(RwLock::new(settings)));
    }

    /// Get an immutable reference to the settings.
    fn get_settings(&self) -> RwLockReadGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.read().unwrap()
    }

    /// Get a mutable reference to the settings.
    fn get_settings_mut(&self) -> RwLockWriteGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.write().unwrap()
    }

    /// Update the theme.
    fn update_settings_theme(&self, theme: Theme) {
        let mut settings = self.get_settings_mut();
        settings.theme = theme;
    }

    /// Update a shortcut.
    ///
    /// The side effect is that the involved shortcuts will be unregistered or
    /// registered. If the provided shortcut is `None`, it means removing that
    /// shortcut.
    fn update_settings_shortcut(&self, key: ShortcutKey, shortcut: Option<String>) -> Result<()> {
        let mut settings = self.get_settings_mut();
        settings.update_shortcut(self.global_shortcut(), key, shortcut)?;
        Ok(())
    }

    /// Update the settings of a widget.
    fn update_settings_widget(&self, id: String, update: WidgetSettings) {
        let mut settings = self.get_settings_mut();
        settings.widgets.insert(id, update);
    }
}

impl<R: Runtime> SettingsStateExt<R> for App<R> {}
impl<R: Runtime> SettingsStateExt<R> for AppHandle<R> {}
