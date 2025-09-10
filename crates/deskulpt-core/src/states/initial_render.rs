//! State management for the initial render.

use std::sync::Mutex;

use anyhow::Result;
use tauri::{App, AppHandle, Emitter, Manager, Runtime};
use tauri_specta::Event;

use crate::events::RenderWidgetsEvent;
use crate::window::DeskulptWindow;

/// Inner structure for [`InitialRenderState`].
#[derive(Default)]
struct InitialRenderStateInner {
    /// Whether the canvas is ready to render widgets.
    ready: bool,
    /// Pending event to emit when the canvas is ready.
    ///
    /// If the manager window wants to emit a [`RenderWidgetsEvent`] to the
    /// canvas window when it is not ready to render widgets yet, the message
    /// will be stored in this field and emitted later when the canvas is ready.
    pending: Option<RenderWidgetsEvent>,
}

/// Managed state for the initial render.
///
/// The first parameter indicates whether the canvas is ready to render widgets,
/// i.e., whether the listener for the `render-widgets` event has been set up.
/// The second parameter is an optional pending payload for that event. In
/// particular, the manager window will that event to the canvas on startup, but
/// the canvas may not be ready to receive the event at that point. In this
/// case, we need to store the payload and emit it later when the canvas is
/// ready.
#[derive(Default)]
struct InitialRenderState(Mutex<InitialRenderStateInner>);

/// Extension trait for operations related to the initial render.
pub trait InitialRenderStateExt<R: Runtime>: Manager<R> + Emitter<R> + Sized {
    /// Initialize state management for the initial render.
    fn manage_initial_render(&self) {
        self.manage(InitialRenderState::default());
    }

    /// Set the canvas as ready to render widgets.
    ///
    /// If there is a pending payload, a [`RenderWidgetsEvent`] will be emitted
    /// to the canvas with that payload.
    fn set_render_ready(&self) -> Result<()> {
        let state = self.state::<InitialRenderState>();
        let mut initial_render = state.0.lock().unwrap();
        initial_render.ready = true;

        if let Some(event) = initial_render.pending.take() {
            event.emit_to(self, DeskulptWindow::Canvas)?;
        }
        Ok(())
    }

    /// Emit a [`RenderWidgetsEvent`] to the canvas when it is ready.
    ///
    /// If the canvas is already ready to render widgets, emit the given payload
    /// to the canvas immediately. Otherwise, store the payload as pending so
    /// that it can be emitted later when the canvas is ready.
    fn emit_on_render_ready(&self, event: RenderWidgetsEvent) -> Result<()> {
        let state = self.state::<InitialRenderState>();
        let mut initial_render = state.0.lock().unwrap();

        if !initial_render.ready {
            initial_render.pending = Some(event);
            return Ok(());
        }
        event.emit_to(self, DeskulptWindow::Canvas)?;
        Ok(())
    }
}

impl<R: Runtime> InitialRenderStateExt<R> for App<R> {}
impl<R: Runtime> InitialRenderStateExt<R> for AppHandle<R> {}
