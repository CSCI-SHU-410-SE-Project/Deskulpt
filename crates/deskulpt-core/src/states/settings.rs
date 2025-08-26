//! State management for Deskulpt settings.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::Result;
use tauri::menu::MenuItem;
use tauri::{App, AppHandle, Manager, Runtime};

use crate::settings::{Settings, SettingsUpdate};
use crate::{PathExt, ShortcutsExt};

/// Managed state for Deskulpt settings.
struct SettingsState {
    inner: RwLock<Settings>,
}

/// Extension trait for operations on Deskulpt settings state.
pub trait StatesExtSettings<R: Runtime>: Manager<R> + PathExt<R> + ShortcutsExt<R> {
    /// Initialize state management for the settings state.
    ///
    /// The persisted settings are loaded. Shortcuts are initialized because
    /// they may partially override initial settings. If any error has occurred,
    /// default settings are applied.
    fn manage_settings(&self) {
        let mut settings = self
            .persist_dir()
            .and_then(Settings::load)
            .unwrap_or_else(|e| {
                eprintln!("Failed to load settings: {e}");
                Settings::default()
            });

        self.init_shortcuts(&mut settings.app.shortcuts);

        self.manage(SettingsState {
            inner: RwLock::new(settings),
        });
    }

    /// Get a read-only reference to the settings.
    fn get_readable_settings(&self) -> RwLockReadGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.inner.read().unwrap()
    }

    /// Get a read-and-write reference to the settings.
    fn get_writable_settings(&self) -> RwLockWriteGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.inner.write().unwrap()
    }

    fn update_settings(&self, updates: Vec<SettingsUpdate>) -> Result<()> {
        self.get_writable_settings().update(updates)
    }

    fn toggle_canvas_imode(&self) -> Result<()> {
        // TODO(Charlie-XIAO): Remove
        Ok(())
    }

    fn set_canvas_imode_menu_item(&self, _: &MenuItem<R>) {
        // TODO(Charlie-XIAO): Remove
    }
}

impl<R: Runtime> StatesExtSettings<R> for App<R> {}
impl<R: Runtime> StatesExtSettings<R> for AppHandle<R> {}
