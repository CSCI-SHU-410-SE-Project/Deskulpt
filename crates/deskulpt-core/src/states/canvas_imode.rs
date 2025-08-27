//! State management for canvas interaction mode.

use std::sync::Mutex;

use anyhow::{bail, Result};
use tauri::{App, AppHandle, Manager, Runtime, WebviewWindow};

use crate::events::{EventsExt, ShowToastPayload};
use crate::settings::CanvasImode;
use crate::tray::MenuItems;

/// The internal of the managed state for canvas interaction mode.
struct CanvasImodeStateInner<R: Runtime> {
    /// The canvas window.
    canvas: Option<WebviewWindow<R>>,
    /// The tray menu items.
    menu_items: Option<MenuItems<R>>,
}

/// Managed state for canvas interaction mode.
struct CanvasImodeState<R: Runtime>(Mutex<CanvasImodeStateInner<R>>);

/// Extension trait for operations on canvas interaction mode.
pub trait StatesExtCanvasImode<R: Runtime>: Manager<R> + EventsExt<R> {
    /// Initialize state management for canvas interaction mode.
    ///
    /// The canvas is in sink mode by default.
    fn manage_canvas_imode(&self) {
        let inner = CanvasImodeStateInner {
            canvas: None::<WebviewWindow<R>>,
            menu_items: None::<MenuItems<R>>,
        };
        self.manage(CanvasImodeState(Mutex::new(inner)));
    }

    fn post_manage_canvas_imode(&self, menu_items: MenuItems<R>) {
        let state = self.state::<CanvasImodeState<R>>();
        let mut state = state.0.lock().unwrap();
        state.canvas = Some(
            self.get_webview_window("canvas")
                .expect("Canvas window not found"),
        );
        state.menu_items = Some(menu_items);
    }

    fn set_canvas_imode(&self, mode: CanvasImode) -> Result<()> {
        let state = self.state::<CanvasImodeState<R>>();
        let state = state.0.lock().unwrap();
        if state.canvas.is_none() || state.menu_items.is_none() {
            bail!(
                "Canvas interaction mode state is not properly initialized; \
                 post_manage_canvas_imode must be called first"
            );
        }
        // Safe to unwrap because we have already checked for None
        let canvas = state.canvas.as_ref().unwrap();
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
        if let Err(e) = self.emit_show_toast_to_canvas(ShowToastPayload::Success(toast_message)) {
            eprintln!("Failed to emit show-toast to canvas: {}", e);
        }

        Ok(())
    }
}

impl<R: Runtime> StatesExtCanvasImode<R> for App<R> {}
impl<R: Runtime> StatesExtCanvasImode<R> for AppHandle<R> {}
