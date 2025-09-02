//! Configuration of Deskulpt widgets.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    ignore: bool,
}

/// Deserialized `package.json`.
#[derive(Clone, Default, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

/// Helper trait for loading configuration files from a directory.
trait LoadFromFile: Sized + DeserializeOwned {
    /// The name of the configuration file.
    const FILE_NAME: &'static str;

    /// Load the configuration file from the given directory.
    ///
    /// This method returns `Ok(None)` if the target file does not exist and
    /// `Err` if there is failure to read or parse the file.
    fn load(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join(Self::FILE_NAME);
        if !path.exists() {
            return Ok(None);
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(Some(config))
    }
}

impl LoadFromFile for DeskulptConf {
    const FILE_NAME: &'static str = "deskulpt.conf.json";
}

impl LoadFromFile for PackageJson {
    const FILE_NAME: &'static str = "package.json";
}

/// Full configuration of a Deskulpt widget.
#[derive(Clone, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WidgetConfig {
    /// Valid widget configuration.
    #[serde(rename_all = "camelCase")]
    Valid {
        /// The directory name of the widget.
        dir: String,
        /// The required `deskulpt.conf.json` configuration.
        deskulpt_conf: DeskulptConf,
        /// The optional `package.json` configuration.
        package_json: Option<PackageJson>,
    },
    /// Invalid widget configuration.
    #[serde(rename_all = "camelCase")]
    Invalid {
        /// The directory name of the widget.
        dir: String,
        /// Error message.
        error: String,
    },
}

impl WidgetConfig {
    /// Read widget configuration from a directory.
    ///
    /// This returns `None` if the directory is not considered a widget
    /// directory, or if the widget is explicitly marked as ignored.
    pub fn load<P: AsRef<Path>>(dir: P) -> Option<Self> {
        let dir = dir.as_ref();
        let dir_name = dir.file_name()?.to_string_lossy();

        let deskulpt_conf =
            match DeskulptConf::load(dir).context("Failed to load deskulpt.conf.json") {
                Ok(Some(deskulpt_conf)) => deskulpt_conf,
                Ok(None) => return None,
                Err(e) => {
                    return Some(WidgetConfig::Invalid {
                        dir: dir_name.to_string(),
                        error: format!("{e:?}"),
                    })
                },
            };

        // Ignore widgets that are explcitly marked as such
        if deskulpt_conf.ignore {
            return None;
        }

        let package_json = match PackageJson::load(dir).context("Failed to load package.json") {
            Ok(package_json) => package_json,
            Err(e) => {
                return Some(WidgetConfig::Invalid {
                    dir: dir_name.to_string(),
                    error: format!("{e:?}"),
                })
            },
        };

        Some(WidgetConfig::Valid {
            dir: dir_name.to_string(),
            deskulpt_conf,
            package_json,
        })
    }

    /// Get the directory of the widget inside the widgets directory.
    pub fn dir(&self) -> &str {
        match self {
            WidgetConfig::Valid { dir, .. } => dir,
            WidgetConfig::Invalid { dir, .. } => dir,
        }
    }

    /// Get the widget ID.
    ///
    /// This ID is derived from the widget directory name using UUID v5. It is
    /// deterministic for the same directory name.
    pub fn id(&self) -> String {
        let dir_encoded = self.dir().as_bytes();
        Uuid::new_v5(&Uuid::NAMESPACE_URL, dir_encoded).to_string()
    }
}
