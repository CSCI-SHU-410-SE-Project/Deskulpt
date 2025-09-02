//! Application and widget settings.

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::hash::Hash;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use deskulpt_macros::Persisted;
use serde::{Deserialize, Serialize};

/// Helper trait for converting a persisted type into its original type.
pub trait FromPersisted<T> {
    /// Convert a persisted value into its original type.
    fn from_persisted(value: T) -> Self;
}

impl<K, V, VP> FromPersisted<HashMap<K, VP>> for HashMap<K, V>
where
    V: FromPersisted<VP>,
    K: Hash + Eq,
{
    fn from_persisted(value: HashMap<K, VP>) -> Self {
        value
            .into_iter()
            .map(|(k, v)| (k, V::from_persisted(v)))
            .collect()
    }
}

/// The settings file name in the persistence directory.
static SETTINGS_FILE: &str = "settings.json";

/// Light/dark theme of the application.
#[derive(Clone, Default, Deserialize, Serialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// Keyboard shortcuts registered in the application.
///
/// A keyboard shortcut being `None` means that it is disabled, otherwise it is
/// a string parsable into [`Shortcut`](tauri_plugin_global_shortcut::Shortcut).
#[derive(Default, Deserialize, Serialize, specta::Type, Persisted)]
#[serde(rename_all = "camelCase")]
pub struct Shortcuts {
    /// For toggling canvas interaction mode.
    pub toggle_canvas_imode: Option<String>,
    /// For opening the manager window.
    pub open_manager: Option<String>,
}

/// Application-wide settings.
#[derive(Default, Deserialize, Serialize, specta::Type, Persisted)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// The application theme.
    theme: Theme,
    /// The keyboard shortcuts.
    #[persisted(type = "ShortcutsPersisted")]
    shortcuts: Shortcuts,
}

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
#[derive(Clone, Deserialize, Serialize, specta::Type, Persisted)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSettings {
    /// The leftmost x-coordinate in pixels.
    x: i32,
    /// The topmost y-coordinate in pixels.
    y: i32,
    /// The opacity in percentage.
    #[persisted(default = "default_opacity")]
    opacity: i32,
}

fn default_opacity() -> i32 {
    100
}

/// Full settings of the application.
#[derive(Default, Deserialize, Serialize, specta::Type, Persisted)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Application-wide settings.
    app: AppSettings,
    /// The mapping from widget IDs to their respective settings.
    #[persisted(type = "HashMap<String, WidgetSettingsPersisted>")]
    widgets: HashMap<String, WidgetSettings>,
}

impl Settings {
    /// Read the settings from the persistence directory.
    ///
    /// Default settings will be returned if the settings file does not exist.
    pub fn load<P: AsRef<Path>>(persist_dir: P) -> Result<Self> {
        let settings_path = persist_dir.as_ref().join(SETTINGS_FILE);
        if !settings_path.exists() {
            return Ok(Default::default());
        }
        let file = File::open(settings_path)?;
        let reader = BufReader::new(file);
        let settings: SettingsPersisted = serde_json::from_reader(reader)?;
        Ok(settings.into())
    }

    /// Write the settings to the persistence directory.
    pub fn dump<P: AsRef<Path>>(&self, persist_dir: P) -> Result<()> {
        // On certain platforms, File::create fails if intermediate directories
        // do not exist, in which case we need to manually create the directory;
        // see https://doc.rust-lang.org/std/fs/struct.File.html#method.create
        let persist_dir = persist_dir.as_ref();
        if !persist_dir.exists() {
            create_dir_all(persist_dir)?;
        }
        let file = File::create(persist_dir.join(SETTINGS_FILE))?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    /// Get the mutable reference to the keyboard shortcuts.
    pub fn shortcuts_mut(&mut self) -> &mut Shortcuts {
        &mut self.app.shortcuts
    }
}
