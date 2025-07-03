//! State management for canvas click-through.

use std::sync::Mutex;

use anyhow::Result;
use tauri::menu::MenuItem;
use tauri::{App, AppHandle, Manager, Runtime};

use crate::events::{EventsExt, ShowToastPayload};

/// Managed state for canvas click-through.
///
/// Apart from the boolean indicating whether the canvas is click-through, this
/// state also carries the menu item for toggling the click-through state
/// because there is no direct way to access a specific menu item from the app
/// handle.
struct CanvasClickThroughState<R: Runtime>(Mutex<(bool, Option<MenuItem<R>>)>);

/// Extension trait for operations on canvas click-through state.
pub trait StatesExtCanvasClickThrough<R: Runtime>: Manager<R> + EventsExt<R> {
    /// Initialize state management for whether the canvas is click-through.
    ///
    /// The canvas is click-through by default.
    fn manage_canvas_click_through(&self) {
        self.manage(CanvasClickThroughState::<R>(Mutex::new((true, None))));
    }

    /// Set the menu item for toggling the canvas click-through state.
    fn set_canvas_click_through_menu_item(&self, menu_item: &MenuItem<R>) {
        let state = self.state::<CanvasClickThroughState<R>>();
        let mut canvas_click_through = state.0.lock().unwrap();

        // Cloning works because menu items are behind shared references
        canvas_click_through.1 = Some(menu_item.clone());
    }

    /// Toggle the click-through state of the canvas window.
    ///
    /// This will also update the menu item text and show a toast message on the
    /// canvas if possible.
    fn toggle_canvas_click_through(&self) -> Result<()> {
        let canvas = self
            .get_webview_window("canvas")
            .expect("Canvas window not found");

        let state = self.state::<CanvasClickThroughState<R>>();
        let mut canvas_click_through = state.0.lock().unwrap();
        let prev_click_through = canvas_click_through.0;
        canvas.set_ignore_cursor_events(!prev_click_through)?;
        canvas_click_through.0 = !prev_click_through;

        let (menu_item_text, toast_message) = if prev_click_through {
            // If the canvas is toggled to not click-through, try to regain
            // focus to avoid flickering on the first click
            if let Err(e) = canvas.set_focus() {
                eprintln!("Failed to gain focus on canvas: {e}");
            }
            ("Sink", "Canvas floated.")
        } else {
            ("Float", "Canvas sunk.")
        };

        // Update menu item text if it exists
        if let Some(menu_item) = canvas_click_through.1.as_ref() {
            if let Err(e) = menu_item.set_text(menu_item_text) {
                eprintln!("Failed to update menu item for toggling canvas click-through: {e}");
            }
        }

        // Show a toast message on the canvas
        if let Err(e) =
            self.emit_show_toast_to_canvas(ShowToastPayload::Success(toast_message.to_string()))
        {
            eprintln!("Failed to emit show-toast to canvas: {e}");
        }

        Ok(())
    }
}

impl<R: Runtime> StatesExtCanvasClickThrough<R> for App<R> {}
impl<R: Runtime> StatesExtCanvasClickThrough<R> for AppHandle<R> {}
