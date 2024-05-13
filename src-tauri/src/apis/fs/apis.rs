//! This module implements the commands for `fs` in `@deskulpt-test/apis`.

use super::utils;
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
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    Ok(file_path.exists())
}

#[command]
pub(crate) fn is_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    Ok(file_path.is_file())
}

#[command]
pub(crate) fn is_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<bool> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    Ok(file_path.is_dir())
}

#[command]
pub(crate) fn read_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> CommandOut<String> {
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
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
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
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
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
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
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let file_path = utils::get_resource_path(&app_handle, &widget_id, &path);
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
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let folder_path = utils::get_resource_path(&app_handle, &widget_id, &path);
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
    utils::validate_resource_path(&app_handle, &widget_id, &path)
        .map_err(|e| cmderr!(e))?;
    let folder_path = utils::get_resource_path(&app_handle, &widget_id, &path);
    std::fs::remove_dir_all(&folder_path)
        .context(format!("Failed to delete directory '{}'", folder_path.display()))
        .map_err(|e| cmderr!(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::states::WidgetBaseDirectoryState;
    use rstest::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use tauri::test::{mock_app, MockRuntime};
    use tauri::Manager;
    use tempfile::tempdir;

    fn setup_widget_directory(widget_base: &PathBuf, widget_id: &str) -> PathBuf {
        let widget_dir = widget_base.join(widget_id);
        fs::create_dir_all(&widget_dir).expect("Failed to create widget directory");
        widget_dir
    }

    #[fixture]
    fn env() -> (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir) {
        let temp_dir = tempdir().expect("Failed to create a temporary directory");
        let app_dir = temp_dir.path();
        let widget_base = app_dir.join("widgets");
        fs::create_dir_all(&widget_base)
            .expect("Failed to create widget base directory");

        let app = mock_app();
        let app_handle = app.handle();
        app_handle.manage(WidgetBaseDirectoryState(widget_base.clone()));

        (app_handle.clone(), widget_base, temp_dir)
    }

    #[rstest]
    #[case("test_exists", "test_file.txt", true)]
    #[case("test_exists", "non_existent_file.txt", false)]
    fn test_exists(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] file_name: &str,
        #[case] exists_expected: bool,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        let file_path = widget_dir.join(file_name);
        if exists_expected {
            File::create(&file_path).unwrap().write_all(b"Hello, world!").unwrap();
        }
        assert_eq!(
            exists(env.0.clone(), widget_id.to_string(), file_name.to_string())
                .unwrap(),
            exists_expected,
            "File existence should match expected: {}",
            exists_expected
        );
    }

    #[rstest]
    #[case("test_is_file", "test_file.txt", true)]
    #[case("test_is_file", "new_dir", false)]
    fn test_is_file_or_dir(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] file_name: &str,
        #[case] is_file_expected: bool,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        let path = widget_dir.join(file_name);
        if is_file_expected {
            File::create(&path).unwrap().write_all(b"Hello, world!").unwrap();
        } else {
            fs::create_dir(&path).unwrap();
        }
        assert_eq!(
            is_file(env.0.clone(), widget_id.to_string(), file_name.to_string())
                .unwrap(),
            is_file_expected,
            "Should be file: {}",
            is_file_expected
        );
        assert_eq!(
            is_dir(env.0.clone(), widget_id.to_string(), file_name.to_string())
                .unwrap(),
            !is_file_expected,
            "Should be directory: {}",
            !is_file_expected
        );
    }

    #[rstest]
    #[case("test_read_file", "test_file.txt", "Hello, world!", true)]
    #[case("test_read_file", "nonexistent_file.txt", "", false)]
    fn test_read_file(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] file_name: &str,
        #[case] content: &str,
        #[case] should_succeed: bool,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        let file_path = widget_dir.join(file_name);
        if should_succeed {
            File::create(&file_path).unwrap().write_all(content.as_bytes()).unwrap();
        }
        let result =
            read_file(env.0.clone(), widget_id.to_string(), file_name.to_string());
        assert_eq!(
            result.is_ok(),
            should_succeed,
            "Reading file should match expected success state: {}",
            should_succeed
        );
        if should_succeed {
            assert_eq!(result.unwrap(), content, "File content should match");
        }
    }

    #[rstest]
    #[case("test_write_file", "new_file.txt", "Hello, new world!")]
    fn test_write_file(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] file_name: &str,
        #[case] content: &str,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        write_file(
            env.0,
            widget_id.to_string(),
            file_name.to_string(),
            content.to_string(),
        )
        .unwrap();
        let file_path = widget_dir.join(file_name);
        assert_eq!(
            fs::read_to_string(file_path).unwrap(),
            content,
            "Written content should match input"
        );
    }

    #[rstest]
    #[case(
        "test_append_file",
        "appendable_file.txt",
        "Hello",
        ", world!",
        "Hello, world!"
    )]
    fn test_append_file(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] file_name: &str,
        #[case] initial_content: &str,
        #[case] appended_content: &str,
        #[case] expected_content: &str,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        let file_path = widget_dir.join(file_name);
        File::create(&file_path)
            .unwrap()
            .write_all(initial_content.as_bytes())
            .unwrap();
        append_file(
            env.0,
            widget_id.to_string(),
            file_name.to_string(),
            appended_content.to_string(),
        )
        .unwrap();
        assert_eq!(
            fs::read_to_string(file_path).unwrap(),
            expected_content,
            "Content after append should match expected"
        );
    }

    #[rstest]
    #[case("test_remove_file", "removable_file.txt")]
    fn test_remove_file(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] file_name: &str,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        let file_path = widget_dir.join(file_name);
        File::create(&file_path).unwrap().write_all(b"Delete me").unwrap();
        remove_file(env.0, widget_id.to_string(), file_name.to_string()).unwrap();
        assert!(!file_path.exists(), "File should be removed");
    }

    #[rstest]
    #[case("test_create_dir", "new_directory")]
    fn test_create_dir(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] dir_name: &str,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        create_dir(env.0, widget_id.to_string(), dir_name.to_string()).unwrap();
        assert!(widget_dir.join(dir_name).is_dir(), "Directory should be created");
    }

    #[rstest]
    #[case("test_remove_dir", "removable_directory")]
    fn test_remove_dir(
        env: (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir),
        #[case] widget_id: &str,
        #[case] dir_name: &str,
    ) {
        let widget_dir = setup_widget_directory(&env.1, widget_id);
        let dir_path = widget_dir.join(dir_name);
        fs::create_dir_all(&dir_path).unwrap();
        remove_dir(env.0, widget_id.to_string(), dir_name.to_string()).unwrap();
        assert!(!dir_path.exists(), "Directory should be removed");
    }
}
