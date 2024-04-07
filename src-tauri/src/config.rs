//! The module implements configuration-related utilities and structures.

use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Error};
use serde::{Deserialize, Serialize};

/// Full configuration of a widget.
#[derive(Clone, Serialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
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
#[cfg_attr(test, derive(PartialEq, Debug))]
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
#[cfg_attr(test, derive(PartialEq, Debug))]
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
/// If the directory should not be treated as a widget, it will return `Ok(None)`. Any
/// failure to load the configuration will return an error.
///
/// The cases where a directory should not be treated as a widget include:
/// - `deskulpt.conf.json` is not found.
/// - The `ignore` flag in `deskulpt.conf.json` is set to `true`.
pub(crate) fn read_widget_config(path: &Path) -> Result<Option<WidgetConfig>, Error> {
    if !path.is_absolute() || !path.is_dir() {
        // Note that `is_dir` also checks if the path exists
        bail!("Absolute path to an existing directory is expected; got: {path:?}");
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
    let deskulpt_conf: DeskulptConf = serde_json::from_str(&deskulpt_conf_str)
        .context("Failed to parse deskulpt.conf.json")?;

    // Respect the `ignore` flag in configuration
    if deskulpt_conf.ignore {
        return Ok(None);
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_return_none() {
        // Check the cases we return `Ok(None)`, i.e., no configuration file or the
        // ignore flag is set to `true`
        for widget_dir in [
            Path::new("tests/fixtures/config/no_conf").canonicalize().unwrap(),
            Path::new("tests/fixtures/config/ignore_true").canonicalize().unwrap(),
        ] {
            let result = read_widget_config(&widget_dir);
            assert!(result.is_ok());
            assert!(result.unwrap().is_none());
        }
    }

    #[test]
    fn test_read_invalid_path_error() {
        // Check that we get an error unless we pass an absolute path to a directory,
        // i.e., error if path is not absolute, not a directory, or does not exist
        for widget_dir in [
            PathBuf::from("tests/fixtures/config/full"),
            Path::new("tests/fixtures/config/dummy").canonicalize().unwrap(),
            Path::new("tests/fixtures/config")
                .canonicalize()
                .unwrap()
                .join("non_existent"),
        ] {
            let result = read_widget_config(&widget_dir);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                format!(
                    "Absolute path to an existing directory is expected; got: \
                    {widget_dir:?}"
                ),
            );
        }
    }
}
