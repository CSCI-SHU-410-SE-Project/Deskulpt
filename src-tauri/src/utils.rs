//! This module contains utilities that does not fit into any other module.

use crate::states::CanvasClickThroughState;
use anyhow::{bail, Error};
use tauri::{AppHandle, Manager};

/// Toggle the click-through state of the canvas window.
///
/// This will toggle whether the canvas window ignores cursor events and update the
/// state accordingly. If the canvas is toggled to not click-through, it will try to
/// regain focus automatically. The function will fail if:
///
/// - The canvas window is not found.
/// - Fails to set the canvas to ignore/unignore cursor events.
pub(crate) fn toggle_click_through_state(app_handle: &AppHandle) -> Result<(), Error> {
    let canvas = match app_handle.get_webview_window("canvas") {
        Some(canvas) => canvas,
        None => bail!("Canvas window not found"),
    };

    let click_through_state = &app_handle.state::<CanvasClickThroughState>();
    let mut click_through = click_through_state.0.lock().unwrap();
    let prev_can_click_through = click_through.yes();

    // Try to toggle the click through state
    canvas.set_ignore_cursor_events(!prev_can_click_through)?;
    click_through.toggle();

    // If the canvas is previously click-through, meaning that it is now set to not
    // click-through, try to regain focus to avoid flickering on the first click
    if prev_can_click_through {
        let _ = canvas.set_focus(); // Consume any error because this is not critical
    }
    Ok(())
}
