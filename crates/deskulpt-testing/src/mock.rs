use deskulpt_test_states::{WidgetBaseDirectoryState, WidgetConfigMapState};
use tauri::test::{mock_app, MockRuntime};
use tauri::{AppHandle, Manager};
use tempfile::{tempdir, TempDir};

/// Setup a mock environment for testing.
///
/// This function does the following:
///
/// - Creates a temporary directory that serves as the base directory for the
///   mock environment. It should be used the same as `$APPDATA`, `$APPCONFIG`,
///   etc. The `TempDir` object itself is returned, because it will be deleted
///   once it goes out of scope.
///
/// - Creates a mock Tauri application. The mock application manages the widget
///   base directory state, which is `$MOCKBASE/widgets`. It also manages an
///   empty widget configuration collection state. A handle to this mock
///   application is returned.
pub fn setup_mock_env() -> (TempDir, AppHandle<MockRuntime>) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let mock_base_dir = temp_dir.path().to_path_buf();

    let app = mock_app();
    let app_handle = app.handle().clone();
    app_handle.manage(WidgetBaseDirectoryState(mock_base_dir.join("widgets")));
    app_handle.manage(WidgetConfigMapState::default());

    (temp_dir, app_handle)
}
