//! Deskulpt core events.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use tauri_specta::Event;

use crate::config::WidgetConfigRegistry;
use crate::settings::Settings;

/// TODO(Charlie-XIAO)
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RenderWidgetsEvent(
    /// The mapping from widget IDs to their respective bundled code.
    pub BTreeMap<String, String>,
);

/// Event for showing a toast notification.
///
/// This event is emitted from the backend to the canvas window when a toast
/// notification needs to be displayed.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum ShowToastEvent {
    /// Show a [success](https://sonner.emilkowal.ski/toast#success) toast.
    Success(String),
    /// Show an [error](https://sonner.emilkowal.ski/toast#error) toast.
    Error(String),
}

/// Event for updating the settings.
///
/// This event is emitted from the backend to all windows when the settings are
/// updated.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateSettingsEvent(
    /// The updated settings.
    pub Settings,
);

/// Event for updating the widget configuration registry.
///
/// This event is emitted from the backend to the manager window when the
/// widget configuration registry is updated.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateWidgetConfigRegistryEvent(
    /// The updated widget configuration registry.
    pub WidgetConfigRegistry,
);
