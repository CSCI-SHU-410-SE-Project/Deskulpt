//! The module provides the types for state management in Tauri.
//!
//! In Tauri, different states are distinguished by their unique types, thus we always
//! use structs to mark the states.

use crate::config::WidgetConfigCollection;
use std::{path::PathBuf, sync::Mutex};

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
