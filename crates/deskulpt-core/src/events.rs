//! Deskulpt core events.

use deskulpt_common::event::Event;
use serde::{Deserialize, Serialize};

use crate::settings::{Theme, WidgetSettings};

/// Event for exiting the application.
///
/// This event is emitted from the backend to the manager window when the
/// application needs to be closed for it to persist the states before exiting.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct ExitAppEvent;

/// Event for removing widgets.
///
/// This event is emitted from the manager window to the canvas window when
/// widgets need to be removed.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RemoveWidgetsEvent(
    /// The list of widget IDs to be removed.
    Vec<String>,
);

/// Inner structure for [`RenderWidgetsEvent`].
#[derive(Clone, Serialize, Deserialize, specta::Type)]
pub struct RenderWidgetsEventInner {
    /// The ID of the widget being re-rendered.
    id: String,
    /// If provided, update the settings of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = WidgetSettings)]
    settings: Option<WidgetSettings>,
    /// If provided, update the code of the widget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    code: Option<String>,
}

/// Event for re-rendering widgets.
///
/// This event is mainly emitted from the manager window to the canvas window
/// when settings or code of a widget needs to be re-rendered. It may also be
/// emitted from the backend to the canvas window for the initial render.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct RenderWidgetsEvent(
    /// The list of widgets to be re-rendered.
    Vec<RenderWidgetsEventInner>,
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

/// Event for switching the app theme.
///
/// This event is emitted from the manager window to the canvas window when the
/// theme is switched from the manager side.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct SwitchThemeEvent(
    /// The theme to switch to.
    Theme,
);

/// Event for updating settings of a widget.
///
/// This event is emitted between the manager window and the canvas window to
/// each other when widget settings are updated on one side.
#[derive(Clone, Serialize, Deserialize, specta::Type, Event)]
pub struct UpdateSettingsEvent {
    /// The ID of the widget being updated.
    id: String,
    /// [`WidgetSettings::x`](crate::settings::WidgetSettings::x)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = i32)]
    x: Option<i32>,
    /// [`WidgetSettings::y`](crate::settings::WidgetSettings::y)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = i32)]
    y: Option<i32>,
    /// [`WidgetSettings::opacity`](crate::settings::WidgetSettings::opacity)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = i32)]
    opacity: Option<i32>,
}
