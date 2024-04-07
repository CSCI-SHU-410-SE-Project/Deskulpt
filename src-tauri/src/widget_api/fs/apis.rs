//! This module contains the apis for restricted file system access by the widget.
//! A widget can only read/write/create/delete folders and files in $APPDATA/widgets/<widget_id>/

use crate::states::WidgetBaseDirectoryState;
use tauri::{command, AppHandle, Manager, Runtime};

// Read the file at path $APPDATA/widgets/<widget_id>/storage/<path>
#[command]
pub fn read_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<String, String> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    let widget_dir = widget_base.join(widget_id);
    let widget_storage_dir = widget_dir.join("storage");
    let file_path = widget_storage_dir.join(path);
    match std::fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.to_string()),
    }
}
