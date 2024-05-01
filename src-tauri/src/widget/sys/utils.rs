//! This module contains the utilities for `fs` in `@deskulpt-test/apis`.

use crate::states::WidgetBaseDirectoryState;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

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
