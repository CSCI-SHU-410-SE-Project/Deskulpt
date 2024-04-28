//! The module provides the types for state management in Tauri.
//!
//! In Tauri, different states are distinguished by their unique types, thus we always
//! use structs to mark the states.

use crate::config::WidgetConfigCollection;
use std::{fs::create_dir_all, path::PathBuf, sync::Mutex};
use tauri::{App, Manager};

/// The type for the state of the collection of widget configurations.
///
/// The managed state will be updated at runtime and is thus protected by a mutex.
#[derive(Default)]
pub(crate) struct WidgetConfigCollectionState(pub(crate) Mutex<WidgetConfigCollection>);

/// The type for the state of the widget base directory.
///
/// This contains the path to the base directory `$APPDATA/widgets/` where all widgets
/// should be locally stored. This state is static and should not be changed during the
/// runtime.
pub(crate) struct WidgetBaseDirectoryState(pub(crate) PathBuf);

impl WidgetBaseDirectoryState {
    /// Initialize the widget base directory state.
    ///
    /// This creates the widget base directory if it does not exist.
    pub(crate) fn init(app: &App) -> Self {
        let app_data_dir = app.path().app_data_dir().unwrap();
        let widget_base_dir = app_data_dir.join("widgets");
        if !widget_base_dir.exists() {
            create_dir_all(&widget_base_dir).unwrap();
        }
        Self(widget_base_dir)
    }
}

/// The type for the state of whether the canvas can be clicked through.
///
/// The managed state will be updated at runtime and is thus protected by a mutex.
pub(crate) struct CanvasClickThroughState(pub(crate) Mutex<bool>);

impl Default for CanvasClickThroughState {
    fn default() -> Self {
        Self(Mutex::new(true)) // Initially click through
    }
}
