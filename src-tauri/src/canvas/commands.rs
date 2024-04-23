//! This module provides a cross-platform interface for adjusting the
//! position of the canvas (sink, float)

use crate::canvas::platform;
use tauri::{command, window::Window, AppHandle, Manager};

fn get_canvas(app_handle: &AppHandle) -> Window {
    let canvas_label = "canvas";
    app_handle.get_window(canvas_label).unwrap()
}

/// Sink the canvas
///
/// To sink a canvas is to make a window
/// - ignore cursor events, so that we can click through it
/// - set z-index so that the window is at the bottom of all windows
#[command]
pub(crate) fn sink_canvas(app_handle: AppHandle) {
    println!("Sink the canvas");
    let window: Window = get_canvas(&app_handle);
    platform::common::ignore_cursor(&window);
    platform::common::set_window_to_bottom(&window);
}

/// Float the canvas
///
/// To float a canvas is to make a window
/// - catch cursor events, so that we can interact with it
/// - set z-index to be at the bottom of all windows
///    Note that z-index should always be set to the bottom of all windows because
///    the window is a canvas of desktop widgets, artistically like a desktop wallpaper
#[command]
pub(crate) fn float_canvas(app_handle: AppHandle) {
    println!("Float the canvas");
    let window: Window = get_canvas(&app_handle);
    platform::common::catch_cursor(&window);
    // platform::common::set_window_to_bottom(&window);
}

/// Set the canvas to the bottom of all windows
#[command]
pub(crate) fn set_canvas_to_bottom(app_handle: AppHandle) {
    println!("Set the canvas to the bottom");
    let window: Window = get_canvas(&app_handle);
    platform::common::set_window_to_bottom(&window);
}

/// Set the canvas to always be at the bottom of all windows
#[command]
pub(crate) fn set_canvas_always_to_bottom(app_handle: AppHandle) {
    println!("Set the canvas to always be at the bottom");
    let window: Window = get_canvas(&app_handle);
    platform::common::set_window_always_to_bottom(&window);
}
