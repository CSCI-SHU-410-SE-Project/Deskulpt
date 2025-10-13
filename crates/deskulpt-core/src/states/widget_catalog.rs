//! State management for the widget catalog.

use std::collections::HashMap;
use std::sync::RwLock;

use tauri::{App, AppHandle, Manager, Runtime};

use crate::config::WidgetConfig;
use crate::path::PathExt;

/// Collection of widgets discovered by the application.
#[derive(Default, Clone)]
pub struct WidgetCatalog {
    configs: HashMap<String, WidgetConfig>,
}

impl WidgetCatalog {
    /// Create a new empty widget catalog.
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    /// Borrow the underlying configuration map.
    pub fn configs(&self) -> &HashMap<String, WidgetConfig> {
        &self.configs
    }

    /// Mutably borrow the underlying configuration map.
    pub fn configs_mut(&mut self) -> &mut HashMap<String, WidgetConfig> {
        &mut self.configs
    }

    /// Look up configuration for a widget by ID.
    pub fn get(&self, id: &str) -> Option<&WidgetConfig> {
        self.configs.get(id)
    }
}

impl From<HashMap<String, WidgetConfig>> for WidgetCatalog {
    fn from(configs: HashMap<String, WidgetConfig>) -> Self {
        Self { configs }
    }
}

/// Managed state for the widget catalog.
#[derive(Default)]
struct WidgetCatalogState(RwLock<WidgetCatalog>);

/// Extension trait for operations on widget catalog state.
pub trait WidgetCatalogStateExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Initialize state management for the widget catalog.
    fn manage_widget_catalog(&self) {
        self.manage(WidgetCatalogState::default());
    }

    /// Provide reference to the widget catalog within a closure.
    ///
    /// This will lock the widget catalog state. The return value of the closure
    /// will be propagated.
    fn with_widget_catalog<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&WidgetCatalog) -> T,
    {
        let state = self.state::<WidgetCatalogState>();
        let catalog = state.0.read().unwrap();
        f(&catalog)
    }

    /// Provide mutable reference to the widget catalog within a closure.
    ///
    /// This will lock the widget catalog state. The return value of the closure
    /// will be propagated.
    fn with_widget_catalog_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut WidgetCatalog) -> T,
    {
        let state = self.state::<WidgetCatalogState>();
        let mut catalog = state.0.write().unwrap();
        f(&mut catalog)
    }
}

impl<R: Runtime> WidgetCatalogStateExt<R> for App<R> {}
impl<R: Runtime> WidgetCatalogStateExt<R> for AppHandle<R> {}
