use crate::states::WidgetBaseDirectoryState;
use path_absolutize::Absolutize;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

// TODO: Use context instead of map_err
// TODO: Use debugg println!
// TODO: Write unit tests
// DONE: Remove the use of storage directory

/// Validate if the widget ID corresponds to a direct folder in the widget_base folder
///
/// Note that if the folder does not exist, the function will return an error, since this
/// likely indicates a mistake in the widget ID.
pub fn validate_widget_id<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
) -> Result<(), String> {
    if cfg!(debug_assertions) {
        println!("Validating widget ID: '{}'", widget_id);
    }

    // Error messages should be generic and not contain any specific information
    //  to prevent information leakage.
    let error_msg = format!(
        "Invalid widget ID: '{}'. Widget ID must correspond to a folder in the widget base directory.",
        widget_id
    );

    let widget_base = get_widget_base(app_handle);
    let widget_dir = get_widget_dir(app_handle, widget_id);
    // if canonicalized() is used, and the app runs on Windows, the long path prefix "\\?\" will be added
    //  to the path, which may cause issues with path comparisons.
    let widget_dir_absolute = widget_dir.absolutize().map_err(|e| {
        format!(
            "Failed to get absolutize path of widget directory '{}': {}",
            widget_id, e
        )
    })?;

    if !widget_dir_absolute.exists() {
        // return Err(error_msg);
        return Err(format!(
            "Invalid widget ID: '{}'. Widget ID must correspond to an existing folder in the widget base directory.",
            widget_id
        ));
    }

    // Test if the $widget_base/$widget_id is a directory
    if !widget_dir_absolute.is_dir() {
        if cfg!(debug_assertions) {
            return Err(format!(
                "Invalid widget ID: '{}'. Widget ID must be a folder",
                widget_id
            ));
        } else {
            return Err(error_msg);
        }
    }

    // Test if the $widget_base/$widget_id is a **direct** subdirectory of $widget_base
    if widget_dir_absolute.parent() != Some(&widget_base) {
        if cfg!(debug_assertions) {
            return Err(format!(
                "Invalid widget ID: '{}'. Widget ID must be a direct subfolder of the widget base directory.\n\twidget_base: '{}'\n\twidget_dir: '{}'",
                widget_id, widget_base.display(), widget_dir_absolute.display()
            ));
        } else {
            return Err(error_msg);
        }
    }
    Ok(())
}

/// Validate the file system entry (file or folder) path by checking
/// - if the widget ID is valid
/// - if the entry is within the widget directory.
///
/// Note that this function doesn't check if the entry exists or not, unlike `validate_widget_id()`
///   where the widget folder must exist to be valid. This is because the file or folder may not exist yet,
///   and will be created later.
pub fn validate_entry_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> Result<(), String> {
    if cfg!(debug_assertions) {
        println!("Validating entry path: '{}' for widget '{}'", path, widget_id);
    }

    // Validate if the widget ID is a direct subfolder of the widget_base folder
    validate_widget_id(app_handle, widget_id)?;

    let widget_dir = get_widget_dir(app_handle, widget_id);
    let entry_path = widget_dir.join(path);

    // Note that we use absolutize() instead of canonicalize() here, since we don't need to check
    //   if the file exists or not.
    let entry_path_absolute = entry_path.absolutize().map_err(|e| {
        format!(
            "Failed to get absolute path of entry '{}': {}",
            entry_path.display(),
            e
        )
    })?;

    // Validate if the file is within the widget directory
    if entry_path_absolute.parent() != Some(&widget_dir) {
        return Err(format!(
            "Invalid entry path: '{}'. Entry must be within the widget directory.",
            path
        ));
    }

    Ok(())
}

/// Get the widget base directory from the app state, return the absolute path of the widget base directory.
pub fn get_widget_base<R: Runtime>(app_handle: &AppHandle<R>) -> PathBuf {
    app_handle.state::<WidgetBaseDirectoryState>().0.clone()
}

/// Get the widget directory from the app state given the widget ID.
pub fn get_widget_dir<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
) -> PathBuf {
    get_widget_base(app_handle).join(widget_id)
}

