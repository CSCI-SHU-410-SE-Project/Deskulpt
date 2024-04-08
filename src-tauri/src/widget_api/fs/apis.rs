use crate::states::WidgetBaseDirectoryState;
use std::io::Write;
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager, Runtime};

use crate::widget_api::fs::utils;

#[command]
pub fn exists<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<bool, String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let file_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    Ok(file_path.exists())
}

// Note that the `is_file` is a command for widgets. For checking if an entrhy is file,
//    we should use std::path::PathBuf::is_file instead.
#[command]
pub fn is_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<bool, String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let file_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    Ok(file_path.is_file())
}

#[command]
pub fn is_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<bool, String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let file_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    Ok(file_path.is_dir())
}

#[command]
pub fn read_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<String, String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let file_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    if !file_path.is_file() {
        return Err(format!("Path '{}' is not a file", file_path.display()));
    }

    std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))
}

#[command]
pub fn write_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let file_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    std::fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write file '{}': {}", file_path.display(), e))
}

#[command]
pub fn append_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
    content: String,
) -> Result<(), String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let file_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| format!("Failed to append file '{}': {}", file_path.display(), e))
}

#[command]
pub fn remove_file<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let file_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    if !file_path.is_file() {
        return Err(format!("Path '{}' is not a file", file_path.display()));
    }
    std::fs::remove_file(&file_path)
        .map_err(|e| format!("Failed to delete file '{}': {}", file_path.display(), e))
}

#[command]
pub fn create_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let folder_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    if folder_path.exists() {
        return Err(format!("Directory '{}' already exists", folder_path.display()));
    }
    std::fs::create_dir_all(&folder_path).map_err(|e| {
        format!("Failed to create directory '{}': {}", folder_path.display(), e)
    })
}

