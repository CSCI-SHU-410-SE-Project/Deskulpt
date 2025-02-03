//! State management for whether the `render` listener is ready.

use std::sync::Mutex;

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};

use crate::EventsExt;

/// Managed state for whether the `render` listener is ready.
///
/// The first parameter indicates whether the listener (on the canvas window)
/// for the `render` event is ready. The second parameter is an optional pending
/// payload for the `render` event. In particular, the manager window will emit
/// a `render` event on startup, at which point the listener may not be ready
/// yet. In this case we need to store the payload and emit it later when the
/// listener is ready.
#[derive(Default)]
struct RenderReadyState(Mutex<(bool, Option<serde_json::Value>)>);

/// Extension trait for operations related to the `render` listener readiness.
pub trait StatesExtRenderReady<R: Runtime>: Manager<R> + EventsExt<R> {
    /// Initialize state management for whether the `render` listener is ready.
    fn manage_render_ready(&self) {
        self.manage(RenderReadyState::default());
    }

    /// Set the `render` listener as ready.
    ///
    /// If there is a pending payload, emit a `render` event with that payload
    /// to the canvas.
    fn set_render_ready(&self) -> Result<()> {
        let state = self.state::<RenderReadyState>();
        let mut render_ready = state.0.lock().unwrap();
        render_ready.0 = true;

        if let Some(payload) = render_ready.1.take() {
            self.emit_render_to_canvas(payload)?;
        }
        Ok(())
    }

    /// Emit the `render` event to the canvas when the listener is ready.
    ///
    /// If the `render` listener is not ready, store the given payload as
    /// pending so that it can be emitted later when the listener is ready.
    /// Otherwise, emit a `render` event with the given payload to the canvas
    /// immediately.
    fn emit_on_render_ready(&self, payload: serde_json::Value) -> Result<()> {
        let state = self.state::<RenderReadyState>();
        let mut render_ready = state.0.lock().unwrap();

        if !render_ready.0 {
            render_ready.1 = Some(payload);
            return Ok(());
        }
        self.emit_render_to_canvas(payload)
    }
}

impl<R: Runtime> StatesExtRenderReady<R> for App<R> {}
impl<R: Runtime> StatesExtRenderReady<R> for AppHandle<R> {}
