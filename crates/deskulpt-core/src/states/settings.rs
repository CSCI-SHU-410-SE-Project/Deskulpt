use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

use crate::path::PathExt;
use crate::settings::{Settings, ShortcutKey, Theme, WidgetSettings};

struct SettingsState(RwLock<Settings>);

pub trait SettingsStateExt<R: Runtime>: Manager<R> + PathExt<R> + GlobalShortcutExt<R> {
    fn manage_settings(&self) {
        let settings = self
            .persist_dir()
            .and_then(Settings::load)
            .unwrap_or_else(|e| {
                eprintln!("Failed to load settings: {e}");
                Settings::default()
            });

        self.manage(SettingsState(RwLock::new(settings)));
    }

    fn get_settings(&self) -> RwLockReadGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.read().unwrap()
    }

    fn get_settings_mut(&self) -> RwLockWriteGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.write().unwrap()
    }

    fn update_settings_theme(&self, theme: Theme) {
        let mut settings = self.get_settings_mut();
        settings.app.theme = theme;
    }

    fn update_settings_shortcut(&self, key: ShortcutKey, shortcut: Option<String>) -> Result<()> {
        let mut settings = self.get_settings_mut();
        settings.update_shortcut(self.global_shortcut(), &key, shortcut)?;
        Ok(())
    }

    fn update_settings_widget(&self, id: String, update: WidgetSettings) {
        let mut settings = self.get_settings_mut();
        settings.widgets.insert(id, update);
    }
}

impl<R: Runtime> SettingsStateExt<R> for App<R> {}
impl<R: Runtime> SettingsStateExt<R> for AppHandle<R> {}
