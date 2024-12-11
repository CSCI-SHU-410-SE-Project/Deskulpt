//! This module invludes the global and per-widget settings and relevant utilities.

use crate::utils::IdMap;
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::BufWriter,
    path::Path,
};

/// The theme appearance.
#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ThemeAppearance {
    Light,
    #[default]
    Dark,
}

/// The global settings.
#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Settings {
    /// Dark/Light theme appearance.
    theme_appearance: ThemeAppearance,
    /// The keyboard shortcut for toggling the canvas.
    toggle_shortcut: Option<String>,
    /// The collection of per-widget settings.
    widget_settings: IdMap<WidgetSetting>,
}

/// The per-widget settings.
///
/// These are the settings that are not controlled by the configuration file but rather
/// controlled directly by the frontend.
#[derive(Deserialize, Serialize)]
pub(crate) struct WidgetSetting {
    /// The x-coordinate of the widget in pixels.
    x: i32,
    /// The y-coordinate of the widget in pixels.
    y: i32,
    /// The opacity of the widget in percentage.
    opacity: i32,
}

/// Read the widget internals.
///
/// This looks for `${app_config_dir}/.settings.json` and returns the widget internals
/// if the file exists and can be loaded correctly. Otherwise it returns an empty map.
pub(crate) fn read_settings(app_config_dir: &Path) -> Settings {
    let settings_path = app_config_dir.join(".settings.json");
    if !settings_path.exists() {
        return Default::default();
    }
    match read_to_string(settings_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Default::default(),
    }
}

/// Write the widget internals.
///
/// This writes the widget internals to `${app_config_dir}/.settings.json`. It will
/// create the file if it does not exist, and overwrite the file if it does.
pub(crate) fn write_settings(
    app_config_dir: &Path,
    settings: &Settings,
) -> Result<(), Error> {
    // On certain platform, if the file directory does not exist, file creation using std::fs:;File::create will fail.
    // See [the doc](https://doc.rust-lang.org/std/fs/struct.File.html#method.create) for detail.
    // Thus we need to manually create app_config_dir if it doesn't exist.
    if !app_config_dir.exists() {
        create_dir_all(app_config_dir)?;
    }

    let file = File::create(app_config_dir.join(".settings.json"))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, settings)?;
    Ok(())
}
