//! This module contains the utilities for `fs` in `@deskulpt-test/apis`.

use crate::states::WidgetBaseDirectoryState;
use anyhow::{bail, Context, Error};
use path_clean::PathClean;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

/// Validate if the widget ID corresponds to a direct folder in the widget base folder。
///
/// Note that if the folder does not exist, the function will return an error, since
/// this likely indicates a mistake in the widget ID.
pub(crate) fn validate_widget_id<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
) -> Result<(), Error> {
    let widget_base = get_widget_base(app_handle);
    let widget_dir = get_widget_dir(app_handle, widget_id);

    let widget_base_clean = widget_base.clean();
    let widget_dir_clean = widget_dir.clean();

    // If the following conditions are not met, the widget ID is invalid:
    // - the $widget_base/$widget_id exists
    // - the $widget_base/$widget_id is a directory
    // - the $widget_base/$widget_id is a **direct** subdirectory of $widget_base
    if !widget_dir_clean.is_dir()
        || !widget_dir_clean.starts_with(widget_base_clean.clone())
    {
        bail!(
            "Invalid widget ID: '{}'. Widget ID must be a direct subdirectory of the \
            widget base directory.",
            widget_id
        );
    }

    // Check if widget_dir is a direct subdirectory of widget_base
    // This checks that there is exactly one more component in widget_dir than in widget_base
    let base_component_count = widget_base_clean.iter().count();
    let dir_component_count = widget_dir_clean.iter().count();

    if dir_component_count != base_component_count + 1 {
        bail!(
            "Invalid widget ID: '{}'. Widget ID must be a direct subdirectory of the \
            widget base directory.",
            widget_id
        );
    }

    Ok(())
}

/// Validate the file system resource (file or folder) path.
///
/// In particular, this function checks if the widget ID corresponds to a direct
/// subfolder of $widget_base.
///
/// Note that this function does not check if the resource exists or not, unlike
/// `validate_widget_id` which requires the widget folder to exist to be valid. This is
/// because the file or folder may not exist yet, and will be created later.
pub(crate) fn validate_resource_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> Result<(), Error> {
    // Validate if the widget ID is a direct subfolder of the widget_base folder
    validate_widget_id(app_handle, widget_id)
        .context(format!("Failed to validate widget ID: '{}'", widget_id))?;

    let widget_dir = get_widget_dir(app_handle, widget_id);
    let resource_path = widget_dir.join(path).clean();

    // Validate if the file is within the widget directory
    if !resource_path.starts_with(&widget_dir) {
        bail!(
            "Invalid resource path: '{}'. Resource must be within the widget directory",
            path
        );
    }

    Ok(())
}

/// Get the widget base directory.
pub(crate) fn get_widget_base<R: Runtime>(app_handle: &AppHandle<R>) -> PathBuf {
    app_handle.state::<WidgetBaseDirectoryState>().0.clone()
}

/// Get the widget directory by its widget ID.
pub(crate) fn get_widget_dir<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
) -> PathBuf {
    get_widget_base(app_handle).join(widget_id)
}

