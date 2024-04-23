// #[cfg(target_os = "linux")]
// fn set_window_always_on_bottom(window: &Window) {
//     use x11::xlib::{XLowerWindow, Display, Window as XWindow};
// 	   // Cast to XWindow (specific to X11/Linux)
//     let x_window = window.xlib_window().unwrap() as XWindow;
//     let display = window.xlib_display().unwrap() as *mut Display;
//     unsafe {
//         XLowerWindow(display, x_window);
//     }
//     println!("Window set to always on bottom (Linux)");
// }
