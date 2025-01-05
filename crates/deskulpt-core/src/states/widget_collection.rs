//! State management for the widget collection.

use std::sync::Mutex;

use tauri::{App, AppHandle, Manager, Runtime};

use crate::config::WidgetCollection;

/// Managed state for the widget collection.
#[derive(Default)]
struct WidgetCollectionState(Mutex<WidgetCollection>);

/// Extension trait for operations on widget collection state.
pub trait StatesExtWidgetCollection {
    /// Initialize state management for the widget collection.
    fn manage_widget_collection(&self);

    /// Provide reference to the widget collection within a closure.
    ///
    /// This will lock the widget collection state. The return value of the
    /// closure will be propagated.
    fn with_widget_collection<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&WidgetCollection) -> T;

    /// Provide mutable reference to the widget collection within a closure.
    ///
    /// This will lock the widget collection state. The return value of the
    /// closure will be propagated.
    fn with_widget_collection_mut<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut WidgetCollection) -> T;
}

/// Shared implementation of [`StatesExtWidgetCollection`].
macro_rules! shared_impl {
    ($app: ty) => {
        impl<R: Runtime> StatesExtWidgetCollection for $app {
            fn manage_widget_collection(&self) {
                self.manage(WidgetCollectionState::default());
            }

            fn with_widget_collection<F, T>(&self, f: F) -> T
            where
                F: FnOnce(&WidgetCollection) -> T,
            {
                let state = self.state::<WidgetCollectionState>();
                let widget_collection = state.0.lock().unwrap();
                f(&widget_collection)
            }

            fn with_widget_collection_mut<F, T>(&self, f: F) -> T
            where
                F: FnOnce(&mut WidgetCollection) -> T,
            {
                let state = self.state::<WidgetCollectionState>();
                let mut widget_collection = state.0.lock().unwrap();
                f(&mut widget_collection)
            }
        }
    };
}

shared_impl!(App<R>);
shared_impl!(AppHandle<R>);
