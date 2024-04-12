use crate::widget_api::fs::utils;
use anyhow::Context;
use std::io::Write;
use tauri::{command, AppHandle, InvokeError, Runtime};

// TODO: Write formatted string to files (now there is no way to break new lines)
// TODO: Write auto-generated unit tests to cover more corner cases

#[command]
pub(crate) fn exists<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<bool, InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    Ok(file_path.exists())
}

#[command]
pub(crate) fn is_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<bool, InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    Ok(file_path.is_file())
}

#[command]
pub(crate) fn is_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<bool, InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    Ok(file_path.is_dir())
}

#[command]
pub(crate) fn read_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<String, InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    if !file_path.is_file() {
        return Err(InvokeError::from(format!(
            "Path '{}' is not a file",
            file_path.display()
        )));
    }
    std::fs::read_to_string(&file_path)
        .context(format!("Failed to read file '{}'", file_path.display()))
        .map_err(InvokeError::from_anyhow)
}

#[command]
pub(crate) fn write_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    std::fs::write(&file_path, content)
        .context(format!("Failed to write file '{}'", file_path.display()))
        .map_err(InvokeError::from_anyhow)
}

#[command]
pub(crate) fn append_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .context(format!("Failed to append file '{}'", file_path.display()))
        .map_err(InvokeError::from_anyhow)
}

#[command]
pub(crate) fn remove_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    if !file_path.is_file() {
        return Err(InvokeError::from(format!(
            "Path '{}' is not a file",
            file_path.display()
        )));
    }
    std::fs::remove_file(&file_path)
        .context(format!("Failed to delete file '{}'", file_path.display()))
        .map_err(InvokeError::from_anyhow)
}

#[command]
pub(crate) fn create_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let folder_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    if folder_path.exists() {
        return Err(InvokeError::from(format!(
            "Directory '{}' already exists",
            folder_path.display()
        )));
    }
    std::fs::create_dir_all(&folder_path)
        .context(format!("Failed to create directory '{}'", folder_path.display()))
        .map_err(InvokeError::from_anyhow)
}

#[command]
pub(crate) fn remove_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), InvokeError> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .context("Failed to validate resource path")
        .map_err(InvokeError::from_anyhow)?;
    let folder_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    std::fs::remove_dir_all(&folder_path)
        .context(format!("Failed to delete directory '{}'", folder_path.display()))
        .map_err(InvokeError::from_anyhow)
}
