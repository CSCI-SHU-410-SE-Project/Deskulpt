//! Deskulpt path utilities.

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use tauri::{App, AppHandle, Manager, Runtime};

/// Thread-safe lazily-initialized static for the widgets directory.
static WIDGETS_DIR: OnceCell<Arc<PathBuf>> = OnceCell::new();

/// Thread-safe lazily-initialized static for the persistence directory.
static PERSIST_DIR: OnceCell<Arc<PathBuf>> = OnceCell::new();

/// Extension trait for path-related operations.
pub trait PathExt<R: Runtime>: Manager<R> {
    /// Initialize the widgets directory.
    ///
    /// This will create the widgets directory if it does not exist. It must be
    /// called before calling the [`widgets_dir`](PathExt::widgets_dir) method.
    fn init_widgets_dir(&self) -> Result<()> {
        let widgets_dir = WIDGETS_DIR.get_or_init(|| {
            let widgets_base_dir = if cfg!(debug_assertions) {
                self.path().resource_dir().unwrap()
            } else {
                self.path().document_dir().unwrap().join("Deskulpt")
            };
            let widgets_base_dir = dunce::simplified(&widgets_base_dir);
            Arc::new(widgets_base_dir.join("widgets"))
        });

        if !widgets_dir.exists() {
            create_dir_all(widgets_dir.as_ref())?;
        }
        Ok(())
    }

    /// Get a reference to the widgets directory.
    ///
    /// This will create the widgets directory if it does not exist, which can
    /// happen if one removes that directory during the application. This will
    /// error if the [`init_widgets_dir`](PathExt::init_widgets_dir) method has
    /// not been called.
    fn widgets_dir(&self) -> Result<&Path> {
        let widgets_dir = WIDGETS_DIR
            .get()
            .ok_or_else(|| anyhow!("`init_widgets_dir` must be called first"))?
            .as_path();

        if !widgets_dir.exists() {
            create_dir_all(widgets_dir)?;
        }
        Ok(widgets_dir)
    }

    /// Initialize the persistence directory.
    ///
    /// This will create the persistence directory if it does not exist. It must
    /// be called before calling the [`persist_dir`](PathExt::persist_dir)
    /// method.
    fn init_persist_dir(&self) -> Result<()> {
        let persist_dir = PERSIST_DIR.get_or_init(|| {
            let persist_dir = self.path().app_local_data_dir().unwrap();
            Arc::new(persist_dir)
        });

        if !persist_dir.exists() {
            create_dir_all(persist_dir.as_ref())?;
        }
        Ok(())
    }

    /// Get a reference to the persistence directory.
    ///
    /// This will create the persistence directory if it does not exist, which
    /// can happen if one removes that directory during the application. This
    /// will error if the [`init_persist_dir`](PathExt::init_persist_dir) method
    /// has not been called.
    fn persist_dir(&self) -> Result<&Path> {
        let persist_dir = PERSIST_DIR
            .get()
            .ok_or_else(|| anyhow!("`init_persist_dir` must be called first"))?
            .as_path();

        if !persist_dir.exists() {
            create_dir_all(persist_dir)?;
        }
        Ok(persist_dir)
    }
}

impl<R: Runtime> PathExt<R> for App<R> {}
impl<R: Runtime> PathExt<R> for AppHandle<R> {}