// Get the resource path in the widget directory with given ID.
pub(crate) fn get_resource_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> PathBuf {
    get_widget_dir(app_handle, widget_id).join(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use path_clean::PathClean;
    use rstest::rstest;
    use std::fs;
    use std::path::PathBuf;
    use tauri::test::{mock_app, MockRuntime};
    use tempfile::tempdir;
    // use rstest_reuse::{self, *};

    // Setup the base widget environment and return the app handle and base directory path
    fn setup_base_environment() -> (AppHandle<MockRuntime>, PathBuf, tempfile::TempDir)
    {
        let temp_dir = tempdir().expect("Failed to create a temporary directory");
        let app_dir = temp_dir.path();
        let widget_base = PathBuf::from(app_dir).join("widgets").clean();
        fs::create_dir_all(&widget_base)
            .expect("Failed to create widget base directory");
        let app = mock_app();
        let app_handle = app.handle();
        app_handle.manage(WidgetBaseDirectoryState(widget_base.clone()));
        (app_handle.clone(), widget_base, temp_dir)
    }

    // Setup a specific widget directory within the widget base environment
    fn setup_widget_directory(widget_base: &PathBuf, widget_id: &str) -> PathBuf {
        let widget_dir = widget_base.join(widget_id);
        fs::create_dir_all(&widget_dir).expect("Failed to create widget directory");
        widget_dir
    }

    #[rstest]
    #[case("test_widget")]
    fn test_get_widget_base(#[case] widget_id: &str) {
        let (app_handle, expected_base, _temp_dir) = setup_base_environment();
        let widget_base = get_widget_base(&app_handle);
        assert_eq!(
            widget_base, expected_base,
            "The widget base directory should match the expected path."
        );
    }

    #[rstest]
    #[case("test_widget")]
    fn test_get_widget_dir(#[case] widget_id: &str) {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let _widget_dir = setup_widget_directory(&widget_base, widget_id);
        let widget_dir = get_widget_dir(&app_handle, widget_id);
        let expected_dir = widget_base.join(widget_id);
        assert_eq!(
            widget_dir, expected_dir,
            "The widget directory should match the expected path."
        );
    }

    #[rstest]
    #[case("test_widget", "test.txt")]
    fn test_get_resource_path(#[case] widget_id: &str, #[case] resource_name: &str) {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let _widget_dir = setup_widget_directory(&widget_base, widget_id);
        let resource_path = get_resource_path(&app_handle, widget_id, resource_name);
        let expected_path = get_widget_dir(&app_handle, widget_id).join(resource_name);
        assert_eq!(
            resource_path, expected_path,
            "The resource path should match the expected path."
        );
    }

    // To cover corner cases of validate widget id, we need to write some verbose code
    #[rstest]
    // A valid widget ID that is a direct subfolder of the widget base directory
    #[case("valid_widget", true)]
    // An invalid widget ID that is not an existing folder in the widget base directory
    #[case("nonexisting-widget", false)]
    // An invalid widget ID that is not a folder but a file
    #[case("invalid_widget", false)]
    // An invalid widget ID that is a subfolder but not a direct subfolder of the widget base directory
    #[case("subfolder/subsubfolder_widget", false)]
    // An invalid widget ID that is a folder outside of the widget base directory
    //   `validate_widget_id`` won't search outside of the widget base directory,
    //   so the error is caused by the path not existing.
    #[case("../outside_widget", false)]
    // an invalid widget ID that is a folder outside of the widget base directory containing the relative path
    //    `validate_widget_id` will search outside of the widget base directory after cleaning the path,
    //    so the error is caused by the path not being a direct subfolder of the widget base directory.
    #[case("../relative_outside_widget", false)]
    fn test_validate_widget_id(#[case] widget_id: &str, #[case] is_valid: bool) {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        println!("Widget Base: {:?}", widget_base);
        let is_widget_base_exists = widget_base.exists();
        println!("Widget Base Exists: {:?}", is_widget_base_exists);

        match widget_id {
            "valid_widget" => {
                setup_widget_directory(&widget_base, widget_id);
            },
            "invalid_widget" => {
                let file_path = widget_base.join(widget_id);
                fs::File::create(&file_path)
                    .expect("Failed to create a file for testing");
            },
            "subfolder/subsubfolder_widget" => {
                let subfolder_path =
                    widget_base.join("subfolder").join("subsubfolder_widget");
                fs::create_dir_all(&subfolder_path)
                    .expect("Failed to create a subfolder for testing");
            },
            "../outside_widget" | "../relative_outside_widget" => {
                let outside_path = widget_base
                    .parent()
                    .unwrap()
                    .join(widget_id.strip_prefix("../").unwrap());
                fs::create_dir_all(&outside_path).expect("Failed to create a folder outside the widget base directory for testing");
            },
            _ => {}, // Handle nonexisting-widget by not setting up anything
        }

        let result = validate_widget_id(&app_handle, widget_id);
        assert_eq!(
            result.is_ok(),
            is_valid,
            "Validation for widget ID '{}' should {}. Error: {:?}",
            widget_id,
            if is_valid { "succeed" } else { "fail" },
            result.err()
        );
    }

    #[rstest]
    #[case("widget_for_resource_validation", "file.txt", true)]
    #[case("widget_for_resource_validation", "subdir", true)]
    #[case("widget_for_resource_validation", "non_existing_file.txt", true)]
    #[case("widget_for_resource_validation", "../outside_file.txt", false)]
    fn test_validate_resource_path(
        #[case] widget_id: &str,
        #[case] resource: &str,
        #[case] should_succeed: bool,
    ) {
        let (app_handle, widget_base, _temp_dir) = setup_base_environment();
        let widget_dir = setup_widget_directory(&widget_base, widget_id);
        let file_path = widget_dir.join(resource);
        if resource.contains(".") && should_succeed {
            fs::File::create(&file_path).expect("Failed to create a file for testing");
        } else if should_succeed {
            fs::create_dir_all(&file_path)
                .expect("Failed to create a directory for testing");
        }
        let result = validate_resource_path(&app_handle, widget_id, resource);
        assert_eq!(
            result.is_ok(),
            should_succeed,
            "Expected resource path validation for '{}' to be {}. Error: {:?}",
            resource,
            should_succeed,
            result
        );
    }
}
