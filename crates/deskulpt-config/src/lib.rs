//! The module implements configuration-related utilities and structures.

#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Error};
use deskulpt_test_utils::IdMap;
use serde::{Deserialize, Serialize};

/// The collection of widget configurations or errors.
pub type WidgetConfigMap = IdMap<Result<WidgetConfig, String>>;

/// Full configuration of a widget.
#[derive(Clone, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfig {
    /// Deskulpt configuration [`DeskulptConf`].
    pub deskulpt_conf: DeskulptConf,
    /// External dependencies, empty if None.
    pub external_deps: HashMap<String, String>,
    /// Absolute path to the widget directory.
    ///
    /// It is absolute so that we do not need to query the widget base directory
    /// state and call join to be able to obtain the absolute path.
    pub directory: PathBuf,
}

/// Deskulpt configuration of a widget, corresponding to `deskulpt.conf.json`.
#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct DeskulptConf {
    /// The name of the widget.
    pub name: String,
    /// The entry file of the widget, relative to the widget directory.
    pub entry: String,
    /// Whether to ignore the widget.
    ///
    /// Setting this to `true` will exclude the widget from the widget
    /// collection.
    pub ignore: bool,
}

#[derive(Deserialize)]
struct PackageJson {
    dependencies: Option<HashMap<String, String>>,
}

/// Read a widget directory into a widget configuration.
///
/// This function reads the `deskulpt.conf.json` file and optionally the
/// `package.json` file in the given widget directory `path`.
///
/// If widget configuration is loaded successfully, it will return
/// `Ok(Some(config))`. If the directory does not represent a widget that is
/// meant to be rendered, it will return `Ok(None)`. Any failure to load the
/// configuration will return an error.
///
/// The cases where a directory is not meant to be rendered include:
/// - `deskulpt.conf.json` is not found.
/// - The `ignore` flag in `deskulpt.conf.json` is set to `true`.
pub fn read_widget_config(path: &Path) -> Result<Option<WidgetConfig>, Error> {
    if !path.is_absolute() || !path.is_dir() {
        // We require absolute path because it will be directly used as the widget
        // directory in the configuration; there is no need to check path existence
        // because `is_dir` already does that
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
    let external_deps = if package_json_path.exists() {
        let package_json_str =
            read_to_string(package_json_path).context("Failed to read package.json")?;
        let package_json: PackageJson =
            serde_json::from_str(&package_json_str).context("Failed to interpret package.json")?;
        package_json.dependencies.unwrap_or_default()
    } else {
        Default::default()
    };

    Ok(Some(WidgetConfig {
        directory: path.to_path_buf(),
        deskulpt_conf,
        external_deps,
    }))
}

#[cfg(test)]
mod tests {
    use deskulpt_test_testing::assert::{assert_eq, assert_err_eq, ChainReason};
    use deskulpt_test_testing::fixture_path;
    use rstest::rstest;

    use super::*;

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
        fixture_path("config/standard"),
        Some(WidgetConfig {
            directory: fixture_path("config/standard"),
            deskulpt_conf: get_standard_deskulpt_conf(),
            external_deps: [("express".to_string(), "^4.17.1".to_string())].into(),
        }),
    )]
    // A standard configuration with `deskulpt.conf.json` but no `package.json`
    #[case::no_package_json(
        fixture_path("config/no_package_json"),
        Some(WidgetConfig {
            directory: fixture_path("config/no_package_json"),
            deskulpt_conf: get_standard_deskulpt_conf(),
            external_deps: HashMap::new(),
        }),
    )]
    // `package.json` does not contain `dependencies` field
    #[case::package_json_no_dependencies(
        fixture_path("config/package_json_no_dependencies"),
        Some(WidgetConfig {
            directory: fixture_path("config/package_json_no_dependencies"),
            deskulpt_conf: get_standard_deskulpt_conf(),
            external_deps: HashMap::new(),
        }),
    )]
    // No configuration file, should not be treated as a widget
    #[case::no_conf(fixture_path("config/no_conf"), None)]
    // Widget is explicitly ignored
    #[case::ignore_true(fixture_path("config/ignore_true"), None)]
    fn test_read_ok(#[case] path: PathBuf, #[case] expected_config: Option<WidgetConfig>) {
        let result =
            read_widget_config(&path).expect("Expected successful read of widget configuration");
        assert_eq!(result, expected_config);
    }

    #[rstest]
    // Input path is not absolute
    #[case::not_absolute(
        "config/not_absolute",
        vec![ChainReason::Exact(
            "Absolute path to an existing directory is expected; got: \
            config/not_absolute".to_string()
        )],
    )]
    // Input path is not a directory
    #[case::not_dir(
        fixture_path("config/not_a_directory"),
        vec![ChainReason::Exact(format!(
            "Absolute path to an existing directory is expected; got: {}",
            fixture_path("config/not_a_directory").display(),
        ))],
    )]
    // Input path does not exist
    #[case::non_existent(
        fixture_path("config/non_existent"),
        vec![ChainReason::Exact(format!(
            "Absolute path to an existing directory is expected; got: {}",
            fixture_path("config/non_existent").display(),
        ))],
    )]
    // `deskulpt.conf.json` is not readable (is a directory)
    #[case::conf_not_readable(
        fixture_path("config/conf_not_readable"),
        vec![
            ChainReason::Exact("Failed to read deskulpt.conf.json".to_string()),
            ChainReason::IOError,
        ],
    )]
    // `deskulpt.conf.json` is missing a field
    #[case::conf_missing_field(
        fixture_path("config/conf_missing_field"),
        vec![
            ChainReason::Exact("Failed to interpret deskulpt.conf.json".to_string()),
            ChainReason::SerdeError,
        ],
    )]
    // `package.json` is not readable (is a directory)
    #[case::package_json_not_readable(
        fixture_path("config/package_json_not_readable"),
        vec![
            ChainReason::Exact("Failed to read package.json".to_string()),
            ChainReason::IOError,
        ],
    )]
    fn test_read_error(#[case] path: PathBuf, #[case] expected_error: Vec<ChainReason>) {
        let error =
            read_widget_config(&path).expect_err("Expected an error reading widget configuration");
        assert_err_eq(error, expected_error);
    }
}
