//! State management for whether windows are ready.

use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use tauri::{App, AppHandle, Manager, Runtime};

use crate::EventsExt;

/// Managed state for whether windows are ready.
///
/// The two booleans represent whether the canvas window and manager window are
/// ready, respectively.
#[derive(Default)]
struct WindowReadyState(AtomicBool, AtomicBool);

/// Extension trait for operations on window ready state.
pub trait StatesExtWindowReady<R: Runtime>: Manager<R> + EventsExt<R> {
    /// Initialize state management for whether windows are ready.
    fn manage_window_ready(&self) {
        self.manage(WindowReadyState::default());
    }

    /// Set that the canvas window is ready.
    ///
    /// If the manager window is also ready, emit the `window-ready` event.
    fn set_canvas_window_ready(&self) -> Result<()> {
        let state = self.state::<WindowReadyState>();
        state.0.store(true, Ordering::Relaxed);

        if state.1.load(Ordering::Relaxed) {
            self.emit_window_ready()?;
        }
        Ok(())
    }

    /// Set that the manager window is ready.
    ///
    /// If the canvas window is also ready, emit the `window-ready` event.
    fn set_manager_window_ready(&self) -> Result<()> {
        let state = self.state::<WindowReadyState>();
        state.1.store(true, Ordering::Relaxed);

        if state.0.load(Ordering::Relaxed) {
            self.emit_window_ready()?;
        }
        Ok(())
    }
}

impl<R: Runtime> StatesExtWindowReady<R> for App<R> {}
impl<R: Runtime> StatesExtWindowReady<R> for AppHandle<R> {}
