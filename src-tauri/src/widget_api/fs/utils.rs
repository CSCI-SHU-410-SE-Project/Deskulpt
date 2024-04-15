use crate::states::WidgetBaseDirectoryState;
use anyhow::{bail, Context, Error};
use path_clean::PathClean;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

// TODO: Write auto-generated unittests to cover more corner cases

/// Validate if the widget ID corresponds to a direct folder in the widget base folder。
///
/// Note that if the folder does not exist, the function will return an error, since
/// this likely indicates a mistake in the widget ID.
pub(crate) fn validate_widget_id<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
) -> Result<(), Error> {
    let widget_base = get_widget_base(app_handle);
    let widget_dir = get_widget_dir(app_handle, widget_id);

    let widget_base_clean = widget_base.clean();
    let widget_dir_clean = widget_dir.clean();

    // If the following conditions are not met, the widget ID is invalid:
    // - the $widget_base/$widget_id exists
    // - the $widget_base/$widget_id is a directory
    // - the $widget_base/$widget_id is a **direct** subdirectory of $widget_base
    if !widget_dir_clean.is_dir() || !widget_dir_clean.starts_with(widget_base_clean) {
        bail!(
            "Invalid widget ID: '{}'. Widget ID must correspond to a folder in the \
            widget base directory.",
            widget_id
        );
    }

    Ok(())
}

/// Validate the file system resource (file or folder) path.
///
/// In particular, this function checks if the widget ID corresponds to a direct
/// subfolder of $widget_base.
///
/// Note that this function does not check if the resource exists or not, unlike
/// `validate_widget_id` which requires the widget folder to exist to be valid. This is
/// because the file or folder may not exist yet, and will be created later.
pub(crate) fn validate_resource_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> Result<(), Error> {
    // Validate if the widget ID is a direct subfolder of the widget_base folder
    validate_widget_id(app_handle, widget_id)
        .context(format!("Failed to validate widget ID: '{}'", widget_id))?;

    let widget_dir = get_widget_dir(app_handle, widget_id);
    let resource_path = widget_dir.join(path).clean();

    // Validate if the file is within the widget directory
    if !resource_path.starts_with(&widget_dir) {
        bail!(
            "Invalid resource path: '{}'. Resource must be within the widget directory",
            path
        );
    }

    Ok(())
}

/// Get the widget base directory.
pub(crate) fn get_widget_base<R: Runtime>(app_handle: &AppHandle<R>) -> PathBuf {
    app_handle.state::<WidgetBaseDirectoryState>().0.clone()
}

/// Get the widget directory by its widget ID.
pub(crate) fn get_widget_dir<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
) -> PathBuf {
    get_widget_base(app_handle).join(widget_id)
}

// Get the resource path in the widget directory with given ID.
pub(crate) fn get_resource_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> PathBuf {
    get_widget_dir(app_handle, widget_id).join(path)
}
