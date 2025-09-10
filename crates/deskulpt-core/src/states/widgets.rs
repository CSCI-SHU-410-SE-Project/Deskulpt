//! State management for the widgets.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use anyhow::{anyhow, Result};
use tauri::{App, AppHandle, Manager, Runtime};

use crate::path::PathExt;
use crate::widgets::Widget;

/// Managed state for the widgets.
#[derive(Default)]
struct WidgetsState(RwLock<BTreeMap<String, Widget>>);

/// Extension trait for operations on widgets state.
pub trait WidgetsStateExt<R: Runtime>: Manager<R> + PathExt<R> {
    /// Initialize state management for the widgets.
    fn manage_widgets(&self) {
        self.manage(WidgetsState::default());
    }

    /// Get the directory of a widget by ID.
    ///
    /// This will error if the widgets directory cannot be accessed or if the
    /// widget is not found in the collection.
    fn widget_dir<S: AsRef<str>>(&self, id: S) -> Result<PathBuf> {
        let widgets_dir = self.widgets_dir()?;
        let id = id.as_ref();

        let state = self.state::<WidgetsState>();
        let widgets = state.0.read().unwrap();
        let widget = widgets
            .get(id)
            .ok_or_else(|| anyhow!("Widget {id} not found in the collection"))?;

        Ok(widgets_dir.join(widget.dir()))
    }

    /// Get an immutable reference to the widgets.
    fn get_widgets(&self) -> RwLockReadGuard<'_, BTreeMap<String, Widget>> {
        let state = self.state::<WidgetsState>().inner();
        state.0.read().unwrap()
    }

    /// Get a mutable reference to the widgets.
    fn get_widgets_mut(&self) -> RwLockWriteGuard<'_, BTreeMap<String, Widget>> {
        let state = self.state::<WidgetsState>().inner();
        state.0.write().unwrap()
    }
}

impl<R: Runtime> WidgetsStateExt<R> for App<R> {}
impl<R: Runtime> WidgetsStateExt<R> for AppHandle<R> {}
