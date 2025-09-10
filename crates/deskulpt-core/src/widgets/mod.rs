mod config;

use std::path::Path;

use anyhow::Context;
use config::{DeskulptConf, LoadFromConfigFile, Package};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ValidWidget {
    /// The directory name of the widget.
    pub dir: String,
    /// The required `deskulpt.conf.json` configuration.
    #[serde(flatten)]
    pub deskulpt_conf: DeskulptConf,
    /// The optional `package.json` configuration.
    #[serde(flatten)]
    pub package: Package,
}

#[derive(Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct InvalidWidget {
    /// The directory name of the widget.
    pub dir: String,
    /// The error message.
    pub error: String,
}

#[derive(Clone, Serialize, specta::Type)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Widget {
    /// A valid widget.
    Valid(ValidWidget),
    /// An invalid widget.
    Invalid(InvalidWidget),
}

impl Widget {
    /// Load widget from a directory.
    ///
    /// If the directory is not considered a widget directory or if the widget
    /// is explicitly marked as ignored, this method returns `None`. If there is
    /// any error loading the widget, it returns an [`InvalidWidget`] variant.
    /// Otherwise, it returns a [`ValidWidget`] variant.
    pub fn load<P: AsRef<Path>>(dir: P) -> Option<Self> {
        let dir = dir.as_ref();
        let dir_name = dir.file_name()?.to_string_lossy();

        let deskulpt_conf = match DeskulptConf::load(dir)
            .context(format!("Failed to load {}", DeskulptConf::CONFIG_FILE_NAME))
        {
            Ok(Some(deskulpt_conf)) => deskulpt_conf,
            Ok(None) => return None,
            Err(e) => {
                return Some(Widget::Invalid(InvalidWidget {
                    dir: dir_name.to_string(),
                    error: format!("{e:?}"),
                }));
            },
        };

        // Ignore widgets that are explcitly marked as such
        if deskulpt_conf.ignore {
            return None;
        }

        let package = match Package::load(dir)
            .context(format!("Failed to load {}", Package::CONFIG_FILE_NAME))
        {
            Ok(package) => package.unwrap_or_default(),
            Err(e) => {
                return Some(Widget::Invalid(InvalidWidget {
                    dir: dir_name.to_string(),
                    error: format!("{e:?}"),
                }));
            },
        };

        Some(Widget::Valid(ValidWidget {
            dir: dir_name.to_string(),
            deskulpt_conf,
            package,
        }))
    }

    /// Get the directory of the widget inside the widgets directory.
    pub fn dir(&self) -> &str {
        match self {
            Widget::Valid(valid) => &valid.dir,
            Widget::Invalid(invalid) => &invalid.dir,
        }
    }

    /// Get the widget ID.
    ///
    /// This ID is derived from the widget directory name using UUID v5. It is
    /// deterministic for the same directory name.
    pub fn id(&self) -> String {
        let dir_encoded = self.dir().as_bytes();
        Uuid::new_v5(&Uuid::NAMESPACE_URL, dir_encoded).to_string()
    }
}
