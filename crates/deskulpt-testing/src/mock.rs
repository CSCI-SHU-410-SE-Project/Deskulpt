//! Mocking utilities for Deskulpt components.

use std::path::{Path, PathBuf};

use copy_dir::copy_dir;
use deskulpt_test_states::{WidgetBaseDirectoryState, WidgetConfigMapState};
use path_clean::PathClean;
use tauri::test::{mock_app, MockRuntime};
use tauri::{App, AppHandle, Manager};
use tempfile::{tempdir, TempDir};

/// Builder for the Deskulpt mocker.
#[derive(Default)]
pub struct MockerBuilder {
    widgets_dir: Option<PathBuf>,
}

impl MockerBuilder {
    pub fn with_widgets_dir<T: AsRef<Path>>(mut self, widgets_dir: T) -> Self {
        self.widgets_dir = Some(widgets_dir.as_ref().to_path_buf());
        self
    }

    /// Build a new mocker instance.
    pub fn build(&self) -> Mocker {
        let app = mock_app();
        let data_dir = tempdir().expect("Failed to create temporary directory");

        if let Some(widgets_dir_src) = &self.widgets_dir {
            copy_dir(widgets_dir_src, &data_dir.path().join("widgets"))
                .expect("Failed to copy widgets directory");
        }

        let app_handle = app.handle();
        app_handle.manage(WidgetBaseDirectoryState::init(
            data_dir.path().to_path_buf(),
        ));
        app_handle.manage(WidgetConfigMapState::default());

        Mocker { app, data_dir }
    }
}

/// The Deskulpt mocker.
pub struct Mocker {
    app: App<MockRuntime>,
    data_dir: TempDir,
}

impl Mocker {
    /// Get a handle to the mock application.
    pub fn handle(&self) -> &AppHandle<MockRuntime> {
        self.app.handle()
    }

    /// Absolutize a relative path within the mock widgets directory.
    pub fn widgets_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.data_dir.path().join("widgets").join(path).clean()
    }
}
