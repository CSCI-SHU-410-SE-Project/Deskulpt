//! The module provides the commands used internally by Deskulpt.

use crate::{
    bundler::bundle,
    config::{read_widget_config, WidgetConfigCollection},
    states::{WidgetBaseDirectoryState, WidgetConfigCollectionState},
};
use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string, File},
    io::BufWriter,
};
use tauri::{api, command, AppHandle, Manager};

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
/// This command will fail if:
///
/// - There is an error reading the widget base directory.
/// - There is an error getting some entry in the widget base directory.
/// - There is an error inferring the widget ID from the path of the entry.
///
/// Note that failure to load a widget configuration will not lead to an overall failure
/// of the command. Instead, the widget ID will correspond to an error message instead
/// of a widget configuration.
#[command]
pub(crate) fn refresh_widget_collection(
    app_handle: AppHandle,
) -> CommandOut<WidgetConfigCollection> {
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
        let widget_id = match path.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => cmdbail!("Cannot infer widget ID from '{}'", path.display()),
        };

        // Load the widget configuration and raise on error
        let widget_config = match read_widget_config(&path) {
            Ok(widget_config) => widget_config,
            Err(e) => {
                // We should not fail the whole command if some widget configuration
                // fails to be loaded; instead we record the error corresponding to the
                // widget ID
                new_widget_collection.insert(widget_id, Err(cmderr!(e)));
                continue;
            },
        };

        // Widget configuration being `None` means that the directory is not a widget
        // that is meant to be rendered
        if let Some(widget_config) = widget_config {
            new_widget_collection.insert(widget_id, Ok(widget_config));
        }
    }

    // Update the widget collection state
    let widget_collection = app_handle.state::<WidgetConfigCollectionState>();
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
/// This command will fail if:
///
/// - The widget ID is not found in the state of the widget collection.
/// - The widget collection state corresponding to the widget ID is an error message
///   instead of a widget configuration.
/// - There is an error when bundling the widget.
#[command]
pub(crate) fn bundle_widget(
    app_handle: AppHandle,
    widget_id: String,
    apis_blob_url: String,
) -> CommandOut<String> {
    let widget_collection_state = &app_handle.state::<WidgetConfigCollectionState>();
    let widget_collection = widget_collection_state.0.lock().unwrap();

    if let Some(widget_config) = widget_collection.get(&widget_id) {
        let widget_config = match widget_config.as_ref() {
            Ok(widget_config) => widget_config,
            Err(e) => cmdbail!(e.clone()),
        };
        // Obtain the absolute path of the widget entry point
        let widget_entry =
            &widget_config.directory.join(&widget_config.deskulpt_conf.entry);

        // Wrap the bundled code if success, otherwise let the error propagate
        return bundle(
            &widget_config.directory,
            widget_entry,
            apis_blob_url,
            &widget_config.external_dependencies,
        )
        .context(format!("Failed to bundle widget (id={})", widget_id))
        .map_err(|e| cmderr!(e));
    }

    // Error out if the widget ID is not found in the collection
    cmdbail!("Widget '{widget_id}' is not found in the collection")
}

/// Command for opening the widget base directory.
///
/// This command will fail if Tauri fails to open the widget base directory, most likely
/// because of a misconfigured allow list.
#[command]
pub(crate) fn open_widget_base(app_handle: AppHandle) -> CommandOut<()> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;

    api::shell::open(&app_handle.shell_scope(), widget_base.to_string_lossy(), None)
        .map_err(|e| cmderr!(e))
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

/// Command for initializing the widget internals state.
///
/// This command tries to load the previously stored widget internals located at
/// `$APPCONFIG/.deskulpt.json`. This command never fails, but instead returns an empty
/// widget internals mapping whenever there is an error loading the file.
#[command]
pub(crate) fn init_widget_internals(
    app_handle: AppHandle,
) -> CommandOut<HashMap<String, WidgetInternal>> {
    let app_config_dir = match app_handle.path_resolver().app_config_dir() {
        Some(app_config_dir) => app_config_dir,
        None => return Ok(Default::default()),
    };

    let internals_path = app_config_dir.join(".deskulpt.json");
    if !internals_path.exists() {
        return Ok(Default::default());
    }

    let internals = match read_to_string(&internals_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Default::default(),
    };
    Ok(internals)
}

/// Command for exiting the application.
///
/// This command will write the widget internals as JSON for persistence and then exit
/// the application. This could fail in rare cases but we do not really care.
#[command]
pub(crate) fn exit_app(
    app_handle: AppHandle,
    widget_internals: HashMap<String, WidgetInternal>,
) -> CommandOut<()> {
    let internals_path = match app_handle.path_resolver().app_config_dir() {
        Some(app_config_dir) => app_config_dir.join(".deskulpt.json"),
        None => cmdbail!("Failed to get app config directory"),
    };

    // We do not care about previous internals, so we overwrite the whole file
    let internals_file = File::create(internals_path).map_err(|e| cmderr!(e))?;
    let internals_writer = BufWriter::new(internals_file);
    serde_json::to_writer(internals_writer, &widget_internals)
        .map_err(|e| cmderr!(e))?;

    app_handle.exit(0);
    Ok(())
}
