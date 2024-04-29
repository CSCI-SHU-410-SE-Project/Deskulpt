//! This module contains utilities that does not fit into any other module.

use crate::states::CanvasClickThroughState;
use anyhow::{bail, Error};
use tauri::{AppHandle, Manager};

/// Toggle the click-through state of the canvas window.
///
/// See [`crate::commands::toggle_click_through`] for documentation.
pub(crate) fn toggle_click_through_state(
    app_handle: &AppHandle,
) -> Result<bool, Error> {
    let canvas = match app_handle.get_webview_window("canvas") {
        Some(canvas) => canvas,
        None => bail!("Canvas window not found"),
    };

    let click_through_state = &app_handle.state::<CanvasClickThroughState>();
    let mut can_click_through = click_through_state.0.lock().unwrap();

    // Try to toggle the click through state
    canvas.set_ignore_cursor_events(!*can_click_through)?;
    *can_click_through = !*can_click_through;

    // If the canvas is now set to not click-through, try to regain focus to avoid
    // flickering on the first click
    if *can_click_through {
        let _ = canvas.set_focus(); // Consume any error because this is not critical
    }
    Ok(*can_click_through)
}
