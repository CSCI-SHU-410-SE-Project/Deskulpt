//! Event system for IPC between Deskulpt frontend and backend.

use anyhow::Result;
use deskulpt_macros::{register_deskulpt_events, DeskulptEvent};
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Runtime};

use crate::settings::WidgetSettings;
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
    /// The ID of the widget being rendered.
    id: String,
    /// If provided, update the settings of the widget.
    settings: Option<WidgetSettings>,
    /// If provided, update the code of the widget.
    code: Option<String>,
}

/// Event for the initial render of the application.
///
/// This event is emitted from the backend to the canvas window when we are sure
/// that it is ready for the initial render.
#[derive(Serialize, Deserialize, ts_rs::TS, DeskulptEvent)]
#[ts(export, export_to = "types.ts")]
pub struct RenderWidgetsEvent(Vec<RenderWidgetsEventInner>);

register_deskulpt_events![ShowToastEvent, ExitAppEvent, RenderWidgetsEvent];
