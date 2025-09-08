//! Application and widget settings.

use std::collections::BTreeMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use deskulpt_macros::Persisted;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod shortcuts;

/// The settings file name in the persistence directory.
static SETTINGS_FILE: &str = "settings.json";

/// The URL to the JSON schema file of the settings.
static SETTINGS_SCHEMA_URL: &str = "https://csci-shu-410-se-project.github.io/settings-schema.json";

/// Light/dark theme of the application.
#[derive(Clone, Default, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, JsonSchema, specta::Type,
)]
#[serde(rename_all = "camelCase")]
pub enum ShortcutKey {
    /// For toggling canvas interaction mode.
    ToggleCanvasImode,
    /// For opening the manager window.
    OpenManager,
}

/// Application-wide settings.
#[derive(Clone, Default, Deserialize, Serialize, JsonSchema, specta::Type, Persisted)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// The application theme.
    pub theme: Theme,
    /// The keyboard shortcuts.
    pub shortcuts: BTreeMap<ShortcutKey, String>,
}

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
#[derive(Clone, Deserialize, Serialize, JsonSchema, specta::Type, Persisted)]
#[serde(rename_all = "camelCase")]
pub struct WidgetSettings {
    /// The leftmost x-coordinate in pixels.
    pub x: i32,
    /// The topmost y-coordinate in pixels.
    pub y: i32,
    /// The opacity in percentage.
    #[persisted(default = "default_opacity")]
    pub opacity: i32,
}

fn default_opacity() -> i32 {
    100
}

/// Full settings of the Deskulpt application.
#[derive(Clone, Default, Deserialize, Serialize, JsonSchema, specta::Type, Persisted)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Application-wide settings.
    #[persisted(type = "AppSettingsPersisted")]
    pub app: AppSettings,
    /// The mapping from widget IDs to their respective settings.
    #[persisted(type = "BTreeMap<String, WidgetSettingsPersisted>")]
    pub widgets: BTreeMap<String, WidgetSettings>,
}

/// Wrapper of [`Settings`] with additional metadata.
#[derive(Serialize)]
struct SettingsWithMeta<'a> {
    /// The JSON schema URL `$schema`.
    #[serde(rename = "$schema")]
    schema: &'static str,
    /// The settings.
    ///
    /// This field is borrowed because this struct is only for serialization
    /// purposes and does not need ownership so as to avoid unnecessary cloning.
    /// It is flattened in serialization.
    #[serde(flatten)]
    settings: &'a Settings,
}

impl<'a> SettingsWithMeta<'a> {
    /// Wrap the borrowed settings with metadata.
    fn new(settings: &'a Settings) -> Self {
        Self {
            schema: SETTINGS_SCHEMA_URL,
            settings,
        }
    }
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
        let settings = SettingsWithMeta::new(self);
        serde_json::to_writer_pretty(writer, &settings)?;
        Ok(())
    }
}
