//! This module contains the utilities for `fs` in `@deskulpt-test/apis`.

use std::path::PathBuf;

use deskulpt_test_states::StatesExt;
use deskulpt_test_utils::cmdbail;
use path_clean::PathClean;
use tauri::{AppHandle, Runtime};

/// Validate the file system resource (file or folder) path.
///
/// This raises an error if the widget ID is invalid, in cases where:
///
/// - $widgets_dir/$widget_id does not exist
/// - $widgets_dir/$widget_id is not a directory
/// - $widgets_dir/$widget_id is not a direct subdirectory of $widgets_dir
///
/// or if the resource path is invalid, in cases where:
///
/// - $widgets_dir/$widget_id/$path is not within the widget directory
///
/// Note, however, that this function does not check if the resource exists or
/// not, since the file or folder may not exist yet, and could be created later.
pub fn get_resource_path<R: Runtime>(
    app_handle: &AppHandle<R>,
    widget_id: &str,
    path: &str,
) -> Result<PathBuf, String> {
    let widgets_dir = app_handle.widgets_dir();
    let widget_dir = widgets_dir.join(widget_id).clean();

    if !widget_dir.is_dir() || !widget_dir.starts_with(&widgets_dir) || {
        // Count the number of path components to make sure that the widget directory
        // is a direct descendant of the widget base directory
        let base_component_count = widgets_dir.iter().count();
        let dir_component_count = widget_dir.iter().count();
        base_component_count + 1 != dir_component_count
    } {
        cmdbail!(
            "Invalid widget ID: '{widget_id}'; widget ID must correspond to a \
            direct subdirectory of the widget base directory."
        );
    }

    let resource_path = widget_dir.join(path).clean();
    if !resource_path.starts_with(&widget_dir) {
        cmdbail!(
            "Invalid resource path: '{path}'; resource path must stay within \
            its corresponding widget directory."
        );
    }

    Ok(resource_path)
}

#[cfg(test)]
mod tests {
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
    fn test_get_resource_path(mocker: Mocker) {
        // Test that `get_resource_path` returns the correct path when valid
        let result = get_resource_path(mocker.handle(), "dummy", "file.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), mocker.widgets_path("dummy/file.txt"));
    }

    #[rstest]
    // Widget ID corresponds to a non-existent directory
    #[case::non_existent("non_existent")]
    // Widget ID corresponds to a file instead of a directory
    #[case::not_a_directory("not_a_directory")]
    // Widget ID corresponds to a directory that is beyond the widget base directory
    #[case::beyond_base("orphan")]
    // Widget ID corresponds to a directory that is not a direct descendant of the
    // widget base directory
    #[case::not_direct_descendant("nested/dummy")]
    fn test_get_resource_path_invalid_id(#[case] widget_id: &str, mocker: Mocker) {
        // Test that `get_resource_path` raises an error when the widget ID is invalid
        let result = get_resource_path(mocker.handle(), widget_id, "file.txt");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                "Invalid widget ID: '{widget_id}'; widget ID must correspond \
                to a direct subdirectory of the widget base directory."
            )
        );
    }

    #[rstest]
    fn test_get_resource_path_invalid_path(mocker: Mocker) {
        // Test that `get_resource_path` raises an error when the widget ID is valid but
        // the given relative resource path is invalid
        let result = get_resource_path(mocker.handle(), "dummy", "../orphan");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                "Invalid resource path: '../orphan'; resource path must stay \
                within its corresponding widget directory."
            )
        );
    }
}
