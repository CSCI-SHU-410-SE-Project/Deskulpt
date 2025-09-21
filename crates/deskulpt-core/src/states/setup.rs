//! State management for application setup.

use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Mutex;

use serde::Deserialize;
use tauri::{App, AppHandle, Emitter, Manager, Runtime};

/// An application setup task.
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum SetupTask {
    /// Whether the canvas window is ready to listen to
    /// [`UpdateSettingsEvent`](crate::events::UpdateSettingsEvent).
    CanvasUpdateSettings,
    /// Whether the canvas window is ready to listen to
    /// [`RenderWidgetsEvent`](crate::events::RenderWidgetsEvent).
    CanvasRenderWidgets,
    /// Whether the canvas window is ready to listen to
    /// [`ShowToastEvent`](crate::events::ShowToastEvent).
    CanvasShowToast,
    /// Whether the manager window is ready to listen to
    /// [`UpdateSettingsEvent`](crate::events::UpdateSettingsEvent).
    ManagerUpdateSettings,
    /// Whether the manager window is ready to listen to
    /// [`UpdateWidgetConfigRegistryEvent`](crate::events::UpdateWidgetConfigRegistryEvent).
    ManagerUpdateWidgetConfigRegistry,
}

/// Managed state for application setup.
#[derive(Default)]
struct SetupState(Mutex<HashSet<SetupTask>>);

/// Extension trait for operations on the application setup state.
pub trait SetupStateExt<R: Runtime>: Manager<R> + Emitter<R> + Sized {
    /// Initialize state management for application setup.
    fn manage_setup(&self) {
        self.manage(SetupState::default());
    }

    fn mark_setup(&self, task: SetupTask) -> Option<bool> {
        let state = self.state::<SetupState>();
        let mut state = state.0.lock().unwrap();
        if state.len() == 5 {
            return None;
        }
        state.insert(task);
        return Some(state.len() == 5);
    }
}

impl<R: Runtime> SetupStateExt<R> for App<R> {}
impl<R: Runtime> SetupStateExt<R> for AppHandle<R> {}
