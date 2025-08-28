//! State management for the widget configuration map.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::{anyhow, Result};
use tauri::{App, AppHandle, Manager, Runtime};

use crate::config::WidgetConfig;
use crate::PathExt;

/// Managed state for the widget configuration map.
#[derive(Default)]
struct WidgetConfigMapState(RwLock<HashMap<String, WidgetConfig>>);

/// Extension trait for operations on widget configuration map state.
pub trait WidgetConfigMapStatesExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Initialize state management for the widget configuration map.
    fn manage_widget_config_map(&self) {
        self.manage(WidgetConfigMapState::default());
    }

    /// Get the directory of a widget by ID.
    ///
    /// This will error if the widgets directory cannot be accessed or if the
    /// widget is not found in the collection.
    fn widget_dir<S: AsRef<str>>(&self, id: S) -> Result<PathBuf> {
        let widgets_dir = self.widgets_dir()?;
        let id = id.as_ref();

        let state = self.state::<WidgetConfigMapState>();
        let widget_config_map = state.0.read().unwrap();
        let widget_config = widget_config_map
            .get(id)
            .ok_or_else(|| anyhow!("Widget {id} not found in the collection"))?;

        Ok(widgets_dir.join(widget_config.dir()))
    }

    /// Provide reference to the widget configuration map within a closure.
    ///
    /// This will lock the widget configuration map state. The return value of
    /// the closure will be propagated.
    fn with_widget_config_map<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&HashMap<String, WidgetConfig>) -> T,
    {
        let state = self.state::<WidgetConfigMapState>();
        let widget_config_map = state.0.read().unwrap();
        f(&widget_config_map)
    }

    /// Provide mutable reference to the widget configuration map within a
    /// closure.
    ///
    /// This will lock the widget configuration map state. The return value of
    /// the closure will be propagated.
    fn with_widget_config_map_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut HashMap<String, WidgetConfig>) -> T,
    {
        let state = self.state::<WidgetConfigMapState>();
        let mut widget_config_map = state.0.write().unwrap();
        f(&mut widget_config_map)
    }
}

impl<R: Runtime> WidgetConfigMapStatesExt<R> for App<R> {}
impl<R: Runtime> WidgetConfigMapStatesExt<R> for AppHandle<R> {}
