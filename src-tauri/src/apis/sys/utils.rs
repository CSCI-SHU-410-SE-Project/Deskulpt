//! This module contains the utilities for `fs` in `@deskulpt-test/apis`.

use crate::states::WidgetBaseDirectoryState;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

/// Get the widget base directory.
pub(crate) fn get_widget_base<R: Runtime>(app_handle: &AppHandle<R>) -> PathBuf {
    app_handle.state::<WidgetBaseDirectoryState>().0.clone()
}
