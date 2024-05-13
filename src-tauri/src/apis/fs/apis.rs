//! This module implements the commands for `fs` in `@deskulpt-test/apis`.

use super::utils::get_resource_path;
use crate::{cmderr, commands::CommandOut};
use anyhow::Context;
use std::io::Write;
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) fn exists<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    Ok(file_path.exists())
}

#[command]
pub(crate) fn is_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    Ok(file_path.is_file())
}

#[command]
pub(crate) fn is_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    let file_path = get_resource_path(&app_handle, &widget_id, &path)?;
    Ok(file_path.is_dir())
}

#[command]
pub(crate) fn read_file<R: Runtime>(
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
pub(crate) fn write_file<R: Runtime>(
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
pub(crate) fn append_file<R: Runtime>(
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
pub(crate) fn remove_file<R: Runtime>(
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
pub(crate) fn create_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<()> {
    let folder_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::create_dir_all(&folder_path)
        .context(format!("Failed to create directory '{}'", folder_path.display()))
        .map_err(|e| cmderr!(e))
}

#[command]
pub(crate) fn remove_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<()> {
    let folder_path = get_resource_path(&app_handle, &widget_id, &path)?;
    std::fs::remove_dir_all(&folder_path)
        .context(format!("Failed to delete directory '{}'", folder_path.display()))
        .map_err(|e| cmderr!(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::setup_mock_env;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::path::{Path, PathBuf};

    /// Set up a widget directory with the given ID.
    fn setup_widget_directory(base_dir: &Path, widget_id: &str) -> PathBuf {
        let widget_dir = base_dir.join("widgets").join(widget_id);
        std::fs::create_dir_all(&widget_dir)
            .expect("Failed to create widget directory");
        widget_dir
    }

    #[rstest]
    fn test_exists() {
        // Test the `fs::exists` command
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");
        let file_path = widget_dir.join("dummy_file.txt");

        // The file does not exist yet and should return false
        let result = exists(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Create the file and should return true
        std::fs::File::create(&file_path).unwrap();
        let result = exists(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[rstest]
    fn test_is_file_or_dir() {
        // Test the `fs::is_file` and `fs::is_dir` commands
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");

        // Create a file and a directory
        let file_path = widget_dir.join("dummy_file.txt");
        let dir_path = widget_dir.join("dummy_dir");
        std::fs::File::create(&file_path).unwrap();
        std::fs::create_dir(&dir_path).unwrap();

        // Check that `is_file` gives true and `is_dir` gives false for the file
        let result = is_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
        let result = is_dir(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Check that `is_file` gives false and `is_dir` gives true for the directory
        let result =
            is_file(app_handle.clone(), "dummy".to_string(), "dummy_dir".to_string());
        assert!(result.is_ok());
        assert!(!result.unwrap());
        let result =
            is_dir(app_handle.clone(), "dummy".to_string(), "dummy_dir".to_string());
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[rstest]
    fn test_read_file() {
        // Test the `fs::read_file` command
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");
        let file_path = widget_dir.join("dummy_file.txt");

        // The file does not exist yet and should return an error
        let result = read_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(
            format!("Failed to read file '{}'", file_path.display()).as_str()
        ));

        // Create the file with some content and should return the content
        let content = "Hello, world!";
        std::fs::File::create(&file_path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
        let result = read_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[rstest]
    fn test_write_file() {
        // Test the `fs::write_file` command
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");
        let file_path = widget_dir.join("dummy_file.txt");

        // Writing to a non-existent path should create the file
        let content = "Hello, world!";
        let result = write_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
            content.to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), content);

        // Writing to an existing file should overwrite the content
        let new_content = "Hello, new world!";
        let result = write_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
            new_content.to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), new_content);
    }

    #[rstest]
    fn test_append_file() {
        // Test the `fs::append_file` command
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");
        let file_path = widget_dir.join("dummy_file.txt");

        // Appending to a non-existent path should create the file
        let content = "Hello, world!\n";
        let result = append_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
            content.to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), content);

        // Appending to an existing file should append the content
        let new_content = "Hello, new world!\n";
        let result = append_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
            new_content.to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(
            std::fs::read_to_string(&file_path).unwrap(),
            format!("{content}{new_content}")
        );
    }

    #[rstest]
    fn test_create_dir() {
        // Test the `fs::create_dir` command
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");

        let result = create_dir(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_dir".to_string(),
        );
        assert!(result.is_ok());
        assert!(widget_dir.join("dummy_dir").is_dir());
    }

    #[rstest]
    fn test_remove_file_or_dir() {
        // Test the `fs::remove_file` and `fs::remove_dir` commands
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");

        // Removing a non-existent file should raise an error
        let result = remove_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(
            format!(
                "Failed to delete file '{}'",
                widget_dir.join("dummy_file.txt").display()
            )
            .as_str()
        ));

        // Removing a non-existent directory should raise an error
        let result = remove_dir(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_dir".to_string(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(
            format!(
                "Failed to delete directory '{}'",
                widget_dir.join("dummy_dir").display()
            )
            .as_str()
        ));

        // Create a file and a directory
        let file_path = widget_dir.join("dummy_file.txt");
        let dir_path = widget_dir.join("dummy_dir");
        std::fs::File::create(&file_path).unwrap();
        std::fs::create_dir(&dir_path).unwrap();

        // Removing a directory using `remove_file` should raise an error
        let result = remove_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_dir".to_string(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(
            format!(
                "Failed to delete file '{}'",
                widget_dir.join("dummy_dir").display()
            )
            .as_str()
        ));

        // Removing a file using `remove_dir` should raise an error
        let result = remove_dir(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(
            format!(
                "Failed to delete directory '{}'",
                widget_dir.join("dummy_file.txt").display()
            )
            .as_str()
        ));

        // Remove the file correctly
        let result = remove_file(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_file.txt".to_string(),
        );
        assert!(result.is_ok());
        assert!(!file_path.exists());

        // Remove the directory correctly
        let result = remove_dir(
            app_handle.clone(),
            "dummy".to_string(),
            "dummy_dir".to_string(),
        );
        assert!(result.is_ok());
        assert!(!dir_path.exists());
    }
}
