//! Application and widget settings.

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// The settings file name in the persistence directory.
static SETTINGS_FILE: &str = "settings.json";

/// Light/dark theme of the application.
#[derive(Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "lowercase")]
#[ts(export_to = "types.ts")]
enum Theme {
    #[default]
    Light,
    Dark,
}

/// Keyboard shortcuts registered in the application.
///
/// A keyboard shortcut being `None` means that it is disabled, otherwise it is
/// a string parsable into [`Shortcut`](tauri_plugin_global_shortcut::Shortcut).
#[derive(Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "types.ts")]
pub struct Shortcuts {
    /// For toggling canvas interaction mode.
    #[serde(default)]
    pub toggle_canvas_imode: Option<String>,
    /// For opening the manager window.
    #[serde(default)]
    pub open_manager: Option<String>,
}

/// Application-wide settings.
#[derive(Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "types.ts")]
struct AppSettings {
    /// The application theme.
    #[serde(default)]
    theme: Theme,
    /// The keyboard shortcuts.
    #[serde(default)]
    shortcuts: Shortcuts,
}

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
#[derive(Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "types.ts")]
pub struct WidgetSettings {
    /// The leftmost x-coordinate in pixels.
    #[serde(default)]
    x: i32,
    /// The topmost y-coordinate in pixels.
    #[serde(default)]
    y: i32,
    /// The opacity in percentage.
    #[serde(default = "default_opacity")]
    opacity: i32,
}

fn default_opacity() -> i32 {
    100
}

/// Full settings of the application.
#[derive(Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Settings {
    /// Application-wide settings.
    #[serde(default)]
    app: AppSettings,
    /// The mapping from widget IDs to their respective settings.
    #[serde(default)]
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
        let settings: Settings = serde_json::from_reader(reader)?;
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
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    /// Get the mutable reference to the keyboard shortcuts.
    pub fn shortcuts_mut(&mut self) -> &mut Shortcuts {
        &mut self.app.shortcuts
    }
}
