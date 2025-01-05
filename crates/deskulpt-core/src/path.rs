//! Deskulpt path utilities.

use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::OnceCell;
use tauri::{App, AppHandle, Manager, Runtime};

/// Thread-safe lazily-initialized static for the widgets directory.
static WIDGETS_DIR: OnceCell<Arc<PathBuf>> = OnceCell::new();

/// Thread-safe lazily-initialized static for the persistent directory.
static PERSIST_DIR: OnceCell<Arc<PathBuf>> = OnceCell::new();

/// Extension trait for path-related operations.
pub trait PathExt<R: Runtime>: Manager<R> {
    /// Initialize the widgets directory.
    ///
    /// This will create the widgets directory if it does not exist. It must be
    /// called before calling [`PathExt::widgets_dir`].
    fn init_widgets_dir(&self) -> Result<()> {
        let widgets_dir = WIDGETS_DIR.get_or_init(|| {
            let resource_dir = self.path().resource_dir().unwrap();
            Arc::new(resource_dir.join("widgets"))
        });

        if !widgets_dir.exists() {
            create_dir_all(widgets_dir.as_ref())?;
        }
        Ok(())
    }

    /// Get a reference to the widgets directory.
    ///
    /// This will panic if [`PathExt::init_widgets_dir`] has not been called.
    fn widgets_dir(&self) -> &Path {
        WIDGETS_DIR
            .get()
            .expect("create_widgets_dir must be called first")
            .as_ref()
    }

    /// Initialize the persistent directory.
    ///
    /// This will create the persistent directory if it does not exist. It must
    /// be called before calling [`PathExt::persist_dir`].
    fn init_persist_dir(&self) -> Result<()> {
        let persist_dir = PERSIST_DIR.get_or_init(|| {
            let persist_dir = self.path().app_data_dir().unwrap();
            Arc::new(persist_dir)
        });

        if !persist_dir.exists() {
            create_dir_all(persist_dir.as_ref())?;
        }
        Ok(())
    }

    /// Get a reference to the persistent directory.
    ///
    /// This will panic if [`PathExt::init_persist_dir`] has not been called.
    fn persist_dir(&self) -> &Path {
        PERSIST_DIR
            .get()
            .expect("init_persist_dir must be called first")
            .as_ref()
    }
}

impl<R: Runtime> PathExt<R> for App<R> {}
impl<R: Runtime> PathExt<R> for AppHandle<R> {}
