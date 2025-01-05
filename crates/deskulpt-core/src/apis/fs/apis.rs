//! Commands for the `fs` plugin.

use std::io::Write;

use anyhow::Context;
use tauri::{command, AppHandle, Runtime};

use super::utils::get_resource_path;

#[command]
pub async fn exists<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    Ok(file_path.exists())
}

#[command]
pub async fn is_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    Ok(file_path.is_file())
}

#[command]
pub async fn is_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    Ok(file_path.is_dir())
}

#[command]
pub async fn read_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<String> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::read_to_string(&file_path)
        .context(format!("Failed to read file '{}'", file_path.display()))
        .map_err(|e| cmderr!(e))
}

#[command]
pub async fn write_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> CommandOut<()> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::write(&file_path, content)
        .context(format!("Failed to write file '{}'", file_path.display()))
        .map_err(|e| cmderr!(e))
}

#[command]
pub async fn append_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> CommandOut<()> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .context(format!("Failed to append file '{}'", file_path.display()))
        .map_err(|e| cmderr!(e))
}

#[command]
pub async fn remove_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<()> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::remove_file(&file_path)
        .context(format!("Failed to delete file '{}'", file_path.display()))
        .map_err(|e| cmderr!(e))
}

#[command]
pub async fn create_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<()> {
    let folder_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::create_dir_all(&folder_path)
        .context(format!(
            "Failed to create directory '{}'",
            folder_path.display()
        ))
        .map_err(|e| cmderr!(e))
}

#[command]
pub async fn remove_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<()> {
    let folder_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::remove_dir_all(&folder_path)
        .context(format!(
            "Failed to delete directory '{}'",
            folder_path.display()
        ))
        .map_err(|e| cmderr!(e))
}
