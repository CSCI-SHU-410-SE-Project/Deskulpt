//! Application and widget settings.

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// The settings file name in the persistence directory.
static SETTINGS_FILE: &str = "settings.bin";

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
#[derive(Deserialize, Serialize)]
struct WidgetSettings {
    /// The leftmost x-coordinate in pixels.
    x: i32,
    /// The topmost y-coordinate in pixels.
    y: i32,
    /// The opacity in percentage.
    opacity: i32,
}

/// Light/dark theme of the application.
#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Theme {
    #[default]
    Light,
    Dark,
}

/// Full settings of the application.
#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The application theme.
    theme: Theme,
    /// The keyboard shortcut for toggling canvas click-through.
    toggle_shortcut: Option<String>,
    /// The mapping from widget IDs to their respective settings.
    widget_settings_map: HashMap<String, WidgetSettings>,
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
        let settings: Settings = bincode::deserialize_from(reader)?;
        Ok(settings)
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
        bincode::serialize_into(writer, self)?;
        Ok(())
    }
}
