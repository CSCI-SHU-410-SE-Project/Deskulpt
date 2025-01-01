#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Error};
use path_clean::PathClean;
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
    dependencies: Option<HashMap<String, String>>,
}

/// Full configuration of a widget.
#[derive(Clone, Serialize, PartialEq, Debug)]
pub struct WidgetConfig {
    name: String,
    entry: String,
    ignore: bool,
    dependencies: HashMap<String, String>,
    directory: PathBuf,
}

impl WidgetConfig {
    /// Try to read widget configuration from a directory.
    ///
    /// The widget directory must be given as an absolute path. None is returned
    /// if the directory is not a widget or an ignored widget.
    pub fn try_read<P: AsRef<Path>>(path: P) -> Result<Option<Self>, Error> {
        let path = path.as_ref();
        if !path.is_absolute() || !path.is_dir() {
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
        let dependencies = if package_json_path.exists() {
            let package_json_str =
                read_to_string(package_json_path).context("Failed to read package.json")?;
            let package_json: PackageJson = serde_json::from_str(&package_json_str)
                .context("Failed to interpret package.json")?;
            package_json.dependencies.unwrap_or_default()
        } else {
            Default::default()
        };

        Ok(Some(WidgetConfig {
            directory: path.to_path_buf(),
            name: deskulpt_conf.name,
            entry: deskulpt_conf.entry,
            ignore: deskulpt_conf.ignore,
            dependencies,
        }))
    }

    /// Get the directory path of the widget.
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// Get the absolute path to the entry file of the widget.
    pub fn entry_path(&self) -> PathBuf {
        self.directory.join(&self.entry).clean()
    }

    /// Get the external dependencies of the widget.
    pub fn external_deps(&self) -> HashSet<String> {
        self.dependencies.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use deskulpt_test_testing::assert::{assert_eq, assert_err_eq, ChainReason};
    use deskulpt_test_testing::fixture_path;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::all_fields(
        fixture_path("deskulpt-config/widgets/all_fields"),
        Some(WidgetConfig {
            directory: fixture_path("deskulpt-config/widgets/all_fields"),
            name: "all_fields".to_string(),
            entry: "index.jsx".to_string(),
            ignore: false,
            dependencies: [("express".to_string(), "^4.17.1".to_string())].into(),
        }),
    )]
    #[case::package_json_none(
        fixture_path("deskulpt-config/widgets/package_json_none"),
        Some(WidgetConfig {
            directory: fixture_path("deskulpt-config/widgets/package_json_none"),
            name: "package_json_none".to_string(),
            entry: "index.jsx".to_string(),
            ignore: false,
            dependencies: HashMap::new(),
        }),
    )]
    #[case::package_json_no_deps(
        fixture_path("deskulpt-config/widgets/package_json_no_deps"),
        Some(WidgetConfig {
            directory: fixture_path("deskulpt-config/widgets/package_json_no_deps"),
            name: "package_json_no_deps".to_string(),
            entry: "index.jsx".to_string(),
            ignore: false,
            dependencies: HashMap::new(),
        }),
    )]
    #[case::not_a_widget(fixture_path("deskulpt-config/widgets/not_a_widget"), None)]
    #[case::ignored_widget(fixture_path("deskulpt-config/widgets/ignored_widget"), None)]
    fn test_widget_config_read_from_ok(
        #[case] path: PathBuf,
        #[case] expected_config: Option<WidgetConfig>,
    ) {
        let result = WidgetConfig::try_read(&path)
            .expect("Expected successful read of widget configuration");
        assert_eq!(result, expected_config);
    }

    #[rstest]
    // Input path is not absolute
    #[case::not_absolute(
        "deskulpt-config/widgets/not_absolute",
        vec![ChainReason::Exact(
            "Absolute path to an existing directory is expected; got: \
            deskulpt-config/widgets/not_absolute".to_string()
        )],
    )]
    // Input path is not a directory
    #[case::not_dir(
        fixture_path("deskulpt-config/widgets/not_a_directory"),
        vec![ChainReason::Exact(format!(
            "Absolute path to an existing directory is expected; got: {}",
            fixture_path("deskulpt-config/widgets/not_a_directory").display(),
        ))],
    )]
    // Input path does not exist
    #[case::non_existent(
        fixture_path("deskulpt-config/widgets/non_existent"),
        vec![ChainReason::Exact(format!(
            "Absolute path to an existing directory is expected; got: {}",
            fixture_path("deskulpt-config/widgets/non_existent").display(),
        ))],
    )]
    // `deskulpt.conf.json` is not readable (is a directory)
    #[case::conf_not_readable(
        fixture_path("deskulpt-config/widgets/conf_not_readable"),
        vec![
            ChainReason::Exact("Failed to read deskulpt.conf.json".to_string()),
            ChainReason::IOError,
        ],
    )]
    // `deskulpt.conf.json` is missing a field
    #[case::conf_missing_field(
        fixture_path("deskulpt-config/widgets/conf_missing_field"),
        vec![
            ChainReason::Exact("Failed to interpret deskulpt.conf.json".to_string()),
            ChainReason::SerdeError,
        ],
    )]
    // `package.json` is not readable (is a directory)
    #[case::package_json_not_readable(
        fixture_path("deskulpt-config/widgets/package_json_not_readable"),
        vec![
            ChainReason::Exact("Failed to read package.json".to_string()),
            ChainReason::IOError,
        ],
    )]
    fn test_widget_config_read_from_err(
        #[case] path: PathBuf,
        #[case] expected_error: Vec<ChainReason>,
    ) {
        let error = WidgetConfig::try_read(&path)
            .expect_err("Expected an error reading widget configuration");
        assert_err_eq(error, expected_error);
    }
}
