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
    if !path.is_dir() {
        // Note that `is_dir` also checks if the path exists; we require absolute path
        // because it will be directly used as the widget directory in the configuration
        bail!(
            "Absolute path to an existing directory is expected; got: {}",
            path.display()
        );
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
        .context("Failed to interpret deskulpt.conf.json")?;

    // Respect the `ignore` flag in configuration
    if deskulpt_conf.ignore {
        return Ok(None);
    }

    let package_json_path = path.join("package.json");
    let package_json = if package_json_path.exists() {
        let package_json_str =
            read_to_string(package_json_path).context("Failed to read package.json")?;
        let package_json: PackageJson = serde_json::from_str(&package_json_str)
            .context("Failed to interpret package.json")?;
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
    use std::env::current_dir;

    use super::*;
    use crate::testing::{assert_err_eq, ChainReason};
    use path_clean::PathClean;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    /// Get the absolute path to the fixture directory.
    fn fixture_dir() -> PathBuf {
        current_dir().unwrap().join("tests/fixtures/config").clean()
    }

    /// Get the standard Deskulpt configuration.
    fn get_standard_deskulpt_conf() -> DeskulptConf {
        DeskulptConf {
            name: "sample".to_string(),
            entry: "index.jsx".to_string(),
            ignore: false,
        }
    }

    #[rstest]
    // A standard configuration with both `deskulpt.conf.json` and `package.json`
    #[case::standard(
        fixture_dir().join("standard"),
        Some(WidgetConfig {
            directory: fixture_dir().join("standard"),
            deskulpt: get_standard_deskulpt_conf(),
            node: Some(PackageJson {
                dependencies: [("express".to_string(), "^4.17.1".to_string())].into(),
            }),
        }),
    )]
    // A standard configuration with `deskulpt.conf.json` but no `package.json`
    #[case::no_package_json(
        fixture_dir().join("no_package_json"),
        Some(WidgetConfig {
            directory: fixture_dir().join("no_package_json"),
            deskulpt: get_standard_deskulpt_conf(),
            node: None,
        }),
    )]
    // No configuration file, should not be treated as a widget
    #[case::no_conf(fixture_dir().join("no_conf"), None)]
    // Widget is explicitly ignored
    #[case::ignore_true(fixture_dir().join("ignore_true"), None)]
    fn test_read_ok(
        #[case] path: PathBuf,
        #[case] expected_config: Option<WidgetConfig>,
    ) {
        let result = read_widget_config(&path);
        assert!(result.is_ok(), "Expected successful read of widget configuration");

        let result = result.unwrap();
        assert_eq!(result, expected_config);
    }

    #[rstest]
    // Input path is not absolute
    #[case::not_absolute(
        "tests/fixtures/config/not_absolute",
        vec![ChainReason::Exact(
            "Absolute path to an existing directory is expected; got: \
            tests/fixtures/config/not_absolute".to_string()
        )],
    )]
    // Input path is not a directory
    #[case::not_dir(
        fixture_dir().join("not_a_directory"),
        vec![ChainReason::Exact(format!(
            "Absolute path to an existing directory is expected; got: {}",
            fixture_dir().join("not_a_directory").display(),
        ))],
    )]
    // Input path does not exist
    #[case::non_existent(
        fixture_dir().join("non_existent"),
        vec![ChainReason::Exact(format!(
            "Absolute path to an existing directory is expected; got: {}",
            fixture_dir().join("non_existent").display(),
        ))],
    )]
    // `deskulpt.conf.json` is not readable (is a directory)
    #[case::conf_not_readable(
        fixture_dir().join("conf_not_readable"),
        vec![
            ChainReason::Exact("Failed to read deskulpt.conf.json".to_string()),
            ChainReason::IOError,
        ],
    )]
    // `deskulpt.conf.json` is missing a field
    #[case::conf_missing_field(
        fixture_dir().join("conf_missing_field"),
        vec![
            ChainReason::Exact("Failed to interpret deskulpt.conf.json".to_string()),
            ChainReason::SerdeError,
        ],
    )]
    // `package.json` is not readable (is a directory)
    #[case::package_json_not_readable(
        fixture_dir().join("package_json_not_readable"),
        vec![
            ChainReason::Exact("Failed to read package.json".to_string()),
            ChainReason::IOError,
        ],
    )]
    // `package.json` is missing a field
    #[case::package_json_missing_field(
        fixture_dir().join("package_json_missing_field"),
        vec![
            ChainReason::Exact("Failed to interpret package.json".to_string()),
            ChainReason::SerdeError,
        ],
    )]
    fn test_read_error(
        #[case] path: PathBuf,
        #[case] expected_error: Vec<ChainReason>,
    ) {
        let result = read_widget_config(&path);
        assert!(result.is_err(), "Expected an error reading widget configuration");

        let error = result.unwrap_err();
        assert_err_eq(error, expected_error);
    }
}
