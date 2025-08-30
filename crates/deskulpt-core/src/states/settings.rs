//! State management for Deskulpt settings.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{bail, Result};
use tauri::{App, AppHandle, Manager, Runtime};

use crate::settings::{
    AppSettings, AppSettingsUpdate, SettingsUpdate, Shortcuts, ShortcutsUpdate, WidgetSettings,
    WidgetSettingsUpdate,
};
use crate::states::canvas_imode::StatesExtCanvasImode;
use crate::{PathExt, Settings, ShortcutsExt};

/// Trait for applying updates.
pub trait ApplyUpdate<U> {
    /// Apply an update to self.
    fn apply_update<R: Runtime>(
        &mut self,
        app_handle: &AppHandle<R>,
        settings: &Settings,
        u: U,
    ) -> Result<()>;
}

impl ApplyUpdate<ShortcutsUpdate> for Shortcuts {
    fn apply_update<R: Runtime>(
        &mut self,
        _: &AppHandle<R>,
        _: &Settings,
        u: ShortcutsUpdate,
    ) -> Result<()> {
        if let Some(toggle_canvas_imode) = u.toggle_canvas_imode {
            self.toggle_canvas_imode = toggle_canvas_imode;
        }
        if let Some(open_manager) = u.open_manager {
            self.open_manager = open_manager;
        }
        Ok(())
    }
}

impl ApplyUpdate<AppSettingsUpdate> for AppSettings {
    fn apply_update<R: Runtime>(
        &mut self,
        app_handle: &AppHandle<R>,
        settings: &Settings,
        u: AppSettingsUpdate,
    ) -> Result<()> {
        if let Some(theme) = u.theme {
            self.theme = theme;
        }
        if let Some(canvas_imode) = u.canvas_imode {
            app_handle.switch_canvas_imode(&canvas_imode)?;
            self.canvas_imode = canvas_imode;
        }
        if let Some(shortcuts) = u.shortcuts {
            app_handle.reregister_shortcuts(&settings.app.shortcuts, &shortcuts)?;
            self.shortcuts
                .apply_update(app_handle, settings, shortcuts)?;
        }
        Ok(())
    }
}

impl ApplyUpdate<WidgetSettingsUpdate> for WidgetSettings {
    fn apply_update<R: Runtime>(
        &mut self,
        _: &AppHandle<R>,
        _: &Settings,
        u: WidgetSettingsUpdate,
    ) -> Result<()> {
        if let Some(x) = u.x {
            self.x = x;
        }
        if let Some(y) = u.y {
            self.y = y;
        }
        if let Some(opacity) = u.opacity {
            if opacity < 0 || opacity > 100 {
                bail!("Opacity must be between 0 and 100; got {opacity}");
            }
            self.opacity = opacity;
        }
        Ok(())
    }
}

impl ApplyUpdate<SettingsUpdate> for Settings {
    fn apply_update<R: Runtime>(
        &mut self,
        app_handle: &AppHandle<R>,
        settings: &Settings,
        u: SettingsUpdate,
    ) -> Result<()> {
        if let Some(app) = u.app {
            self.app.apply_update(app_handle, settings, app)?;
        }
        if let Some(widgets) = u.widgets {
            for (key, value) in widgets {
                self.widgets
                    .entry(key)
                    .or_default()
                    .apply_update(app_handle, settings, value)?;
            }
        }
        Ok(())
    }
}

/// Managed state for Deskulpt settings.
struct SettingsState(RwLock<Settings>);

/// Extension trait for operations on Deskulpt settings state.
pub trait StatesExtSettings<R: Runtime>:
    Manager<R> + PathExt<R> + ShortcutsExt<R> + StatesExtCanvasImode<R>
{
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

        self.manage(SettingsState(RwLock::new(settings)));
    }

    /// Get a read-only reference to the settings.
    fn get_readable_settings(&self) -> RwLockReadGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.read().unwrap()
    }

    /// Get a read-and-write reference to the settings.
    fn get_writable_settings(&self) -> RwLockWriteGuard<'_, Settings> {
        let state = self.state::<SettingsState>().inner();
        state.0.write().unwrap()
    }

    /// Update the settings.
    ///
    /// This update is atomic, meaning that either all updates are succefully
    /// applied, or none are applied at all.
    ///
    /// For some settings, additional actions need be performed. These settings
    /// will not be updated unless their corresponding actions are successful.
    /// These include:
    ///
    /// - `app.canvas_imode`: Switch the canvas interaction mode.
    /// - `app.shortcuts`: Re-register the shortcut.
    fn update_settings(&self, update: SettingsUpdate) -> Result<()> {
        let settings_cloned = {
            let settings = self.get_readable_settings();
            let mut settings_cloned = self.get_readable_settings().clone();
            settings_cloned.apply_update(&self.app_handle(), &settings, update)?;
            settings_cloned
        };
        *self.get_writable_settings() = settings_cloned;
        Ok(())
    }
}

impl<R: Runtime> StatesExtSettings<R> for App<R> {}
impl<R: Runtime> StatesExtSettings<R> for AppHandle<R> {}
