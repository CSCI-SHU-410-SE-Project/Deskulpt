//! The module provides the commands used internally by Deskulpt.

use serde::Serialize;
use std::{collections::HashMap, fmt::Debug, fs::read_dir};
use tauri::{api::shell, command, AppHandle, Manager};

use anyhow::Context;

use crate::{
    bundler::bundle,
    config::{read_widget_config, WidgetConfig},
    states::{WidgetBaseDirectoryState, WidgetCollectionState},
};

/// The output of a Tauri command.
#[derive(Serialize)]
pub(crate) enum CommandOut<T> {
    /// Indicates that the command has succeeded, containing the output.
    #[serde(rename = "success")]
    Success(T),
    /// Indicates that the command has failed, containing the error message.
    #[serde(rename = "failure")]
    Failure(String),
}

impl<T> CommandOut<T> {
    pub(crate) fn fail<E: Debug>(err: E) -> Self {
        CommandOut::Failure(format!("{:#?}", err))
    }
}

/// Command for refreshing the state of the widget collection.
///
/// This command will scan through the widget base directory and update the current
/// widget collection state with the new widgets found. It will also return the updated
/// widget collection, intended to be used by the frontend to refresh the rendering of
/// the widgets.
///
/// This command will break early and return the `Failure` variant if:
///
/// - There is an error when reading the widget base directory.
/// - There is an error when loading the widget configuration.
/// - There is an error implying the widget ID from its path.
#[command]
pub(crate) fn refresh_widget_collection(
    app_handle: AppHandle,
) -> CommandOut<HashMap<String, WidgetConfig>> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    let mut new_widget_collection = HashMap::new();

    let entries = match read_dir(widget_base) {
        Ok(entries) => entries,
        Err(e) => return CommandOut::fail(e),
    };

    for entry in entries {
        // There could be intermittent IO errors during iteration
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return CommandOut::fail(e),
        };

        let path = entry.path();
        if !path.is_dir() {
            continue; // Non-directory entries are not widgets, skip
        }

        // Load the widget configuration and raise on error
        let widget_config = match read_widget_config(&path) {
            Ok(widget_config) => widget_config,
            Err(e) => return CommandOut::fail(e),
        };

        // Widget configuration being `None` means that the directory is not considered
        // a widget; thus it should be silently excluded from the collection instead of
        // returning a failure that blocks other widgets from rendering
        if let Some(widget_config) = widget_config {
            let widget_id = match path.file_name() {
                Some(file_name) => file_name.to_string_lossy().to_string(),
                None => return CommandOut::fail("Failed to get file name"),
            };

            // All checks passed, insert into the new widget collection
            new_widget_collection.insert(widget_id, widget_config);
        }
    }

    // Update the widget collection state
    let widget_collection = app_handle.state::<WidgetCollectionState>();
    *widget_collection.0.lock().unwrap() = new_widget_collection.clone();
    CommandOut::Success(new_widget_collection)
}

/// Command for bundling the specified widget.
///
/// The widget configuration will be obtained by searching the managed widget collection
/// for the given widget ID. The widget will be bundled into a string of ESM code if the
/// ID is found in the collection.
///
/// This command will return the `Failure` variant if:
///
/// - The widget ID is not found in the state of the widget collection.
/// - There is an error when bundling the widget.
#[command]
pub(crate) fn bundle_widget(
    app_handle: AppHandle,
    widget_id: String,
) -> CommandOut<String> {
    let widget_collection_state = &app_handle.state::<WidgetCollectionState>();
    let widget_collection = widget_collection_state.0.lock().unwrap();

    if let Some(widget_config) = widget_collection.get(&widget_id) {
        // Obtain the absolute path of the widget entry point
        let widget_entry = &widget_config.directory.join(&widget_config.deskulpt.entry);

        // Wrap the bundled code if success, otherwise let the error propagate
        match bundle(
            widget_entry,
            widget_config.node.as_ref().map(|package_json| &package_json.dependencies),
        )
        .context(format!("Failed to bundle widget (id={})", widget_id))
        {
            Ok(bundled_code) => return CommandOut::Success(bundled_code),
            Err(e) => return CommandOut::fail(e),
        }
    }

    // Error out if the widget ID is not found in the collection
    CommandOut::fail(format!(
        "Failed to bundle widget (id={}) because it is not found in the collection",
        widget_id
    ))
}

/// Command for opening the widget base directory.
///
/// This command will return the `Failure` variant if Tauri fails to open the widget
/// base directory, most likely because a misconfigured allow list.
#[command]
pub(crate) fn open_widget_base(app_handle: AppHandle) -> CommandOut<()> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;

    match shell::open(&app_handle.shell_scope(), widget_base.to_string_lossy(), None) {
        Ok(_) => CommandOut::Success(()),
        Err(e) => CommandOut::fail(e),
    }
}
