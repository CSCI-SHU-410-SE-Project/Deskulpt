//! State management for Deskulpt settings.

use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{bail, Result};
use tauri::{App, AppHandle, Manager, Runtime};

use crate::settings::{AppSettingsUpdate, ApplyUpdate, Settings, SettingsUpdate};
use crate::states::canvas_imode::StatesExtCanvasImode;
use crate::{PathExt, ShortcutsExt};

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

    fn update_settings<I>(&self, updates: I) -> Result<()>
    where
        Self: Sized,
        I: IntoIterator<Item = SettingsUpdate>,
    {
        let mut settings = self.get_writable_settings();
        let mut errors = Vec::new();
        for update in updates {
            if let Err(e) = update_settings_internal(self, &mut settings, update) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            let msg = errors
                .into_iter()
                .map(|e| format!("{e}"))
                .collect::<Vec<_>>()
                .join("\n");
            bail!("Error(s) updating settings:\n{msg}")
        }
    }
}

fn update_settings_internal<R, S>(
    state: &S,
    settings: &mut Settings,
    update: SettingsUpdate,
) -> Result<()>
where
    R: Runtime,
    S: StatesExtSettings<R>,
{
    match update {
        // Canvas interaction mode
        SettingsUpdate::App {
            value: AppSettingsUpdate::CanvasImode { ref value },
        } => {
            state.set_canvas_imode(value.clone())?;
        },
        // Keyboard shortcuts
        SettingsUpdate::App {
            value: AppSettingsUpdate::Shortcuts { ref value },
        } => {
            state.reregister_shortcut(&settings.app.shortcuts, value)?;
        },
        // Other settings does not involve additional state changes
        _ => {},
    }

    settings.apply_update(update)?;
    Ok(())
}

impl<R: Runtime> StatesExtSettings<R> for App<R> {}
impl<R: Runtime> StatesExtSettings<R> for AppHandle<R> {}
