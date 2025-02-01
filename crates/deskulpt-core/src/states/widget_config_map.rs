//! State management for the widget configuration map.

use std::collections::HashMap;
use std::sync::RwLock;

use tauri::{App, AppHandle, Manager, Runtime};

use crate::config::WidgetConfig;

/// Managed state for the widget configuration map.
#[derive(Default)]
struct WidgetConfigMapState(RwLock<HashMap<String, WidgetConfig>>);

/// Extension trait for operations on widget configuration map state.
pub trait StatesExtWidgetConfigMap<R: Runtime>: Manager<R> {
    /// Initialize state management for the widget configuration map.
    fn manage_widget_config_map(&self) {
        self.manage(WidgetConfigMapState::default());
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

impl<R: Runtime> StatesExtWidgetConfigMap<R> for App<R> {}
impl<R: Runtime> StatesExtWidgetConfigMap<R> for AppHandle<R> {}
