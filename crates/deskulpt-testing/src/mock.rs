//! Mocking utilities for Deskulpt components.

use std::fs::create_dir;
use std::path::{Path, PathBuf};

use copy_dir::copy_dir;
use deskulpt_test_states::StatesExt;
use path_clean::PathClean;
use tauri::test::{mock_app, MockRuntime};
use tauri::{App, AppHandle};
use tempfile::{tempdir, TempDir};

/// Builder for the Deskulpt mocker.
#[derive(Default)]
pub struct MockerBuilder {
    widgets_dir: Option<PathBuf>,
}

impl MockerBuilder {
    /// Copy the contents of a directory to the mock widgets directory.
    pub fn with_widgets_dir<P: AsRef<Path>>(mut self, widgets_dir: P) -> Self {
        self.widgets_dir = Some(widgets_dir.as_ref().to_path_buf());
        self
    }

    /// Build a new mocker instance.
    ///
    /// The mock application will be set up with mock directories and states.
    pub fn build(&self) -> Mocker {
        let app = mock_app();
        let resource_dir = tempdir().expect("Failed to create temporary directory");

        let widgets_dir = resource_dir.path().join("widgets");
        if let Some(widgets_dir_src) = &self.widgets_dir {
            copy_dir(widgets_dir_src, &widgets_dir).expect("Failed to copy widgets directory");
        } else {
            create_dir(&widgets_dir).expect("Failed to create widgets directory");
        }

        let app_handle = app.handle();
        app_handle.manage_widget_collection();
        app_handle.manage_widgets_dir_at(widgets_dir);
        app_handle.manage_canvas_click_through();

        Mocker { app, resource_dir }
    }
}

/// The Deskulpt mocker.
pub struct Mocker {
    app: App<MockRuntime>,
    resource_dir: TempDir,
}

impl Mocker {
    /// Get a handle to the mock application.
    pub fn handle(&self) -> &AppHandle<MockRuntime> {
        self.app.handle()
    }

    /// Absolutize a relative path within the mock widgets directory.
    pub fn widgets_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.resource_dir.path().join("widgets").join(path).clean()
    }
}
