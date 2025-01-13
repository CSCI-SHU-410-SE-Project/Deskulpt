//! Configuration of Deskulpt widgets.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Deserialized `deskulpt.conf.json`.
#[derive(Clone, Deserialize, Serialize)]
struct DeskulptConf {
    name: String,
    entry: String,
    #[serde(default)]
    ignore: bool,
}

/// Deserialized `package.json`.
#[derive(Deserialize)]
struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
}

/// Full configuration of a Deskulpt widget.
#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct WidgetConfig {
    /// Name of the widget.
    name: String,
    /// Entry file of the widget, relative to the widget directory.
    entry: String,
    /// Whether the widget is ignored.
    ignore: bool,
    /// The dependency mapping from package names to versions.
    dependencies: HashMap<String, String>,
}

/// The widget collection.
///
/// This is a mapping from widget IDs to either widget configurations if valid
/// or configuration error messages otherwise.
pub type WidgetCollection = HashMap<String, Result<WidgetConfig, String>>;

impl WidgetConfig {
    /// Read widget configuratoin from a directory.
    pub fn load<P: AsRef<Path>>(dir: P) -> Result<Option<Self>> {
        let dir = dir.as_ref();
        debug_assert!(dir.is_absolute() && dir.is_dir());

        let deskulpt_conf_path = dir.join("deskulpt.conf.json");
        if !deskulpt_conf_path.exists() {
            return Ok(None);
        }
        let deskulpt_conf_file =
            File::open(&deskulpt_conf_path).context("Failed to open deskulpt.conf.json")?;
        let deskulpt_conf_reader = BufReader::new(deskulpt_conf_file);
        let deskulpt_conf: DeskulptConf = serde_json::from_reader(deskulpt_conf_reader)
            .context("Failed to parse deskulpt.conf.json")?;

        // Ignore widgets that are explcitly marked as such
        if deskulpt_conf.ignore {
            return Ok(None);
        }

        let package_json_path = dir.join("package.json");
        let dependencies = if package_json_path.exists() {
            let package_json_file =
                File::open(&package_json_path).context("Failed to open package.json")?;
            let reader = BufReader::new(package_json_file);
            let package_json: PackageJson =
                serde_json::from_reader(reader).context("Failed to parse package.json")?;
            package_json.dependencies
        } else {
            Default::default()
        };

        Ok(Some(WidgetConfig {
            name: deskulpt_conf.name,
            entry: deskulpt_conf.entry,
            ignore: deskulpt_conf.ignore,
            dependencies,
        }))
    }

    /// Get the entry file of the widget.
    pub fn entry(&self) -> String {
        self.entry.to_string()
    }

    /// Get the set of external dependencies of the widget.
    pub fn external_deps(&self) -> HashSet<String> {
        self.dependencies.keys().cloned().collect()
    }
}
