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
    /// Whether to ignore the widget.
    ///
    /// Setting this to `true` will exclude the widget from the widget collection.
    pub(crate) ignore: bool,
}

/// Node package configuration, corresponding to `package.json`.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct PackageJson {
    /// The `dependencies` field of `package.json`
    ///
    /// This is used for inferring the external dependencies of the widget.
    pub(crate) dependencies: HashMap<String, String>,
}

/// Read a widget directory into a widget configuration.
///
/// This function reads the `deskulpt.conf.json` file and optionally the `package.json`
/// file in the given widget directory `path`.
///
/// If widget configuration is loaded successfully, it will return `Ok(Some(config))`.
/// Any failure to load the configuration will return an error, except:
///
/// - If `deskulpt.conf.json` is not found in the given directory, we do not consider
///   it a widget and return `Ok(None)` instead of an error.
pub(crate) fn read_widget_config(
    path: &PathBuf,
) -> Result<Option<WidgetConfig>, Error> {
    if !path.is_absolute() || !path.is_dir() {
        bail!("Absolute path to a directory is expected; got: {:?}", path);
    }

    let deskulpt_conf_path = path.join("deskulpt.conf.json");
    let deskulpt_conf_str = match read_to_string(deskulpt_conf_path) {
        Ok(deskulpt_conf_str) => deskulpt_conf_str,
        Err(e) => {
            match e.kind() {
                // If the configuration file is not found we consider it not a widget
                // and ignore it without raising an error; in other cases, we do find
                // the configuration file but failed to read it, thus the error
                std::io::ErrorKind::NotFound => return Ok(None),
                _ => return Err(e).context("Failed to read deskulpt.conf.json"),
            }
        },
    };
    let deskulpt_conf = serde_json::from_str(&deskulpt_conf_str)
        .context("Failed to parse deskulpt.conf.json")?;

    let package_json_path = path.join("package.json");
    let package_json = if package_json_path.is_file() {
        let package_json_str = read_to_string(package_json_path)?;
        let package_json: PackageJson = serde_json::from_str(&package_json_str)
            .context("Failed to load package.json")?;
        Some(package_json)
    } else {
        None
    };

    Ok(Some(WidgetConfig {
        directory: path.to_path_buf(),
        deskulpt: deskulpt_conf,
        node: package_json,
    }))
}
