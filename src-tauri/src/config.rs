//! The module implements configuration-related utilities and structures.

use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use anyhow::{bail, Context, Error};
use serde::{Deserialize, Serialize};

/// Full configuration of a widget.
#[derive(Clone, Serialize)]
pub(crate) struct WidgetConfig {
    /// Deskulpt configuration [`DeskulptConf`].
    pub(crate) deskulpt: DeskulptConf,

    /// Node package configuration [`PackageJson`], optional.
    pub(crate) node: Option<PackageJson>,

    /// Absolute path to the widget directory.
    ///
    /// It is absolute so that we do not need to query the widget base directory state
    /// [`crate::states::WidgetBaseDirectoryState`] and call join to be able to obtain
    /// the absolute path.
    pub(crate) directory: PathBuf,
}

/// Deskulpt configuration of a widget, corresponding to `deskulpt.conf.json`.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct DeskulptConf {
    /// The name of the widget.
    pub(crate) name: String,

    /// The entry file of the widget, relative to the widget directory.
    pub(crate) entry: String,

    /// Whether to ignore the widget. Setting this to `true` will exclude the widget
    /// from the widget collection.
    pub(crate) ignore: bool,
}

/// Node package configuration, corresponding to `package.json`.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct PackageJson {
    /// The `dependencies` field of `package.json`, used for implying external
    /// dependencies of the widget.
    pub(crate) dependencies: HashMap<String, String>,
}

/// Read a widget directory into a widget configuration.
///
/// This function reads the `deskulpt.conf.json` file and optionally the `package.json`
/// file in the given widget directory `path`.
///
/// @Charlie-XIAO Refine the function to raise better errors and describe here.
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

    Ok(WidgetConfig {
        directory: path.to_path_buf(),
        deskulpt: deskulpt_conf,
        node: package_json,
    })
}
