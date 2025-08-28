//! Deskulpt core events.

use anyhow::Result;
use deskulpt_macros::{register_deskulpt_events, DeskulptEvent};
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Runtime};

use crate::settings::{Theme, WidgetSettings};
use crate::window::DeskulptWindow;

/// Trait for all Deskulpt events.
pub trait DeskulptEvent: Serialize {
    /// The name of the event.
    const NAME: &'static str;

    /// Emit the event to all targets.
    fn emit<R, M>(&self, manager: &M) -> Result<()>
    where
        R: Runtime,
        M: Emitter<R> + ?Sized,
    {
        manager.emit(Self::NAME, self)?;
        Ok(())
    }

    /// Emit the event to a specific Deskulpt window.
    fn emit_to<R, M>(&self, manager: &M, window: DeskulptWindow) -> Result<()>
    where
        R: Runtime,
        M: Emitter<R> + ?Sized,
    {
        manager.emit_to(window.label(), Self::NAME, self)?;
        Ok(())
    }
}

/// Event for showing a toast notification.
///
/// This event is emitted from the backend to the canvas window when a toast
/// notification needs to be displayed.
#[derive(Serialize, ts_rs::TS, DeskulptEvent)]
#[ts(export, export_to = "types.ts")]
#[serde(tag = "type", content = "content", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShowToastEvent {
    /// Show a [success](https://sonner.emilkowal.ski/toast#success) toast.
    Success(String),
    /// Show an [error](https://sonner.emilkowal.ski/toast#error) toast.
    Error(String),
}

/// Event for exiting the application.
///
/// This event is emitted from the backend to the manager window when the
/// application needs to be closed for it to persist the states before exiting.
#[derive(Serialize, ts_rs::TS, DeskulptEvent)]
#[ts(export, export_to = "types.ts")]
pub struct ExitAppEvent;

/// Inner structure for [`RenderWidgetsEvent`].
#[derive(Serialize, Deserialize, ts_rs::TS)]
#[ts(export_to = "types.ts")]
struct RenderWidgetsEventInner {
    /// The ID of the widget being re-rendered.
    id: String,
    /// If provided, update the settings of the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    settings: Option<WidgetSettings>,
    /// If provided, update the code of the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    code: Option<String>,
}

/// Event for re-rendering widgets.
///
/// This event is mainly emitted from the manager window to the canvas window
/// when settings or code of a widget needs to be re-rendered. It may also be
/// emitted from the backend to the canvas window for the initial render.
#[derive(Serialize, Deserialize, ts_rs::TS, DeskulptEvent)]
#[ts(export, export_to = "types.ts")]
pub struct RenderWidgetsEvent(
    /// The list of widgets to be re-rendered.
    Vec<RenderWidgetsEventInner>,
);

/// Event for removing widgets.
///
/// This event is emitted from the manager window to the canvas window when
/// widgets need to be removed.
#[derive(Serialize, ts_rs::TS, DeskulptEvent)]
#[ts(export, export_to = "types.ts")]
pub struct RemoveWidgetsEvent(
    /// The list of widget IDs to be removed.
    Vec<String>,
);

/// Event for switching the app theme.
///
/// This event is emitted from the manager window to the canvas window when the
/// theme is switched from the manager side.
#[derive(Serialize, ts_rs::TS, DeskulptEvent)]
#[ts(export, export_to = "types.ts")]
pub struct SwitchThemeEvent(
    /// The theme to switch to.
    Theme,
);

/// Event for updating settings of a widget.
///
/// This event is emitted between the manager window and the canvas window to
/// each other when widget settings are updated on one side.
#[derive(Serialize, ts_rs::TS, DeskulptEvent)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateSettingsEvent {
    /// The ID of the widget being updated.
    id: String,
    /// [`WidgetSettings::x`](crate::settings::WidgetSettings::x)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    x: Option<i32>,
    /// [`WidgetSettings::y`](crate::settings::WidgetSettings::y)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    y: Option<i32>,
    /// [`WidgetSettings::opacity`](crate::settings::WidgetSettings::opacity)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    opacity: Option<i32>,
}

register_deskulpt_events![
    ShowToastEvent,
    ExitAppEvent,
    RenderWidgetsEvent,
    RemoveWidgetsEvent,
    SwitchThemeEvent,
    UpdateSettingsEvent,
];
