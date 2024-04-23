use tauri::window::Window;

/// ignore cursor so that we click through the window
///
/// Click through means the window will **pass** the cursor events to the window below it
/// If set, we can even click desktop icons behind the window.
pub(crate) fn ignore_cursor(window: Window) {
    window.set_ignore_cursor_events(true).unwrap();
    println!("Ignore cursor events");
}

/// catch cursor so that we can interact with the window
pub(crate) fn catch_cursor(window: Window) {
    window.set_ignore_cursor_events(false).unwrap();
    println!("Catch cursor events");
}
