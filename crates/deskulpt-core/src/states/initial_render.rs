//! State management for the initial render.

use std::sync::Mutex;

use anyhow::Result;
use tauri::{App, AppHandle, Emitter, Manager, Runtime};

use crate::events::{DeskulptEvent, RenderWidgetsEvent};
use crate::window::DeskulptWindow;

/// Inner structure for [`InitialRenderState`].
#[derive(Default)]
struct InitialRenderStateInner {
    /// Whether the canvas is ready to render widgets.
    is_ready: bool,
    /// Pending event to emit when the canvas is ready.
    ///
    /// If the manager window wants to emit a [`RenderWidgetsEvent`] to the
    /// canvas window when it is not ready to render widgets yet, the message
    /// will be stored in this field and emitted later when the canvas is ready.
    pending: Option<RenderWidgetsEvent>,
}

/// Managed state for the initial render.
#[derive(Default)]
struct InitialRenderState(Mutex<InitialRenderStateInner>);

/// Extension trait for operations related to the initial render.
pub trait StatesExtInitialRender<R: Runtime>: Manager<R> + Emitter<R> {
    /// Initialize state management for the initial render.
    fn manage_initial_render(&self) {
        self.manage(InitialRenderState::default());
    }

    /// Set the canvas as ready to render widgets.
    ///
    /// If there is a [`RenderWidgetsEvent`], it will be emitted to the canvas.
    fn set_render_ready(&self) -> Result<()> {
        let state = self.state::<InitialRenderState>();
        let mut initial_render = state.0.lock().unwrap();
        initial_render.is_ready = true;

        if let Some(event) = initial_render.pending.take() {
            event.emit_to(self, DeskulptWindow::Canvas)?;
        }
        Ok(())
    }

    /// Emit a [`RenderWidgetsEvent`] to the canvas when it is ready.
    ///
    /// If the canvas is already ready to render widgets, emit the given event
    /// to the canvas immediately. Otherwise, store the event as pending so it
    /// can be emitted later when the canvas is ready.
    fn emit_on_render_ready(&self, event: RenderWidgetsEvent) -> Result<()> {
        let state = self.state::<InitialRenderState>();
        let mut initial_render = state.0.lock().unwrap();

        if !initial_render.is_ready {
            initial_render.pending = Some(event);
            return Ok(());
        }
        event.emit_to(self, DeskulptWindow::Canvas)
    }
}

impl<R: Runtime> StatesExtInitialRender<R> for App<R> {}
impl<R: Runtime> StatesExtInitialRender<R> for AppHandle<R> {}
