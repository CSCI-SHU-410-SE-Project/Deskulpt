use std::collections::HashMap;
use std::fs::read_dir;

use tauri::{command, AppHandle, Runtime};

use super::error::{cmdbail, CmdResult};
use crate::config::{WidgetCollection, WidgetConfig};
use crate::path::PathExt;
use crate::states::StatesExtWidgetCollection;

/// Rescan the widgets directory and update the widget collection.
///
/// This will update the widget collection state and return the updated
/// collection as well.
///
/// ### Errors
///
/// - Error traversing the widgets directory.
/// - Error inferring widget ID from the directory entry.
#[command]
pub async fn rescan_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<WidgetCollection> {
    let widgets_dir = app_handle.widgets_dir();
    let mut new_widget_collection = HashMap::new();

    let entries = read_dir(widgets_dir)?;
    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        if !path.is_dir() {
            continue; // Non-directory entries are not widgets, skip
        }

        let widget_id = match path.file_name() {
            Some(file_name) => file_name.to_string_lossy().to_string(),
            None => cmdbail!("Invalid widget directory: '{}'", path.display()),
        };

        // Load the widget configuration
        match WidgetConfig::load(&path) {
            Ok(Some(widget_config)) => {
                new_widget_collection.insert(widget_id, Ok(widget_config));
            },
            Ok(None) => {},
            Err(e) => {
                // Configuration errors are recorded as error messages and do
                // not fail the command
                new_widget_collection.insert(widget_id, Err(e.to_string()));
            },
        };
    }

    // Update the widget collection state
    app_handle.with_widget_collection_mut(|collection| {
        collection.clone_from(&new_widget_collection);
    });
    Ok(new_widget_collection)
}
