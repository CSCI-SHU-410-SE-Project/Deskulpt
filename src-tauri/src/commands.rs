//! The module provides the commands used internally by Deskulpt.

use std::{collections::HashMap, fs::read_dir};
use tauri::{api, command, AppHandle, Manager};

use anyhow::{Context, Error};

use crate::{
    bundler::bundle,
    config::{read_widget_config, WidgetConfig},
    states::{WidgetBaseDirectoryState, WidgetCollectionState},
};

/// Alias for `Result<T, String>`.
///
/// This is the type to use for the return value of Tauri commands in the project.
pub(crate) type CommandOut<T> = Result<T, String>;

/// Stringify an [`Error`].
///
/// This is a similar representation to that one gets by default if returning an error
/// from `fn main`, except that it never includes the backtrace to not be too verbose.
pub(crate) fn stringify_anyhow(err: Error) -> String {
    err.chain()
        .enumerate()
        .map(|(index, reason)| match index {
            0 => reason.to_string(),
            1 => format!("\nCaused by:\n  1: {reason}"),
            _ => format!("  {index}: {reason}"),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Get a formatted error string.
///
/// It accepts any arguments that can be passed to [`anyhow::anyhow`].
#[macro_export]
macro_rules! cmderr {
    ($msg:literal $(,)?) => {
        $crate::commands::stringify_anyhow(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        $crate::commands::stringify_anyhow(anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::commands::stringify_anyhow(anyhow::anyhow!($fmt, $($arg)*))
    };
}

/// Return early a formatted error string.
///
/// This is equivalent to `return Err(cmderr!($args...))`.
#[macro_export]
macro_rules! cmdbail {
    ($msg:literal $(,)?) => {
        return Err(cmderr!($msg))
    };
    ($err:expr $(,)?) => {{
        return Err(cmderr!($err))
    }};
    ($fmt:expr, $($arg:tt)*) => {
        return Err(cmderr!($fmt, $($arg)*))
    };
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
        Err(e) => cmdbail!(e),
    };

    for entry in entries {
        // There could be intermittent IO errors during iteration
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => cmdbail!(e),
        };

        let path = entry.path();
        if !path.is_dir() {
            continue; // Non-directory entries are not widgets, skip
        }

        // Load the widget configuration and raise on error
        let widget_config = match read_widget_config(&path) {
            Ok(widget_config) => widget_config,
            Err(e) => cmdbail!(e),
        };

        // Widget configuration being `None` means that the directory is not a widget
        // that is meant to be rendered
        if let Some(widget_config) = widget_config {
            let widget_id = match path.file_name() {
                Some(file_name) => file_name.to_string_lossy().to_string(),
                None => cmdbail!("Failed to get file name"),
            };

            // All checks passed, insert into the new widget collection
            new_widget_collection.insert(widget_id, widget_config);
        }
    }

    // Update the widget collection state
    let widget_collection = app_handle.state::<WidgetCollectionState>();
    *widget_collection.0.lock().unwrap() = new_widget_collection.clone();
    Ok(new_widget_collection)
}

/// Command for bundling the specified widget.
///
/// The widget configuration will be obtained by searching the managed widget collection
/// for the given widget ID. The widget will be bundled into a string of ESM code if the
/// ID is found in the collection.
///
/// The command also requires the URL of the APIs blob of the widget. This is used for
/// replacing the imports of `@deskulpt-test/apis` by the actual URL to import from.
///
/// This command will return the `Failure` variant if:
///
/// - The widget ID is not found in the state of the widget collection.
/// - There is an error when bundling the widget.
#[command]
pub(crate) fn bundle_widget(
    app_handle: AppHandle,
    widget_id: String,
    apis_blob_url: String,
) -> CommandOut<String> {
    let widget_collection_state = &app_handle.state::<WidgetCollectionState>();
    let widget_collection = widget_collection_state.0.lock().unwrap();

    if let Some(widget_config) = widget_collection.get(&widget_id) {
        // Obtain the absolute path of the widget entry point
        let widget_entry = &widget_config.directory.join(&widget_config.deskulpt.entry);

        // Wrap the bundled code if success, otherwise let the error propagate
        return bundle(
            &widget_config.directory,
            widget_entry,
            apis_blob_url,
            widget_config.node.as_ref().map(|package_json| &package_json.dependencies),
        )
        .context(format!("Failed to bundle widget (id={})", widget_id))
        .map_err(|e| cmderr!(e));
    }

    // Error out if the widget ID is not found in the collection
    cmdbail!("Widget '{widget_id}' is not found in the collection")
}

/// Command for opening the widget base directory.
///
/// This command will return the `Failure` variant if Tauri fails to open the widget
/// base directory, most likely because a misconfigured allow list.
#[command]
pub(crate) fn open_widget_base(app_handle: AppHandle) -> CommandOut<()> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;

    api::shell::open(&app_handle.shell_scope(), widget_base.to_string_lossy(), None)
        .map_err(|e| cmderr!(e))
}
