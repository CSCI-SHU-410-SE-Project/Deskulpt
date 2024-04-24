use tauri::window::Window;

/// ignore cursor so that we click through the window
///
/// Click through means the window will pass the cursor events to the window below it
/// If set, we can even click desktop icons behind the window.
pub(crate) fn ignore_cursor(window: &Window) {
    window.set_ignore_cursor_events(true).unwrap();
    println!("Ignore cursor events");
}

/// catch cursor so that we can interact with the window
pub(crate) fn catch_cursor(window: &Window) {
    window.set_ignore_cursor_events(false).unwrap();
    println!("Catch cursor events");
}

/// Set the window to always be on the bottom of all windows
pub(crate) fn set_window_to_bottom(window: &Window) {
    #[cfg(target_os = "windows")]
    crate::canvas::platform::windows::platform_set_window_to_bottom(window);
    #[cfg(target_os = "linux")]
    crate::canvas::platform::linux::platform_set_window_to_bottom(window);
}

/// Set the window to be always at the bottom of all windows
pub(crate) fn set_window_always_to_bottom(window: &Window) {
    #[cfg(target_os = "windows")]
    crate::canvas::platform::windows::platform_set_window_always_to_bottom(window);
}
