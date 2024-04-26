//! The module provides the types and initial values for state management in Tauri.
//!
//! In Tauri, different states are distinguished by their unique types, thus we always
//! use structs to mark the states.

use crate::config::WidgetConfigCollection;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, File},
    path::PathBuf,
    sync::Mutex,
};
use tauri::{api, Config};

/// The type for the state of the collection of widget configurations.
///
/// The managed state will by updated at runtime and is thus protected by a mutex. It
/// can be accessed whenever one has access to an app handle:
///
/// ```ignore
/// let widget_collection_state = app.state::<WidgetConfigCollectionState>();
/// let widget_collection = widget_collection_state.0.lock().unwrap();
/// ```
#[derive(Default)]
pub(crate) struct WidgetConfigCollectionState(pub(crate) Mutex<WidgetConfigCollection>);

/// The type for the state of the widget base directory.
///
/// This contains the path to the base directory `$APPDATA/widgets/` where all widgets
/// should be locally stored. This state is static and should not be changed during the
/// runtime. It be accessed whenever one has access to an app handle:
///
/// ```ignore
/// let widget_base_directory = &app.state::<WidgetBaseDirectoryState>().0;
/// ```
pub(crate) struct WidgetBaseDirectoryState(pub(crate) PathBuf);

impl WidgetBaseDirectoryState {
    /// Initialize the widget base directory state according to Tauri configuration.
    pub(crate) fn init(config: &Config) -> Self {
        let app_data_dir = api::path::app_data_dir(config).unwrap();
        let widget_base_dir = app_data_dir.join("widgets");
        if !widget_base_dir.exists() {
            create_dir_all(&widget_base_dir).unwrap();
        }
        Self(widget_base_dir)
    }
}

/// The internals of widgets.
///
/// The internals of widgets refer to configurations that are not controlled by the
/// configuration file but rather controlled by the frontend. They should initially be
/// loaded from a `.deskulpt.json` file on app startup, managed by the frontend during
/// the runtime of the app, and saved back to the file before app shutdown.
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct WidgetInternal {
    x: i32,
    y: i32,
}

/// The type for storing the internals of widgets.
///
/// This contains a mapping from widget IDs to their corresponding internals. Though
/// widget internals keep changing during runtime, the changing state should be tracked
/// only in the frontend and the backend is only responsible for retrieving and storing
/// memoized internals on shutdown. This state is thus static and should not be changed
/// during runtime of the application. It can be accessed whenever one has access to an
/// app handle:
///
/// ```ignore
/// let widget_internals = &app.state::<WidgetInternalsState>().0;
/// ```
pub(crate) struct WidgetInternalsState(pub(crate) HashMap<String, WidgetInternal>);

impl WidgetInternalsState {
    /// Initialize the widget internals state.
    ///
    /// The internals are read from the `$APPCONFIG/.deskulpt.json` written on app
    /// shutdown. If the file does not exist, it means that we failed to memoize
    /// previous internals or it is the first time the app is running, then we return
    /// an empty hash map and create the file.
    pub(crate) fn init(config: &Config) -> Self {
        let app_config_dir = api::path::app_config_dir(config).unwrap();
        let internals_path = app_config_dir.join(".deskulpt.json");
        if !internals_path.exists() {
            let _ = File::create(&internals_path);
            return Self(Default::default());
        }

        // Read the internals
        let internals = match read_to_string(&internals_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Default::default(),
        };
        Self(internals)
    }
}
