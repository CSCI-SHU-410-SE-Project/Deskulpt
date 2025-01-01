#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::path::Path;

use anyhow::Error;
use serde::{Deserialize, Serialize};

/// The settings file name in the application data directory.
static SETTINGS_FILE: &str = "settings.bin";

/// The per-widget settings.
///
/// Different from widget configurations, these are not configured by the
/// configuration files but managed internally by the application.
#[derive(Deserialize, Serialize)]
struct WidgetSetting {
    /// The leftmost x-coordinate in pixels.
    x: i32,
    /// The topmost y-coordinate in pixels.
    y: i32,
    /// The opacity in percentage.
    opacity: i32,
}

/// The light/dark theme appearance of the application.
#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ThemeAppearance {
    #[default]
    Light,
    Dark,
}

/// The global settings.
#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSetting {
    /// The theme appearance.
    theme_appearance: ThemeAppearance,
    /// The keyboard shortcut for toggling canvas click-through.
    toggle_shortcut: Option<String>,
    /// The mapping from widget IDs to their respective settings.
    widget_settings: HashMap<String, WidgetSetting>,
}

impl GlobalSetting {
    /// Read the settings.
    ///
    /// The default (empty) settings will be returned if the settings file does
    /// not exist or cannot be loaded correctly.
    pub fn read<P: AsRef<Path>>(app_data_dir: P) -> Self {
        let settings_file = app_data_dir.as_ref().join(SETTINGS_FILE);
        if !settings_file.exists() {
            return Default::default();
        }

        match File::open(settings_file) {
            Ok(file) => bincode::deserialize_from(file).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    /// Try to write the settings.
    ///
    /// The settings file, if exists, will be overwritten, otherwise it will be
    /// created and written. Any error will be propagated.
    pub fn try_write<P: AsRef<Path>>(&self, app_data_dir: P) -> Result<(), Error> {
        // On certain platforms, File::create fails if intermediate directories
        // do not exist, in which case we need to manually create the directory;
        // see https://doc.rust-lang.org/std/fs/struct.File.html#method.create
        let app_data_dir = app_data_dir.as_ref();
        if !app_data_dir.exists() {
            create_dir_all(app_data_dir)?;
        }

        let file = File::create(app_data_dir.join(SETTINGS_FILE))?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)?;
        Ok(())
    }
}
