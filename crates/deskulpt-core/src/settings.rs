//! Deskulpt settings.

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// The settings file name in the persistence directory.
static SETTINGS_FILE: &str = "settings.json";

trait Apply<U> {
    fn apply(&mut self, u: U) -> Result<()>;
}

/// Light/dark theme of the application.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// Canvas interaction mode.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

/// Keyboard shortcuts registered in the application.
///
/// A keyboard shortcut being `None` means that it is disabled, otherwise it is
/// a string parsable into [`Shortcut`](tauri_plugin_global_shortcut::Shortcut).
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
pub struct Shortcuts {
    /// For toggling canvas interaction mode.
    #[serde(default)]
    pub toggle_canvas_imode: Option<String>,
    /// For opening the manager window.
    #[serde(default)]
    pub open_manager: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "field", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShortcutsUpdate {
    ToggleCanvasImode { value: Option<String> },
    OpenManager { value: Option<String> },
}

impl Apply<ShortcutsUpdate> for Shortcuts {
    fn apply(&mut self, u: ShortcutsUpdate) -> Result<()> {
        match u {
            ShortcutsUpdate::ToggleCanvasImode { value } => {
                self.toggle_canvas_imode = value;
            },
            ShortcutsUpdate::OpenManager { value } => {
                self.open_manager = value;
            },
        }
        Ok(())
    }
}

/// Application-wide settings.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
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

#[derive(Deserialize)]
#[serde(tag = "field", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppSettingsUpdate {
    Theme { value: Theme },
    CanvasImode { value: CanvasImode },
    Shortcuts { value: ShortcutsUpdate },
}

impl Apply<AppSettingsUpdate> for AppSettings {
    fn apply(&mut self, u: AppSettingsUpdate) -> Result<()> {
        match u {
            AppSettingsUpdate::Theme { value } => {
                self.theme = value;
            },
            AppSettingsUpdate::CanvasImode { value } => {
                self.canvas_imode = value;
            },
            AppSettingsUpdate::Shortcuts { value } => {
                self.shortcuts.apply(value)?;
            },
        }
        Ok(())
    }
}

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
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

#[derive(Deserialize)]
#[serde(tag = "field", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WidgetSettingsUpdate {
    X { value: i32 },
    Y { value: i32 },
    Opacity { value: i32 },
}

impl Apply<WidgetSettingsUpdate> for WidgetSettings {
    fn apply(&mut self, u: WidgetSettingsUpdate) -> Result<()> {
        match u {
            WidgetSettingsUpdate::X { value } => {
                self.x = value;
            },
            WidgetSettingsUpdate::Y { value } => {
                self.y = value;
            },
            WidgetSettingsUpdate::Opacity { value } => {
                if value < 0 || value > 100 {
                    bail!("Opacity must be between 0 and 100; got {value}");
                }
                self.opacity = value;
            },
        }
        Ok(())
    }
}

/// Full settings of the application.
#[derive(Clone, Default, Deserialize, Serialize, ts_rs::TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Settings {
    /// Application-wide settings.
    #[serde(default)]
    pub app: AppSettings,
    /// The mapping from widget IDs to their respective settings.
    #[serde(default)]
    pub widgets: HashMap<String, WidgetSettings>,
}

#[derive(Deserialize)]
#[serde(tag = "field", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettingsUpdate {
    App {
        value: AppSettingsUpdate,
    },
    Widget {
        key: String,
        value: WidgetSettingsUpdate,
    },
}

impl Apply<SettingsUpdate> for Settings {
    fn apply(&mut self, u: SettingsUpdate) -> Result<()> {
        match u {
            SettingsUpdate::App { value } => {
                self.app.apply(value)?;
            },
            SettingsUpdate::Widget { key, value } => {
                self.widgets.entry(key).or_default().apply(value)?;
            },
        }
        Ok(())
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

    /// Apply a sequence of updates to the settings.
    ///
    /// This will apply each update in the order they are provided. It will not
    /// terminate on error, but will accumulate any errors that occur and does
    /// the best effort to apply all updates. The update is not atomic.
    pub fn update<I>(&mut self, u: I) -> Result<()>
    where
        I: IntoIterator<Item = SettingsUpdate>,
    {
        let mut errors = Vec::new();
        for update in u {
            if let Err(e) = self.apply(update) {
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
                .join("; ");
            bail!("Settings update errors: {msg}")
        }
    }
}
