//! The module provides the commands used internally by Deskulpt.

use crate::{
    bundler::bundle,
    config::{read_widget_config, WidgetConfigCollection},
    settings::{read_settings, write_settings, Settings},
    states::{WidgetBaseDirectoryState, WidgetConfigCollectionState},
    utils::toggle_click_through_state,
};
use anyhow::{Context, Error};
use std::{collections::HashMap, fs::read_dir};
use tauri::{command, AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_shell::ShellExt;

/// The return type of all Tauri commands in Deskulpt.
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
    widget_collection.0.lock().unwrap().clone_from(&new_widget_collection);
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
            &widget_config.external_deps,
        )
        .context(format!("Failed to bundle widget (id={})", widget_id))
        .map_err(|e| cmderr!(e));
    }

    // Error out if the widget ID is not found in the collection
    cmdbail!("Widget '{widget_id}' is not found in the collection")
}

/// Register or unregister a global shortcut for toggling the click-through state.
///
/// If `reverse` this will register the shortcut, otherwise it will unregister it. This
/// command will fail if the shortcut (un)registration fails, but any error in the
/// registered callback will be ignored, i.e., it is not guaranteed that the toggling
/// will succeed on the shortcut.
#[command]
pub(crate) fn register_toggle_shortcut(
    app_handle: AppHandle,
    shortcut: String,
    reverse: bool,
) -> CommandOut<()> {
    if reverse {
        app_handle
            .global_shortcut()
            .unregister(shortcut.as_str())
            .map_err(|e| cmderr!(e))
    } else {
        app_handle
            .global_shortcut()
            .on_shortcut(shortcut.as_str(), |inner_app_handle, _, event| {
                if event.state == ShortcutState::Pressed {
                    // We must only react to press events, otherwise we would toggle
                    // again on release; also consume errors because they are not
                    // allowed to propagate
                    let _ = toggle_click_through_state(inner_app_handle);
                }
            })
            .map_err(|e| cmderr!(e))
    }
}

/// Command for opening a widget directory or the widget base directory.
///
/// If the widget ID is `None`, this command will open the widget base directory.
/// Otherwise, it checks whether the widget ID exists in the current widget collection
/// and opens the corresponding widget directory.
///
/// This command will fail if:
///
/// - The given widget ID is not found in the widget collection.
/// - Tauri fails to open the widget base directory, most likely due to misconfigured
///   capabiblities.
#[command]
pub(crate) fn open_widget_directory(
    app_handle: AppHandle,
    widget_id: Option<String>,
) -> CommandOut<()> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;

    let open_path = match widget_id {
        Some(widget_id) => {
            let widget_collection = app_handle.state::<WidgetConfigCollectionState>();
            if !widget_collection.0.lock().unwrap().contains_key(&widget_id) {
                cmdbail!("Widget '{}' is not found in the collection", widget_id)
            }
            widget_base.join(widget_id)
        },
        None => widget_base.to_path_buf(),
    };

    app_handle.shell().open(open_path.to_string_lossy(), None).map_err(|e| cmderr!(e))
}

/// Command for initializing the settings.
///
/// This command tries to load the previously stored settings. It never fails, but
/// instead returns the default settings upon any error.
#[command]
pub(crate) fn init_settings(app_handle: AppHandle) -> CommandOut<Settings> {
    let app_config_dir = match app_handle.path().app_config_dir() {
        Ok(app_config_dir) => app_config_dir,
        Err(_) => return Ok(Default::default()),
    };
    Ok(read_settings(&app_config_dir))
}

/// Command for cleaning up and exiting the application.
///
/// This command will try to save the widget internals for persistence before exiting
/// the application, but failure to do so will not prevent the application from exiting.
#[command]
pub(crate) fn exit_app(app_handle: AppHandle, settings: Settings) -> CommandOut<()> {
    let app_config_dir = match app_handle.path().app_config_dir() {
        Ok(app_config_dir) => app_config_dir,
        Err(_) => {
            app_handle.exit(0);
            return Ok(());
        },
    };

    let _ = write_settings(&app_config_dir, &settings);
    app_handle.exit(0);
    Ok(())
}
