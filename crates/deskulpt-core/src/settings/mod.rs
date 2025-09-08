//! Application and widget settings.

use std::collections::BTreeMap;

use deskulpt_macros::Persisted;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
