//! This module contains utilities that does not fit into any other module.

use crate::states::CanvasClickThroughState;
use anyhow::{bail, Error, Ok};
use serde::Serialize;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, Manager, Runtime};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    CombineRgn, CreateRectRgn, DeleteObject, SetWindowRgn, HRGN, RGN_OR,
};
// use windows::Win32::UI::WindowsAndMessaging::
// {RedrawWindow, RDW_INVALIDATE, RDW_ERASE};
use windows::Win32::Graphics::Gdi::{RedrawWindow, RDW_ERASE, RDW_INVALIDATE};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, SetWindowLongPtrW, GWL_STYLE, WS_POPUP, WS_VISIBLE,
};

/// Mapping from widget IDs to corresponding data.
pub(crate) type IdMap<T> = HashMap<String, T>;

/// Toast kind of the "show-toast" event.
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum ToastKind {
    Success,
}

/// Payload of the "show-toast" event.
#[derive(Serialize, Clone)]
struct ShowToastPayload {
    kind: ToastKind,
    message: String,
}

/// Toggle the click-through state of the canvas window.
///
/// This will toggle whether the canvas window ignores cursor events and update the
/// state accordingly. If the canvas is toggled to not click-through, it will try to
/// regain focus automatically. The function will fail if:
///
/// - The canvas window is not found.
/// - Fails to set the canvas to ignore/unignore cursor events.
pub(crate) fn toggle_click_through_state<R: Runtime>(
    app_handle: &AppHandle<R>,
) -> Result<(), Error> {
    let canvas = match app_handle.get_webview_window("canvas") {
        Some(canvas) => canvas,
        None => bail!("Canvas window not found"),
    };

    #[cfg(target_os = "windows")]
    {
        let hwnd = canvas.hwnd().unwrap();
        print!("Got hwnd for Windows OS: {:?}\n", hwnd);
        let rects = vec![
            (50, 50, 2500, 1000),
            // (200, 200, 300, 300)
        ];
        set_clickable_regions(hwnd, &rects).unwrap();
    }

    let click_through_state = &app_handle.state::<CanvasClickThroughState<R>>();
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

    // Try to let canvas show the toast message
    let _ = app_handle.emit_to(
        "canvas",
        "show-toast",
        ShowToastPayload {
            kind: ToastKind::Success,
            message: format!(
                "Canvas {}.",
                if prev_can_click_through { "floated" } else { "sunk" }
            ),
        },
    );
    Ok(())
}

pub fn set_clickable_regions(
    hwnd: HWND,
    rects: &[(i32, i32, i32, i32)],
) -> Result<(), Error> {
    println!("Set the following region to be clickable, and their outside to be clickthrough:");
    for rect in rects {
        print!("Rect: {}, {}, {}, {}\n", rect.0, rect.1, rect.2, rect.3);
    }
    unsafe {
        // Start with an empty region
        let mut combined_region: HRGN = CreateRectRgn(0, 0, 0, 0);
        if combined_region.0 == std::ptr::null_mut() {
            return Err(Error::msg("Failed to create initial empty region."));
        }

        // Add each rectangle to the combined region
        for &(left, top, right, bottom) in rects {
            let rect_region = CreateRectRgn(left, top, right, bottom);
            if rect_region.0 == std::ptr::null_mut() {
                let _ = DeleteObject(combined_region);
                return Err(Error::msg("Failed to create rectangle region."));
            }

            let result =
                CombineRgn(combined_region, combined_region, rect_region, RGN_OR);
            let _ = DeleteObject(rect_region); // Clean up the temporary region
            if result == windows::Win32::Graphics::Gdi::GDI_REGION_TYPE(0) {
                let _ = DeleteObject(combined_region);
                return Err(Error::msg("Failed to combine rectangle region."));
            }
        }

        // Apply the combined region to the window
        let result = SetWindowRgn(hwnd, combined_region, true);
        if result == 0 {
            let _ = DeleteObject(combined_region);
            return Err(Error::msg("Failed to set the window region."));
        }

        let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as isize; // Cast to isize for operations
        let new_style = (style & !(WS_VISIBLE.0 as isize)) | (WS_POPUP.0 as isize); // Perform bitwise operations
        SetWindowLongPtrW(hwnd, GWL_STYLE, new_style);

        // Trigger a redraw of the window to apply the changes visually
        let _ = RedrawWindow(hwnd, None, None, RDW_INVALIDATE | RDW_ERASE);

        // The system takes ownership of the region after SetWindowRgn, so we don't delete it
        Ok(())
    }
}
