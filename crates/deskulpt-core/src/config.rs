//! Configuration of Deskulpt widgets.

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Deserialized `deskulpt.conf.json`.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Default, Deserialize)]
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
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WidgetConfig {
    /// Valid configuration of a widget.
    #[serde(rename_all = "camelCase")]
    Ok {
        /// The name of the widget.
        name: String,
        /// The entry point of the widget.
        entry: String,
        /// The dependencies of the widget.
        dependencies: HashMap<String, String>,
    },
    /// Error information if a widget failed to load.
    #[serde(rename_all = "camelCase")]
    Err {
        /// The error message.
        error: String,
    },
}

impl WidgetConfig {
    /// Read widget configuration from a directory.
    ///
    /// This returns `None` if the directory is not considered a widget
    /// directory, or if the widget is explicitly marked as ignored.
    pub fn load(dir: &Path) -> Option<Self> {
        debug!(path = %dir.display(), "Inspecting directory for widget metadata");
        let deskulpt_conf =
            match DeskulptConf::load(dir).context("Failed to load deskulpt.conf.json") {
                Ok(Some(deskulpt_conf)) => deskulpt_conf,
                Ok(None) => {
                    debug!(path = %dir.display(), "Missing deskulpt.conf.json; skipping directory");
                    return None;
                },
                Err(e) => {
                    warn!(path = %dir.display(), error = ?e, "Error loading deskulpt.conf.json");
                    return Some(WidgetConfig::Err {
                        error: format!("{e:?}"),
                    });
                },
            };

        // Ignore widgets that are explicitly marked as such
        if deskulpt_conf.ignore {
            info!(
                path = %dir.display(),
                widget_name = %deskulpt_conf.name,
                "Widget marked as ignored; skipping"
            );
            return None;
        }

        let package_json = match PackageJson::load(dir).context("Failed to load package.json") {
            Ok(package_json) => package_json.unwrap_or_default(),
            Err(e) => {
                warn!(path = %dir.display(), error = ?e, "Error loading package.json");
                return Some(WidgetConfig::Err {
                    error: format!("{e:?}"),
                });
            },
        };

        debug!(
            path = %dir.display(),
            entry = %deskulpt_conf.entry,
            dependency_count = package_json.dependencies.len(),
            "Widget configuration loaded"
        );

        Some(WidgetConfig::Ok {
            name: deskulpt_conf.name,
            entry: deskulpt_conf.entry,
            dependencies: package_json.dependencies,
        })
    }
}

/// The widget catalog.
///
/// This is a collection of all widgets discovered locally, mapped from their
/// widget IDs to their configurations.
#[derive(Debug, Default, Clone, Serialize, specta::Type)]
pub struct WidgetCatalog(pub BTreeMap<String, WidgetConfig>);

impl WidgetCatalog {
    /// Load the widget catalog from the given directory.
    ///
    /// This scans all top-level subdirectories and attempts to load them as
    /// widgets. Non-widget directories are simply ignored. See
    /// [`WidgetConfig::load`] for more details.
    pub fn load(dir: &Path) -> Result<Self> {
        let mut catalog = Self::default();

        let entries = std::fs::read_dir(dir)?;
        info!(path = %dir.display(), "Scanning for widgets");
        for entry in entries {
            let entry = entry?;

            let path = entry.path();
            if !path.is_dir() {
                debug!(path = %path.display(), "Skipping non-directory entry");
                continue; // Non-directory entries are not widgets, skip
            }

            if let Some(config) = WidgetConfig::load(&path) {
                // Since each widget must be at the top level of the widgets
                // directory, the directory names must be unique and we can use
                // them as widget IDs
                let id = entry.file_name().to_string_lossy().to_string();
                match &config {
                    WidgetConfig::Ok {
                        name,
                        entry,
                        dependencies,
                    } => {
                        info!(
                            widget_id = %id,
                            widget_name = %name,
                            entry = %entry,
                            dependency_count = dependencies.len(),
                            "Widget detected"
                        );
                    },
                    WidgetConfig::Err { error } => {
                        warn!(widget_id = %id, error = %error, "Widget configuration error");
                    },
                }
                catalog.0.insert(id, config);
            } else {
                debug!(path = %path.display(), "Directory skipped; not a Deskulpt widget");
            }
        }

        info!(
            path = %dir.display(),
            widget_count = catalog.0.len(),
            "Widget scanning completed"
        );
        Ok(catalog)
    }
}
