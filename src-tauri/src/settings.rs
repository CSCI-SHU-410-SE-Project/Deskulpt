use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::BufWriter,
    path::Path,
};

/// The global settings.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Settings {
    /// The keyboard shortcut for toggling the canvas.
    toggle_shorcut: Option<String>,
    /// The collection of per-widget settings.
    widget_settings: HashMap<String, WidgetSetting>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { toggle_shorcut: None, widget_settings: Default::default() }
    }
}

/// The per-widget settings.
///
/// These are the settings that are not controlled by the configuration file but rather
/// controlled directly by the frontend.
#[derive(Deserialize, Serialize)]
pub(crate) struct WidgetSetting {
    /// The x-coordinate of the widget.
    x: i32,
    /// The y-coordinate of the widget.
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
    let file = File::create(app_config_dir.join(".settings.json"))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, settings)?;
    Ok(())
}
