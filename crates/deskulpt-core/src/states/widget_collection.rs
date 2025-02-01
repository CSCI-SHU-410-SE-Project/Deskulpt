//! State management for the widget collection.

use std::sync::RwLock;

use tauri::{App, AppHandle, Manager, Runtime};

use crate::config::WidgetCollection;

/// Managed state for the widget collection.
#[derive(Default)]
struct WidgetCollectionState(RwLock<WidgetCollection>);

/// Extension trait for operations on widget collection state.
pub trait StatesExtWidgetCollection<R: Runtime>: Manager<R> {
    /// Initialize state management for the widget collection.
    fn manage_widget_collection(&self) {
        self.manage(WidgetCollectionState::default());
    }

    /// Provide reference to the widget collection within a closure.
    ///
    /// This will lock the widget collection state. The return value of the
    /// closure will be propagated.
    fn with_widget_collection<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&WidgetCollection) -> T,
    {
        let state = self.state::<WidgetCollectionState>();
        let widget_collection = state.0.read().unwrap();
        f(&widget_collection)
    }

    /// Provide mutable reference to the widget collection within a closure.
    ///
    /// This will lock the widget collection state. The return value of the
    /// closure will be propagated.
    fn with_widget_collection_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut WidgetCollection) -> T,
    {
        let state = self.state::<WidgetCollectionState>();
        let mut widget_collection = state.0.write().unwrap();
        f(&mut widget_collection)
    }
}

impl<R: Runtime> StatesExtWidgetCollection<R> for App<R> {}
impl<R: Runtime> StatesExtWidgetCollection<R> for AppHandle<R> {}
