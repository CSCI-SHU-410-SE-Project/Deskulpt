use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use anyhow::{bail, Context, Error};
use serde::{Deserialize, Serialize};

// Full configuration of a widget
#[derive(Clone, Serialize)]
pub(crate) struct WidgetConfig {
    pub(crate) deskulpt_conf: DeskulptConf,
    pub(crate) package_json: Option<PackageJson>,
    pub(crate) directory: PathBuf, // Absolute path to the widget directory
}

// The structure corresponding to deskulpt.conf.json
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct DeskulptConf {
    pub(crate) name: String,
    pub(crate) entry: String,
    pub(crate) ignore: bool,
}

// The structure corresponding to package.json
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct PackageJson {
    pub(crate) dependencies: HashMap<String, String>,
}

// Read a widget folder into the widget configuration
// @Charlie-XIAO Error handling should be done with higher granularity
pub(crate) fn read_widget_config(path: &PathBuf) -> Result<WidgetConfig, Error> {
    if !path.is_absolute() {
        bail!("Path must be absolute; got: {:?}", path);
    }

    let deskulpt_conf_path = path.join("deskulpt.conf.json");
    let deskulpt_conf_str = read_to_string(deskulpt_conf_path)?;
    let deskulpt_conf: DeskulptConf = serde_json::from_str(&deskulpt_conf_str)
        .context("Failed to load deskulpt.conf.json")?;

    let package_json_path = path.join("package.json");
    let package_json = if package_json_path.is_file() {
        let package_json_str = read_to_string(package_json_path)?;
        let package_json: PackageJson = serde_json::from_str(&package_json_str)
            .context("Failed to load package.json")?;
        Some(package_json)
    } else {
        None
    };

    Ok(WidgetConfig { directory: path.to_path_buf(), deskulpt_conf, package_json })
}
