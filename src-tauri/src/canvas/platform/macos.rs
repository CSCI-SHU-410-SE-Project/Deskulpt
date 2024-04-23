// #[cfg(target_os = "macos")]
// fn set_window_always_on_bottom(window: &Window) {
//     use cocoa::appkit::{NSWindow, NSWindowOrderingMode};
//     use objc::runtime::YES;
//     // Cast to id (specific to macOS/Cocoa)
//     let ns_window = window.ns_window().unwrap() as id;
//     unsafe {
//         ns_window.setOrdered(NSWindowOrderingMode::NSWindowBelow, None, YES);
//     }
//     println!("Window set to always on bottom (macOS)");
// }
