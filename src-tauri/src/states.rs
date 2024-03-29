//! The module provides the types for state management in Tauri.
//!
//! In Tauri, different states are distinguished by their unique types, thus we always
//! use structs to mark the states.

use crate::config::WidgetConfig;

use std::{collections::HashMap, path::PathBuf, sync::Mutex};

/// The type for the state of the collection of widgets.
///
/// This contains a mapping from widget IDs to their corresponding configurations. The
/// managed state will by updated dynamically and is thus protected by a mutex.
///
/// The state can be accessed whenever one has access to an app handle:
///
/// ```ignore
/// let widget_collection_state = app.state::<WidgetCollectionState>();
/// let widget_collection = widget_collection_state.0.lock().unwrap();
/// ```
#[derive(Default)]
pub(crate) struct WidgetCollectionState(
    pub(crate) Mutex<HashMap<String, WidgetConfig>>,
);

/// The type for the state of the widget base directory.
///
/// This contains the path to the base directory `$APPDATA/widgets/` where all widgets
/// should be locally stored. This state is static and should not be changed during the
/// lifetime of the application.
///
/// The state can be accessed whenever one has access to an app handle:
///
/// ```ignore
/// let widget_base_directory = &app.state::<WidgetBaseDirectoryState>().0;
/// ```
pub(crate) struct WidgetBaseDirectoryState(pub(crate) PathBuf);
