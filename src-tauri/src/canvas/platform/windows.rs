use tauri::Window;

#[cfg(target_os = "windows")]
pub(crate) fn set_window_always_on_bottom(window: &Window) {
    use windows::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            SetWindowPos, HWND_BOTTOM, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
        },
    };

    // let hwnd = window.hwnd().unwrap(); // Cast to HWND (specific to Windows platform)
    let hwnd: HWND = HWND(window.hwnd().unwrap().0);
    unsafe {
        SetWindowPos(
            hwnd,
            // this flag set the window to be the bottom-most window
            HWND_BOTTOM,
            0,
            0,
            0,
            0,
            // when calling this function, we want to
            // - not activate the window
            // - not move the window
            // - not resize the window
            SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE,
        )
        .unwrap();
    }
    println!("Window set to always on bottom (Windows)");
}

// #[cfg(target_os = "macos")]
// fn set_window_always_on_bottom(window: &Window) {
//     use cocoa::appkit::{NSWindow, NSWindowOrderingMode};
//     use objc::runtime::YES;

//     let ns_window = window.ns_window().unwrap() as id; // Cast to id (specific to macOS/Cocoa)
//     unsafe {
//         ns_window.setOrdered(NSWindowOrderingMode::NSWindowBelow, None, YES);
//     }
//     println!("Window set to always on bottom (macOS)");
// }

// #[cfg(target_os = "linux")]
// fn set_window_always_on_bottom(window: &Window) {
//     use x11::xlib::{XLowerWindow, Display, Window as XWindow};

//     let x_window = window.xlib_window().unwrap() as XWindow; // Cast to XWindow (specific to X11/Linux)
//     let display = window.xlib_display().unwrap() as *mut Display;
//     unsafe {
//         XLowerWindow(display, x_window);
//     }
//     println!("Window set to always on bottom (Linux)");
// }
