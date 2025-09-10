//! Configuration of Deskulpt widgets.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Helper trait for loading configuration files from a directory.
pub trait LoadFromConfigFile: Sized + DeserializeOwned {
    /// The name of the configuration file.
    const CONFIG_FILE_NAME: &'static str;

    /// Load the configuration file from the given directory.
    ///
    /// This method returns `Ok(None)` if the target file does not exist and
    /// `Err` if there is failure to read or parse the file.
    fn load(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join(Self::CONFIG_FILE_NAME);
        if !path.exists() {
            return Ok(None);
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(Some(config))
    }
}

/// Deserialized `deskulpt.conf.json`.
#[derive(Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DeskulptConf {
    /// The name of the widget.
    ///
    /// This is purely used for display purposes. It does not need to be related
    /// to the widget directory name, and it does not need to be unique.
    pub name: String,
    /// The entry point of the widget.
    ///
    /// This is the path to the file that exports the widget component. The path
    /// should be relative to the widget directory.
    pub entry: String,
    /// Whether to ignore the widget.
    ///
    /// If set to true, the widget will not be discovered by the application.
    /// This is useful for temporarily disabling a widget without removing it.
    #[serde(default, skip_serializing)]
    pub ignore: bool,
}

impl LoadFromConfigFile for DeskulptConf {
    const CONFIG_FILE_NAME: &'static str = "deskulpt.conf.json";
}

/// Deserialized `package.json`.
#[derive(Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

impl LoadFromConfigFile for Package {
    const CONFIG_FILE_NAME: &'static str = "package.json";
}
