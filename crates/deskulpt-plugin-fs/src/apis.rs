//! This module implements the commands for `fs` in `@deskulpt-test/apis`.

use std::io::Write;

use anyhow::Context;
use deskulpt_test_utils::{cmderr, CommandOut};
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

#[cfg(test)]
mod tests {
    use deskulpt_test_testing::assert::assert_eq;
    use deskulpt_test_testing::fixture_path;
    use deskulpt_test_testing::mock::{Mocker, MockerBuilder};
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn mocker() -> Mocker {
        MockerBuilder::default()
            .with_widgets_dir(fixture_path("deskulpt-plugin-fs/widgets"))
            .build()
    }

    #[rstest]
    async fn test_exists(mocker: Mocker) {
        // Test the `fs::exists` command
        let file_path = mocker.widgets_path("dummy/new_file.txt");

        // The file does not exist yet and should return false
        let result = exists(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Create the file and should return true
        std::fs::File::create(file_path).unwrap();
        let result = exists(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[rstest]
    async fn test_is_file_or_dir(mocker: Mocker) {
        // Test the `fs::is_file` and `fs::is_dir` commands

        // Check that `is_file` gives true and `is_dir` gives false for the file
        let result = is_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "file.txt".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap());
        let result = is_dir(
            mocker.handle().clone(),
            "dummy".to_string(),
            "file.txt".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Check that `is_file` gives false and `is_dir` gives true for the directory
        let result = is_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "dir".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
        let result = is_dir(
            mocker.handle().clone(),
            "dummy".to_string(),
            "dir".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[rstest]
    async fn test_read_file(mocker: Mocker) {
        // Test the `fs::read_file` command
        let file_path = mocker.widgets_path("dummy/new_file.txt");

        // The file does not exist yet and should return an error
        let result = read_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
        )
        .await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains(format!("Failed to read file '{}'", file_path.display()).as_str()));

        // Create the file with some content and should return the content
        let content = "Hello, world!";
        std::fs::File::create(&file_path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
        let result = read_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[rstest]
    async fn test_write_file(mocker: Mocker) {
        // Test the `fs::write_file` command
        let file_path = mocker.widgets_path("dummy/new_file.txt");

        // Writing to a non-existent path should create the file
        let content = "Hello, world!";
        let result = write_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
            content.to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), content);

        // Writing to an existing file should overwrite the content
        let new_content = "Hello, new world!";
        let result = write_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
            new_content.to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), new_content);
    }

    #[rstest]
    async fn test_append_file(mocker: Mocker) {
        // Test the `fs::append_file` command
        let file_path = mocker.widgets_path("dummy/new_file.txt");

        // Appending to a non-existent path should create the file
        let content = "Hello, world!\n";
        let result = append_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
            content.to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), content);

        // Appending to an existing file should append the content
        let new_content = "Hello, new world!\n";
        let result = append_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
            new_content.to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert_eq!(
            std::fs::read_to_string(&file_path).unwrap(),
            format!("{content}{new_content}")
        );
    }

    #[rstest]
    async fn test_create_dir(mocker: Mocker) {
        // Test the `fs::create_dir` command
        let result = create_dir(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_dir".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(mocker.widgets_path("dummy/new_dir").is_dir());
    }

    #[rstest]
    async fn test_remove_file_or_dir(mocker: Mocker) {
        // Test the `fs::remove_file` and `fs::remove_dir` commands

        // Removing a non-existent file should raise an error
        let result = remove_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_file.txt".to_string(),
        )
        .await;
        assert!(result.is_err());
        assert!(result.clone().unwrap_err().contains(
            format!(
                "Failed to delete file '{}'",
                mocker.widgets_path("dummy/new_file.txt").display()
            )
            .as_str()
        ));

        // Removing a non-existent directory should raise an error
        let result = remove_dir(
            mocker.handle().clone(),
            "dummy".to_string(),
            "new_dir".to_string(),
        )
        .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(
            format!(
                "Failed to delete directory '{}'",
                mocker.widgets_path("dummy/new_dir").display()
            )
            .as_str()
        ));

        let file_path = mocker.widgets_path("dummy/file.txt");
        let dir_path = mocker.widgets_path("dummy/dir");

        // Removing a directory using `remove_file` should raise an error
        let result = remove_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "dir".to_string(),
        )
        .await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains(format!("Failed to delete file '{}'", dir_path.display()).as_str()));

        // Removing a file using `remove_dir` should raise an error
        let result = remove_dir(
            mocker.handle().clone(),
            "dummy".to_string(),
            "file.txt".to_string(),
        )
        .await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains(format!("Failed to delete directory '{}'", file_path.display()).as_str()));

        // Remove the file correctly
        let result = remove_file(
            mocker.handle().clone(),
            "dummy".to_string(),
            "file.txt".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(!file_path.exists());

        // Remove the directory correctly
        let result = remove_dir(
            mocker.handle().clone(),
            "dummy".to_string(),
            "dir".to_string(),
        )
        .await;
        assert!(result.is_ok());
        assert!(!dir_path.exists());
    }
}
