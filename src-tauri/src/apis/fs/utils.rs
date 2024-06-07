//! This module contains the utilities for `fs` in `@deskulpt-test/apis`.

use crate::{cmdbail, states::WidgetBaseDirectoryState};
use path_clean::PathClean;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

/// Validate the file system resource (file or folder) path.
///
/// This raises an error if the widget ID is invalid, in cases where:
///
/// - $widget_base/$widget_id does not exist
/// - $widget_base/$widget_id is not a directory
/// - $widget_base/$widget_id is not a direct subdirectory of $widget_base
///
/// or if the resource path is invalid, in cases where:
///
/// - $widget_base/$widget_id/$path is not within the widget directory
///
/// Note, however, that this function does not check if the resource exists or not,
/// since the file or folder may not exist yet, and could be created later.
pub(super) fn get_resource_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> Result<PathBuf, String> {
    let widget_base = app_handle.state::<WidgetBaseDirectoryState>().0.clone();
    let widget_dir = widget_base.join(widget_id).clean();

    if !widget_dir.is_dir() || !widget_dir.starts_with(&widget_base) || {
        // Count the number of path components to make sure that the widget directory
        // is a direct descendant of the widget base directory
        let base_component_count = widget_base.iter().count();
        let dir_component_count = widget_dir.iter().count();
        base_component_count + 1 != dir_component_count
    } {
        cmdbail!(
            "Invalid widget ID: '{widget_id}'; widget ID must correspond to a direct \
            subdirectory of the widget base directory."
        );
    }

    let resource_path = widget_dir.join(path).clean();
    if !resource_path.starts_with(&widget_dir) {
        cmdbail!(
            "Invalid resource path: '{path}'; resource path must stay within its \
            corresponding widget directory."
        );
    }

    Ok(resource_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::setup_mock_env;
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
    fn test_get_resource_path() {
        // Test that `get_resource_path` returns the correct path, given that the widget
        // ID and the given path are both valid
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");
        let file_path = widget_dir.join("dummy_file.txt");

        // We did not really create the file but the function should not fail
        let result = get_resource_path(&app_handle, "dummy", "dummy_file.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), file_path);
    }

    #[rstest]
    // Widget ID corresponds to a non-existent directory
    #[case::non_existent("dummy", |_: PathBuf| {})]
    // Widget ID corresponds to a file instead of a directory
    #[case::not_a_directory("dummy", |widget_dir: PathBuf| {
        let file_path = widget_dir.join("dummy");
        std::fs::File::create(file_path).unwrap();
    })]
    // Widget ID corresponds to a directory that is beyond the widget base directory
    #[case::beyond_base("../dummy", |widget_dir: PathBuf| {
        // We can safely create a directory one level outside the widget base directory,
        // since that would still be inside the temporary directory 
        let dir_path = widget_dir.join("../dummy");
        std::fs::create_dir(dir_path).unwrap();
    })]
    // Widget ID corresponds to a directory that is not a direct descendant of the
    // widget base directory
    #[case::not_direct_descendant("dummy/subdummy", |widget_dir: PathBuf| {
        let dir_path = widget_dir.join("dummy/subdummy");
        std::fs::create_dir_all(dir_path).unwrap();
    })]
    fn test_get_resource_path_invalid_id(
        #[case] widget_id: &str,
        #[case] setup: fn(PathBuf),
    ) {
        // Test that `get_resource_path` raises an error when the widget ID is invalid
        let (base_dir, app_handle) = setup_mock_env();
        let widget_base = base_dir.path().join("widgets");
        std::fs::create_dir(&widget_base).unwrap();
        setup(widget_base);

        let result = get_resource_path(&app_handle, widget_id, "dummy_file.txt");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                "Invalid widget ID: '{widget_id}'; widget ID must correspond to a direct \
                subdirectory of the widget base directory."
            )
        );
    }

    #[rstest]
    fn test_get_resource_path_invalid_path() {
        // Test that `get_resource_path` raises an error when the widget ID is valid but
        // the given relative resource path is invalid
        let (base_dir, app_handle) = setup_mock_env();
        let widget_dir = setup_widget_directory(base_dir.path(), "dummy");
        std::fs::File::create(widget_dir.join("../dummy_file.txt")).unwrap();

        // The resource path is outside the widget directory, so the validation should
        // not pass even if that resource actually exists
        let result = get_resource_path(&app_handle, "dummy", "../dummy_file.txt");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                "Invalid resource path: '../dummy_file.txt'; resource path must stay \
                within its corresponding widget directory."
            )
        );
    }
}
