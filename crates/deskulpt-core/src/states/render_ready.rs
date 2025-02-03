//! State management for whether the canvas is ready to render widgets.

use std::sync::Mutex;

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};

use crate::EventsExt;

/// Managed state for whether the canvas is ready to render widgets.
///
/// The first parameter indicates whether the canvas is ready to render widgets.
/// The second parameter is an optional pending payload for the initial render
/// widgets event. In particular, the manager window will emit an event on
/// startup to the canvas to render widgets initially. However, the canvas
/// may not be ready to receive the event at that time. In this case, we need to
/// store the payload and emit it later when the canvas is ready.
#[derive(Default)]
struct RenderReadyState(Mutex<(bool, Option<serde_json::Value>)>);

/// Extension trait for operations related to render readiness.
pub trait StatesExtRenderReady<R: Runtime>: Manager<R> + EventsExt<R> {
    /// Initialize state management for render readiness.
    fn manage_render_ready(&self) {
        self.manage(RenderReadyState::default());
    }

    /// Set the canvas as ready to render widgets.
    ///
    /// If there is a pending payload, emit the render event with that payload
    /// to the canvas.
    fn set_render_ready(&self) -> Result<()> {
        let state = self.state::<RenderReadyState>();
        let mut render_ready = state.0.lock().unwrap();
        render_ready.0 = true;

        if let Some(payload) = render_ready.1.take() {
            self.emit_render_widgets_to_canvas(payload)?;
        }
        Ok(())
    }

    /// Emit a render event to the canvas when it is ready.
    ///
    /// If the canvas is already ready to render widgets, emit the given payload
    /// to the canvas immediately. Otherwise, store the payload as pending so
    /// that it can be emitted later when the canvas is ready.
    fn emit_on_render_ready(&self, payload: serde_json::Value) -> Result<()> {
        let state = self.state::<RenderReadyState>();
        let mut render_ready = state.0.lock().unwrap();

        if !render_ready.0 {
            render_ready.1 = Some(payload);
            return Ok(());
        }
        self.emit_render_widgets_to_canvas(payload)
    }
}

impl<R: Runtime> StatesExtRenderReady<R> for App<R> {}
impl<R: Runtime> StatesExtRenderReady<R> for AppHandle<R> {}
