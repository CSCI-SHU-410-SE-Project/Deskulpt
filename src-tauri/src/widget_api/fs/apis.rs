//! This module contains the apis for restricted file system access by the widget.
//! A widget can only read/write/create/delete folders and files in $APPDATA/widgets/<widget_id>/

use crate::states::WidgetBaseDirectoryState;
use std::io::Write;
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

// Write content to the file at path $APPDATA/widgets/<widget_id>/storage/<path>
#[command]
pub fn write_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), String> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    let widget_dir = widget_base.join(widget_id);
    let widget_storage_dir = widget_dir.join("storage");
    let file_path = widget_storage_dir.join(path);
    match std::fs::write(file_path, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

// Append content to the file at path $APPDATA/widgets/<widget_id>/storage/<path>
#[command]
pub fn append_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), String> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    let widget_dir = widget_base.join(widget_id);
    let widget_storage_dir = widget_dir.join("storage");
    let file_path = widget_storage_dir.join(path);
    match std::fs::OpenOptions::new().append(true).create(true).open(file_path) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

// Delete the file at path $APPDATA/widgets/<widget_id>/storage/<path>
#[command]
pub fn remove_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), String> {
    let widget_base = &app_handle.state::<WidgetBaseDirectoryState>().0;
    let widget_dir = widget_base.join(widget_id);
    let widget_storage_dir = widget_dir.join("storage");
    let file_path = widget_storage_dir.join(path);
    match std::fs::remove_file(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
