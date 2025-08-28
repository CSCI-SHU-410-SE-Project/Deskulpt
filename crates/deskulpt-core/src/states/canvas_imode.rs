//! State management for canvas interaction mode.

use std::sync::Mutex;

use anyhow::{bail, Result};
use tauri::{App, AppHandle, Emitter, Manager, Runtime};

use crate::events::{DeskulptEvent, ShowToastEvent};
use crate::settings::CanvasImode;
use crate::tray::MenuItems;
use crate::window::DeskulptWindow;

/// The internal of the managed state for canvas interaction mode.
struct CanvasImodeStateInner<R: Runtime> {
    /// The tray menu items.
    menu_items: Option<MenuItems<R>>,
}

/// Managed state for canvas interaction mode.
struct CanvasImodeState<R: Runtime>(Mutex<CanvasImodeStateInner<R>>);

/// Extension trait for operations on canvas interaction mode.
pub trait StatesExtCanvasImode<R: Runtime>: Manager<R> + Emitter<R> {
    /// Initialize state management for canvas interaction mode.
    ///
    /// The canvas is in sink mode by default.
    fn manage_canvas_imode(&self) {
        let inner = CanvasImodeStateInner {
            menu_items: None::<MenuItems<R>>,
        };
        self.manage(CanvasImodeState(Mutex::new(inner)));
    }

    fn post_manage_canvas_imode(&self, menu_items: MenuItems<R>) {
        let state = self.state::<CanvasImodeState<R>>();
        let mut state = state.0.lock().unwrap();
        state.menu_items = Some(menu_items);
    }

    fn set_canvas_imode(&self, mode: CanvasImode) -> Result<()> {
        let state = self.state::<CanvasImodeState<R>>();
        let state = state.0.lock().unwrap();
        if state.menu_items.is_none() {
            bail!(
                "Canvas interaction mode state is not properly initialized; \
                 post_manage_canvas_imode must be called first"
            );
        }
        let canvas = DeskulptWindow::Canvas.webview_window(self);
        let menu_items = state.menu_items.as_ref().unwrap();

        match mode {
            CanvasImode::Float => {
                canvas.set_ignore_cursor_events(false)?;
            },
            CanvasImode::Sink => {
                canvas.set_ignore_cursor_events(true)?;
                // Toggled from float to sink, so we try to regain focus to
                // avoid flickering on the first click; failure to do so is not
                // critical so we consume the error
                if let Err(e) = canvas.set_focus() {
                    eprintln!("Failed to gain focus on canvas: {}", e);
                }
            },
        }

        let toast_message = format!("Canvas: {mode:?}");
        menu_items.set_canvas_imode(mode)?;

        // Failure to emit toast is not critical so we consume the error
        if let Err(e) = ShowToastEvent::Success(toast_message).emit_to(self, DeskulptWindow::Canvas)
        {
            eprintln!("Failed to emit show-toast to canvas: {}", e);
        }

        Ok(())
    }
}

impl<R: Runtime> StatesExtCanvasImode<R> for App<R> {}
impl<R: Runtime> StatesExtCanvasImode<R> for AppHandle<R> {}
