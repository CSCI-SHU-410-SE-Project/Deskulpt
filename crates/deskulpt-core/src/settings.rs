//! Deskulpt settings.

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// The settings file name in the persistence directory.
static SETTINGS_FILE: &str = "settings.json";

/// Light/dark theme of the application.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
// Use lowercase to align with Radix UI theme appearance
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "types.ts")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// Canvas interaction mode.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(export, export_to = "types.ts")]
pub enum CanvasImode {
    /// Sink mode.
    ///
    /// The canvas is click-through. Widgets are not interactable. The desktop
    /// is interactable.
    #[default]
    Sink,
    /// Float mode.
    ///
    /// The canvas is not click-through. Widgets are interactable. The desktop
    /// is not interactable.
    Float,
}

impl std::ops::Not for CanvasImode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            CanvasImode::Sink => CanvasImode::Float,
            CanvasImode::Float => CanvasImode::Sink,
        }
    }
}

/// Keyboard shortcuts registered in the application.
///
/// A keyboard shortcut being `None` means that it is disabled, otherwise it is
/// a string parsable into [`Shortcut`](tauri_plugin_global_shortcut::Shortcut).
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Shortcuts {
    /// For toggling canvas interaction mode.
    #[serde(default)]
    pub toggle_canvas_imode: Option<String>,
    /// For opening the manager window.
    #[serde(default)]
    pub open_manager: Option<String>,
}

/// An update to [`Shortcuts`].
#[derive(Clone, Default, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ShortcutsUpdate {
    /// An update to [`Shortcuts::toggle_canvas_imode`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub toggle_canvas_imode: Option<Option<String>>,
    /// An update to [`Shortcuts::open_manager`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub open_manager: Option<Option<String>>,
}

/// Application-wide settings.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AppSettings {
    /// The application theme.
    #[serde(default)]
    pub theme: Theme,
    /// Canvas interaction mode.
    #[serde(default)]
    pub canvas_imode: CanvasImode,
    /// The keyboard shortcuts.
    #[serde(default)]
    pub shortcuts: Shortcuts,
}

/// An update to [`AppSettings`].
#[derive(Clone, Default, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct AppSettingsUpdate {
    /// An update to [`AppSettings::theme`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub theme: Option<Theme>,
    /// An update to [`AppSettings::canvas_imode`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub canvas_imode: Option<CanvasImode>,
    /// An update to [`AppSettings::shortcuts`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub shortcuts: Option<ShortcutsUpdate>,
}

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct WidgetSettings {
    /// The leftmost x-coordinate in pixels.
    #[serde(default)]
    pub x: i32,
    /// The topmost y-coordinate in pixels.
    #[serde(default)]
    pub y: i32,
    /// The opacity in percentage.
    #[serde(default = "default_opacity")]
    pub opacity: i32,
}

fn default_opacity() -> i32 {
    100
}

/// An update to [`WidgetSettings`].
#[derive(Clone, Default, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct WidgetSettingsUpdate {
    /// An update to [`WidgetSettings::x`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub x: Option<i32>,
    /// An update to [`WidgetSettings::y`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub y: Option<i32>,
    /// An update to [`WidgetSettings::opacity`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub opacity: Option<i32>,
}

/// Full settings of the application.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Settings {
    /// Application-wide settings.
    #[serde(default)]
    pub app: AppSettings,
    /// The mapping from widget IDs to their respective settings.
    #[serde(default)]
    #[ts(type = "Record<string, WidgetSettings>")]
    pub widgets: HashMap<String, WidgetSettings>,
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
}

/// An update to [`Settings`].
#[derive(Clone, Default, Deserialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SettingsUpdate {
    /// An update to [`Settings::app`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub app: Option<AppSettingsUpdate>,
    /// An update to [`Settings::widgets`] by widget ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional, type = "Record<string, WidgetSettingsUpdate>")]
    pub widgets: Option<HashMap<String, WidgetSettingsUpdate>>,
}
