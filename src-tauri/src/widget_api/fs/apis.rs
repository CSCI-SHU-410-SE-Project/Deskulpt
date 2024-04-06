//! This module contains the apis for restricted file system access by the widget.
//! A widget can only read/write/create/delete folders and files in $APPDATA/widgets/<widget_id>/

use crate::states::WidgetBaseDirectoryState;
use tauri::{command, AppHandle, Manager};

#[command]
pub fn read_file(app_handle: AppHandle, file_path: String) -> Result<String, String> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    match std::fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.to_string()),
    }
}