#[command]
pub fn remove_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    widget_id: String,
    path: String,
) -> Result<(), String> {
    utils::validate_entry_path(&app_handle, &widget_id, &path)?;
    let folder_path = utils::get_entry_path(&app_handle, &widget_id, &path);
    std::fs::remove_dir_all(&folder_path).map_err(|e| {
        format!("Failed to delete directory '{}': {}", folder_path.display(), e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tauri::test::{mock_app, MockRuntime};
    use tempfile::tempdir;

    fn setup_base_environment() -> (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir)
    {
        let temp_dir = tempdir().expect("Failed to create a temporary directory");
        let app_dir = temp_dir.path();
        let widget_base = app_dir.join("widgets");
        fs::create_dir_all(&widget_base)
            .expect("Failed to create widget base directory");

        let app = mock_app();
        let app_handle = app.handle();
        app_handle.manage(WidgetBaseDirectoryState(widget_base.clone()));

        (app_handle, widget_base, temp_dir)
    }

    fn setup_widget_directory(widget_base: &PathBuf, widget_id: &str) -> PathBuf {
        let widget_dir = widget_base.join(widget_id);
        fs::create_dir_all(&widget_dir).expect("Failed to create widget directory");
        widget_dir
    }

    #[test]
    fn test_exists() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();

        // Test for a file that exists
        let widget_id = "test_exists";
        let widget_dir = setup_widget_directory(&widget_base, widget_id);
        let file_name = "test_file.txt";
        let file_path = widget_dir.join(file_name);
        File::create(&file_path).unwrap().write_all(b"Hello, world!").unwrap();
        assert!(
            exists(app_handle.clone(), widget_id.to_string(), file_name.to_string())
                .unwrap(),
            "File should exist",
        );

        // Test for a file that does not exist
        let non_existent_file = "non_existent_file.txt";
        assert!(
            !exists(
                app_handle.clone(),
                widget_id.to_string(),
                non_existent_file.to_string()
            )
            .unwrap(),
            "File should not exist",
        );
    }

    #[test]
    fn test_is_file() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();

        let widget_id = "test_is_file";
        let widget_dir = setup_widget_directory(&widget_base, widget_id);

        // Test for a file
        let file_name = "test_file.txt";
        let file_path = widget_dir.join(file_name);
        File::create(&file_path).unwrap().write_all(b"Hello, world!").unwrap();
        assert!(
            is_file(app_handle.clone(), widget_id.to_string(), file_name.to_string())
                .unwrap(),
            "Path should be a file",
        );

        // Test for a directory
        let dir_name = "new_dir";
        let dir_path = widget_dir.join(dir_name);
        fs::create_dir(&dir_path).unwrap();
        assert!(
            !is_file(app_handle.clone(), widget_id.to_string(), dir_name.to_string())
                .unwrap(),
            "Path should not be a file",
        );
    }

    #[test]
    fn test_is_dir() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_id = "test_is_dir";
        let widget_dir = setup_widget_directory(&widget_base, widget_id);

        // Test for a directory
        let dir_name = "new_dir";
        let dir_path = widget_dir.join(dir_name);
        fs::create_dir(&dir_path).unwrap();
        assert!(
            is_dir(app_handle.clone(), widget_id.to_string(), dir_name.to_string())
                .unwrap(),
            "Path should be a directory",
        );

        // Test for a file
        let file_name = "test_file.txt";
        let file_path = widget_dir.join(file_name);
        File::create(&file_path).unwrap().write_all(b"Hello, world!").unwrap();
        assert!(
            !is_dir(app_handle.clone(), widget_id.to_string(), file_name.to_string())
                .unwrap(),
            "Path should not be a directory",
        );
    }

    #[test]
    fn test_read_file() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_id = "test_read_file";
        let widget_dir = setup_widget_directory(&widget_base, widget_id);

        // Test for reading a file that exists in the widget directory
        let file_name = "test_file.txt";
        let file_path = widget_dir.join(file_name);
        let file_content = "Hello, world!";
        File::create(&file_path).unwrap().write_all(file_content.as_bytes()).unwrap();
        let result =
            read_file(app_handle.clone(), widget_id.to_string(), file_name.to_string());
        assert!(
            result.is_ok(),
            "Reading an existing file should succeed: {}",
            result.unwrap_err()
        );
        let content = result.unwrap();
        assert_eq!(content, file_content, "File content should match");

        // Test for reading a file that exists in a subdirectory of the widget directory
        let sub_dir_name = "sub_dir";
        let sub_dir_path = widget_dir.join(sub_dir_name);
        fs::create_dir(&sub_dir_path).unwrap();
        let sub_file_name = "sub_file.txt";
        let sub_file_path = sub_dir_path.join(sub_file_name);
        let sub_file_content = "Sub file content";
        File::create(&sub_file_path)
            .unwrap()
            .write_all(sub_file_content.as_bytes())
            .unwrap();
        let sub_file_path_arg = PathBuf::from(sub_dir_name).join(sub_file_name);
        let result = read_file(
            app_handle.clone(),
            widget_id.to_string(),
            sub_file_path_arg.to_string_lossy().to_string(),
        );
        assert!(
            result.is_ok(),
            "Reading an existing sub file should succeed: {}",
            result.unwrap_err()
        );
        let content = result.unwrap();
        assert_eq!(content, sub_file_content, "Sub file content should match");

        // Test for reading a file that does not exist
        let non_existent_file = "non_existent_file.txt";
        let result = read_file(
            app_handle.clone(),
            widget_id.to_string(),
            non_existent_file.to_string(),
        );
        assert!(result.is_err(), "Reading a non-existent file should return an error");
        println!("Error message: {}", result.unwrap_err());

        // Test for reading a nonexisting file outside widget dir whose path contains relative path components
        let relative_outside_nonexistent_file = "../non_existent_file.txt";
        let result = read_file(
            app_handle.clone(),
            widget_id.to_string(),
            relative_outside_nonexistent_file.to_string(),
        );
        assert!(
            result.is_err(),
            "Reading a non-existent file outside widget dir should return an error"
        );
        println!("Error message: {}", result.unwrap_err());

        // Test for reading a existing file outside widget dir whose path contains relative path components
        let relative_outside_existing_file = "../test_file.txt";
        // create a file outside the widget directory
        let outside_file_path = widget_base.join(relative_outside_existing_file);
        let outside_file_content = "Outside file content";
        File::create(&outside_file_path)
            .unwrap()
            .write_all(outside_file_content.as_bytes())
            .unwrap();
        let result = read_file(
            app_handle.clone(),
            widget_id.to_string(),
            relative_outside_existing_file.to_string(),
        );
        assert!(
            result.is_err(),
            "Reading a file outside widget dir should return an error"
        );
        println!("Error message: {}", result.unwrap_err());
    }

    #[test]
    fn test_write_file() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_id = "test_write_file";
        let widget_dir = setup_widget_directory(&widget_base, widget_id);
        let file_name = "test_file.txt";
        let file_content = "Hello, world!";

        write_file(
            app_handle,
            widget_id.to_string(),
            file_name.to_string(),
            file_content.to_string(),
        )
        .unwrap();
        let content = fs::read_to_string(widget_dir.join(file_name)).unwrap();
        assert_eq!(content, file_content, "File content should match");
    }

    #[test]
    fn test_append_file() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_id = "test_append_file";
        let widget_dir = setup_widget_directory(&widget_base, widget_id);
        let file_name = "test_file.txt";
        let initial_content = "Hello";
        let appended_content = ", world!";
        let full_content = format!("{}{}", initial_content, appended_content);

        write_file(
            app_handle.clone(),
            widget_id.to_string(),
            file_name.to_string(),
            initial_content.to_string(),
        )
        .unwrap();
        append_file(
            app_handle,
            widget_id.to_string(),
            file_name.to_string(),
            appended_content.to_string(),
        )
        .unwrap();
        let content = fs::read_to_string(widget_dir.join(file_name)).unwrap();
        assert_eq!(content, full_content, "File content should match");
    }

    #[test]
    fn test_remove_file() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_id = "test_remove_file";
        let widget_dir = setup_widget_directory(&widget_base, widget_id);
        let file_name = "test_file.txt";
        File::create(widget_dir.join(file_name))
            .unwrap()
            .write_all(b"Hello, world!")
            .unwrap();

        remove_file(app_handle, widget_id.to_string(), file_name.to_string()).unwrap();
        assert!(!widget_dir.join(file_name).exists(), "File should be removed");
    }

    #[test]
    fn test_create_dir() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_id = "test_create_dir";
        let _widget_dir = setup_widget_directory(&widget_base, widget_id);
        let dir_name = "new_dir";

        let result =
            create_dir(app_handle, widget_id.to_string(), dir_name.to_string());
        assert!(
            result.is_ok(),
            "Creating a directory should succeed: {}",
            result.unwrap_err()
        );
        assert!(widget_base.join(widget_id).join(dir_name).is_dir());
    }

    #[test]
    fn test_remove_dir() {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_id = "test_remove_dir";
        let _widget_dir = setup_widget_directory(&widget_base, widget_id);
        let dir_name = "new_dir";
        let dir_path = widget_base.join(widget_id).join(dir_name);
        fs::create_dir_all(&dir_path).unwrap();

        remove_dir(app_handle, widget_id.to_string(), dir_name.to_string()).unwrap();
        assert!(!dir_path.exists(), "Directory should be removed");
    }
}
