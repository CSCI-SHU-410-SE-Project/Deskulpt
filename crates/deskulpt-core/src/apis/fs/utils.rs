//! Utilities for the `fs` plugin.

use std::path::PathBuf;

use deskulpt_utils::{cmdbail, PathExt};
use path_clean::PathClean;
use tauri::{AppHandle, Runtime};

/// Validate the file system resource (file or folder) path.
///
/// This raises an error if the widget ID is invalid, in cases where:
///
/// - $widgets_dir/$widget_id does not exist
/// - $widgets_dir/$widget_id is not a directory
/// - $widgets_dir/$widget_id is not a direct subdirectory of $widgets_dir
///
/// or if the resource path is invalid, in cases where:
///
/// - $widgets_dir/$widget_id/$path is not within the widget directory
///
/// Note, however, that this function does not check if the resource exists or
/// not, since the file or folder may not exist yet, and could be created later.
pub fn get_resource_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> Result<PathBuf, String> {
    let widgets_dir = app_handle.widgets_dir();
    let widget_dir = widgets_dir.join(widget_id).clean();

    if !widget_dir.is_dir() || !widget_dir.starts_with(&widgets_dir) || {
        // Count the number of path components to make sure that the widget directory
        // is a direct descendant of the widget base directory
        let base_component_count = widgets_dir.iter().count();
        let dir_component_count = widget_dir.iter().count();
        base_component_count + 1 != dir_component_count
    } {
        cmdbail!(
            "Invalid widget ID: '{widget_id}'; widget ID must correspond to a direct subdirectory \
             of the widget base directory."
        );
    }

    let resource_path = widget_dir.join(path).clean();
    if !resource_path.starts_with(&widget_dir) {
        cmdbail!(
            "Invalid resource path: '{path}'; resource path must stay within its corresponding \
             widget directory."
        );
    }

    Ok(resource_path)
}
