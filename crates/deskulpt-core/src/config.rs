use std::collections::{BTreeMap, HashMap};
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Helper trait for loading configuration files from a directory.
trait LoadFromConfigFile: DeserializeOwned + Sized {
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

/// Deserialized `deskulpt.conf.json` configuration.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeskulptConf {
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

/// Deserialized `package.json` manifest.
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageManifest {
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

impl LoadFromConfigFile for PackageManifest {
    const CONFIG_FILE_NAME: &'static str = "package.json";
}

#[derive(Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WidgetConfig {
    Ok {
        /// The name of the widget.
        name: String,
        /// The entry point of the widget.
        entry: String,
        /// The dependencies of the widget.
        dependencies: HashMap<String, String>,
    },
    Err {
        /// The error message if the widget failed to load.
        error: String,
    },
}

impl WidgetConfig {
    fn err(error: String) -> Self {
        WidgetConfig::Err { error }
    }

    /// Load widget configuration from a directory.
    ///
    /// If the directory is not considered a widget directory or if the widget
    /// is explicitly marked as ignored, this method returns `None`. If there is
    /// any error loading the widget, it returns an [`WidgetConfig::Ok`]
    /// variant. Otherwise, it returns a [`WidgetConfig::Err`] variant.
    fn load(dir: &Path) -> Option<Self> {
        let deskulpt_conf = match DeskulptConf::load(dir)
            .with_context(|| format!("Failed to load {}", DeskulptConf::CONFIG_FILE_NAME))
        {
            Ok(Some(deskulpt_conf)) => deskulpt_conf,
            Ok(None) => return None,
            Err(e) => {
                return Some(WidgetConfig::err(format!("{e:?}")));
            },
        };

        // Ignore widgets that are explcitly marked as such
        if deskulpt_conf.ignore {
            return None;
        }

        let package_manifest = match PackageManifest::load(dir)
            .with_context(|| format!("Failed to load {}", PackageManifest::CONFIG_FILE_NAME))
        {
            Ok(package_manifest) => package_manifest.unwrap_or_default(),
            Err(e) => {
                return Some(WidgetConfig::err(format!("{e:?}")));
            },
        };

        Some(WidgetConfig::Ok {
            name: deskulpt_conf.name,
            entry: deskulpt_conf.entry,
            dependencies: package_manifest.dependencies,
        })
    }
}

#[derive(Clone, Default, Serialize, Deserialize, specta::Type)]
pub struct WidgetConfigRegistry(pub BTreeMap<String, WidgetConfig>);

impl WidgetConfigRegistry {
    pub fn load(dir: &Path) -> Result<Self> {
        let mut registry = Self::default();

        let entries = read_dir(dir)?;
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if !path.is_dir() {
                continue; // Non-directory entries are not widgets, skip
            }

            if let Some(config) = WidgetConfig::load(&path) {
                // Since each widget must be at the top level of the widgets
                // directory, the directory names must be unique and we can use
                // them as widget IDs
                let id = entry.file_name().to_string_lossy().to_string();
                registry.0.insert(id, config);
            }
        }

        Ok(registry)
    }
}
