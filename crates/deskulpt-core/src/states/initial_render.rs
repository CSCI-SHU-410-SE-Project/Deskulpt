//! State management for the initial render.

use std::sync::Mutex;

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};

use crate::EventsExt;

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
struct InitialRenderState(Mutex<(bool, Option<serde_json::Value>)>);

/// Extension trait for operations related to the initial render.
pub trait StatesExtInitialRender<R: Runtime>: Manager<R> + EventsExt<R> {
    /// Initialize state management for the initial render.
    fn manage_initial_render(&self) {
        self.manage(InitialRenderState::default());
    }

    /// Set the canvas as ready to render widgets.
    ///
    /// If there is a pending payload, a `render-widgets` event will be emitted
    /// to the canvas with that payload.
    fn set_render_ready(&self) -> Result<()> {
        let state = self.state::<InitialRenderState>();
        let mut render_ready = state.0.lock().unwrap();
        render_ready.0 = true;

        if let Some(payload) = render_ready.1.take() {
            self.emit_render_widgets_to_canvas(payload)?;
        }
        Ok(())
    }

    /// Emit a `render-widgets` event to the canvas when it is ready.
    ///
    /// If the canvas is already ready to render widgets, emit the given payload
    /// to the canvas immediately. Otherwise, store the payload as pending so
    /// that it can be emitted later when the canvas is ready.
    fn emit_on_render_ready(&self, payload: serde_json::Value) -> Result<()> {
        let state = self.state::<InitialRenderState>();
        let mut render_ready = state.0.lock().unwrap();

        if !render_ready.0 {
            render_ready.1 = Some(payload);
            return Ok(());
        }
        self.emit_render_widgets_to_canvas(payload)
    }
}

impl<R: Runtime> StatesExtInitialRender<R> for App<R> {}
impl<R: Runtime> StatesExtInitialRender<R> for AppHandle<R> {}