// Get the entry path in the widget directory given the widget ID and the entry path.
pub fn get_entry_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> PathBuf {
    get_widget_dir(app_handle, widget_id).join(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use path_absolutize::Absolutize;
    use std::fs;
    use tauri::test::{mock_app, MockRuntime};
    use tempfile::tempdir;

    // Setup the base widget environment and return the app handle and base directory path
    fn setup_base_environment() -> (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir)
    {
        let temp_dir = tempdir().expect("Failed to create a temporary directory");
        let app_dir = temp_dir.path();

        let widget_base = app_dir
            .join("widgets")
            .absolutize()
            .expect("Failed to get absolute path of widget base directory")
            .to_path_buf();

        let app = mock_app();
        let app_handle = app.handle();
        app_handle.manage(WidgetBaseDirectoryState(widget_base.clone()));

        (app_handle, widget_base, temp_dir)
    }

    // Setup a specific widget directory within the widget base environment
    fn setup_widget_directory(widget_base: &PathBuf, widget_id: &str) -> PathBuf {
        let widget_dir = widget_base.join(widget_id);
        fs::create_dir_all(&widget_dir).expect("Failed to create widget directory");
        widget_dir
    }

    #[test]
    fn test_get_widget_base() {
        let (app_handle, expected_base, _temp_dir) = setup_base_environment();
        let widget_base = get_widget_base(&app_handle);
        assert_eq!(
            widget_base, expected_base,
            "The widget base directory should match the expected path."
        );
    }

    #[test]
    fn test_get_widget_dir() {
        let widget_id = "test_widget";
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let _widget_dir = setup_widget_directory(&widget_base, widget_id);
        let widget_dir = get_widget_dir(&app_handle, widget_id);
        let expected_dir = widget_base.join(widget_id);
        assert_eq!(
            widget_dir, expected_dir,
            "The widget directory should match the expected path."
        );
    }

    #[test]
    fn test_get_entry_path() {
        let widget_id = "test_widget";
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let _widget_dir = setup_widget_directory(&widget_base, widget_id);
        let entry_path = get_entry_path(&app_handle, widget_id, "test.txt");
        let expected_path = get_widget_dir(&app_handle, widget_id).join("test.txt");
        assert_eq!(
            entry_path, expected_path,
            "The entry path should match the expected path."
        );
    }

    #[test]
    fn test_validate_widget_id() {
        // A valid widget ID that is a direct subfolder of the widget base directory
        let widget_id = "valid_widget";
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        setup_widget_directory(&widget_base, widget_id);

        let result = validate_widget_id(&app_handle, widget_id);
        assert!(
            result.is_ok(),
            "Validating existing widget ID should succeed. Error: {}",
            result.unwrap_err()
        );

        // An invalid widget ID that is not an existing folder in the widget base directory
        let invalid_nonexisting_widget_id = "nonexisting-widget";
        let result = validate_widget_id(&app_handle, invalid_nonexisting_widget_id);
        assert!(result.is_err(), "Validating non-existing widget ID should fail.");
        println!("Error Message: {}", result.unwrap_err());

        // An invalid widget ID that is not a folder but a file
        let invalid_nonfolder_widget_id = "invalid_widget";
        // Create a file instead of a directory to simulate the invalid widget scenario
        let file_path = widget_base.join(invalid_nonfolder_widget_id);
        fs::File::create(&file_path).expect("Failed to create a file for testing");
        let result = validate_widget_id(&app_handle, invalid_nonfolder_widget_id);
        assert!(result.is_err(), "Validating non-folder widget ID should fail.");
        println!("Error Message: {}", result.unwrap_err());

        // An invalid widget ID that is a subfolder but not a direct subfolder of the widget base directory
        let invalid_subfolder_widget_id = "subsubfolder_widget";
        let subfolder_path =
            widget_base.join("subfolder").join(invalid_subfolder_widget_id);
        fs::create_dir_all(&subfolder_path)
            .expect("Failed to create a subfolder for testing");
        let result = validate_widget_id(&app_handle, invalid_subfolder_widget_id);
        assert!(result.is_err(), "Validating subfolder widget ID should fail.");
        println!("Error Message: {}", result.unwrap_err());

        // An invalid widget ID that is a folder outside of the widget base directory
        //   `validate_widget_id`` won't search outside of the widget base directory,
        //   so the error is caused by the path not existing.
        let invalid_outside_widget_id = "outside_widget";
        let outside_path =
            widget_base.parent().unwrap().join(invalid_outside_widget_id);
        fs::create_dir_all(&outside_path).expect(
            "Failed to create a folder outside the widget base directory for testing",
        );
        let result = validate_widget_id(&app_handle, invalid_outside_widget_id);
        assert!(result.is_err(), "Validating outside widget ID should fail.");
        println!("Error Message: {}", result.unwrap_err());

        // an invalid widget ID that is a folder outside of the widget base directory containing the relative path
        //    `validate_widget_id` will search outside of the widget base directory after absolutizing the path,
        //    so the error is caused by the path not being a direct subfolder of the widget base directory.
        let invalid_relative_widget_id = "../relative_outside_widget";
        let relative_path =
            widget_base.parent().unwrap().join("relative_outside_widget");
        fs::create_dir_all(&relative_path).expect(
            "Failed to create a folder outside the widget base directory for testing",
        );
        let result = validate_widget_id(&app_handle, invalid_relative_widget_id);
        assert!(result.is_err(), "Validating relative outside widget ID should fail.");
        println!("Error Message: {}", result.unwrap_err());
    }

    #[test]
    fn test_validate_entry_path() {
        let widget_id = "widget_for_entry_validation";
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_dir = setup_widget_directory(&widget_base, widget_id);

        // Valid file path within the widget directory
        let entry = "file.txt";
        let file_path = widget_dir.join(&entry);
        fs::File::create(&file_path).expect("Failed to create a file for testing");
        let result = validate_entry_path(&app_handle, widget_id, &entry);
        assert!(
            result.is_ok(),
            "Validating entry path within widget directory should succeed. Error: {}",
            result.unwrap_err()
        );

        // Valid folder path within the widget directory
        let directory_entry = "subdir";
        let dir_path = widget_dir.join(&directory_entry);
        fs::create_dir_all(&dir_path)
            .expect("Failed to create a directory for testing");
        let result = validate_entry_path(&app_handle, widget_id, directory_entry);
        assert!(
            result.is_ok(),
            "Validating entry path that is a directory should succeed. Error: {}",
            result.unwrap_err()
        );

        // Valid file path pointing to a non-existent file
        let non_existing_entry = "non_existing_file.txt";
        let result = validate_entry_path(&app_handle, widget_id, non_existing_entry);
        assert!(
            result.is_ok(),
            "Validating non-existent entry path should succeed. Error: {}",
            result.unwrap_err()
        );

        // Entry path outside the widget directory
        let outside_entry = "../outside_file.txt";
        let result = validate_entry_path(&app_handle, widget_id, outside_entry);
        assert!(
            result.is_err(),
            "Validating entry path outside widget directory should fail."
        );
        println!("Error Message: {}", result.unwrap_err());
    }
}
