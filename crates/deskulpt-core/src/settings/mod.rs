//! Application and widget settings.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, MapSkipError};

mod persistence;
mod shortcuts;

/// Light/dark theme of the application.
#[derive(Clone, Default, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

/// Types of keyboard shortcuts in the application.
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
#[serde_as]
#[derive(Clone, Default, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct AppSettings {
    /// The application theme.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub theme: Theme,
    /// The keyboard shortcuts.
    #[serde_as(deserialize_as = "MapSkipError<_, _>")]
    pub shortcuts: BTreeMap<ShortcutKey, String>,
}

/// Per-widget settings.
///
/// Different from widget configurations, these are independent of the widget
/// configuration files and are managed internally by the application.
#[derive(Clone, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct WidgetSettings {
    /// The leftmost x-coordinate in pixels.
    pub x: i32,
    /// The topmost y-coordinate in pixels.
    pub y: i32,
    /// The opacity in percentage.
    pub opacity: i32,
}

impl Default for WidgetSettings {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            opacity: 100,
        }
    }
}

/// Full settings of the Deskulpt application.
#[serde_as]
#[derive(Clone, Default, Deserialize, Serialize, JsonSchema, specta::Type)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    /// Application-wide settings.
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub app: AppSettings,
    /// The mapping from widget IDs to their respective settings.
    #[serde_as(deserialize_as = "MapSkipError<_, _>")]
    pub widgets: BTreeMap<String, WidgetSettings>,
}
