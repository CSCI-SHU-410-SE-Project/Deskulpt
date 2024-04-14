// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;
use tauri::{
    api, generate_context, utils::config::WindowConfig, AppHandle, Event, Manager,
    Window, WindowBuilder, WindowUrl,
};

// use windows::Win32::{
//     Foundation::HWND,
//     UI::WindowsAndMessaging::{
//         GetWindowLongPtrW, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, HWND_BOTTOM,
//         SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WS_EX_NOACTIVATE,
//     },
// };
// use std::time::{SystemTime, UNIX_EPOCH};

// Greet the user with a message
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod bundler;
mod commands;
mod config;
mod states;
mod widget_api;

#[cfg(test)]
mod testing;

fn create_widget_canvas_window(app_handle: AppHandle) -> String {
    let window_label = "widget-canvas".to_string();
    let window_config: WindowConfig = WindowConfig {
        // url: WindowUrl::App("src/canvas/index.html".into()),
        url: WindowUrl::App("views/canvas.html".into()),

        // url: WindowUrl::App("http://localhost:8080/particles/".into()),
        // url: WindowUrl::App("http://localhost:8080/mytest/".into()),
        // url: WindowUrl::App("http://localhost:8080/mytest/draggable.html".into()),
        label: window_label.clone(),
        // widget window should be transparent, skipTaskbar, and without decorations
        transparent: true,
        skip_taskbar: true,
        resizable: false,
        decorations: false,
        ..Default::default()
    };
    let new_window: Window =
        WindowBuilder::from_config(&app_handle, window_config).build().unwrap();
    println!("Window Canvas window with label \"{}\" created", new_window.label());

    let click_handler = |msg: Event| {
        // let target_window = app_handle.get_window("widget-canvas").unwrap(); // Get the window by label
        // set_window_zindex(target_window.clone()); // Set the window to be below all other windows
        // msg is a Some({ "message": "..." })
        // key by "message"
        println!("click event: {:?}", msg.payload());
    };

    // listen to click event
    new_window.listen("click", click_handler);

    // let window: Window = app_handle.get_window(&window_label).unwrap(); // Get the window by label
    new_window.maximize().unwrap();
    // set_window_zindex(new_window.clone()); // Set the window to be below all other windows
    // make_window_non_activatable(new_window.clone()); // Make the window non-activatable
    // sink_window(new_window.clone()); // Sink the window
    // new_window.set_ignore_cursor_events(true).unwrap();
    window_label
}

// // Common configuration for both main and widget windows
// #[tauri::command]
// fn config_window(window: Window) {
//     make_window_non_activatable(window.clone()); // Make the window non-activatable
//     set_window_zindex(window.clone()); // Set the window to be below all other windows
// }

// When the user clicks the window, it won't become the foreground window
// This should be applied to both main and widget windows, o/w the widget window will still be activatable. This is likely a bug in Tauri or webview2
// #[cfg(windows)]
// fn make_window_non_activatable(window: Window) {
//     match window.hwnd() {
//         Ok(hwnd) => {
//             // Window-specific solution
//             let hwnd = HWND(hwnd.0);
//             unsafe {
//                 let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
//                 SetWindowLongPtrW(
//                     hwnd,
//                     GWL_EXSTYLE,
//                     ex_style | WS_EX_NOACTIVATE.0 as isize, // WS_EX_TOPMOST.0 as isize
//                 );
//             }
//         },
//         Err(err) => {
//             println!("Failed to get window handle: {}", err);
//         },
//     }
// }

// Set window to be below all other windows
// #[tauri::command]
// #[cfg(windows)]
// fn set_window_zindex(window: Window) {
//     // Window-specific solution
//     let hwnd = HWND(window.hwnd().unwrap().0); // Get the HWND of the current window
//                                                // Set window to be always at bottom (behind all other windows but above the desktop) //hwnd: P0, hwndinsertafter: P1, x: i32, y: i32, cx: i32, cy: i32, uflags: SET_WINDOW_POS_FLAGS
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             // progman, // This places your window above the desktop but below other windows
//             HWND_BOTTOM,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE,
//         )
//         .unwrap();
//     };
//     println!("Window with label {} set to be below all other windows", window.label());
// }

#[tauri::command]
fn sink_canvas(app_handle: AppHandle) {
    let window: Window = app_handle.get_window("widget-canvas").unwrap(); // Get the window by label
    sink_window(window.clone()); // Sink the window
                                 // set_window_zindex(window.clone()); // Set the window to be below all other windows
                                 // make_window_non_activatable(window.clone()); // Make the window non-activatable
}

#[tauri::command]
fn float_canvas(app_handle: AppHandle) {
    let window: Window = app_handle.get_window("widget-canvas").unwrap(); // Get the window by label
    float_window(window.clone()); // Sink the window
                                  // set_window_zindex(window.clone()); // Set the window to be below all other windows
                                  // make_window_non_activatable(window.clone()); // Make the window non-activatable
}

// #[cfg(windows)]
fn sink_window(window: Window) {
    window.set_ignore_cursor_events(true).unwrap();
    // let hwnd = HWND(window.hwnd().unwrap().0);
    // unsafe {
    //     let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
    //     SetWindowLongPtrW(
    //         hwnd,
    //         GWL_EXSTYLE,
    //         ex_style |
    //         WS_EX_LAYERED.0 as isize |
    //         WS_EX_TRANSPARENT.0 as isize
    //     );
    // }
}
fn float_window(window: Window) {
    window.set_ignore_cursor_events(false).unwrap();
    // let hwnd = HWND(window.hwnd().unwrap().0);
    // unsafe {
    //     let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
    //     SetWindowLongPtrW(
    //         hwnd,
    //         GWL_EXSTYLE,
    //         ex_style &
    //         !(WS_EX_LAYERED.0 as isize | WS_EX_TRANSPARENT.0 as isize)
    //     );
    // }
}

// Test function
#[tauri::command]
fn test(app_handle: AppHandle) {
    let canvas_window: Window = app_handle.get_window("widget-canvas").unwrap(); // Get the window by label
    canvas_window.maximize().unwrap();
    println!("Test function called");
    // maximize widget manager window
    let window: Window = app_handle.get_window("widget-canvas").unwrap(); // Get the window by label
    window.maximize().unwrap();
}

fn main() {
    // Get the widget base directory in advance; it seems that `.setup` may not finish
    // before the frontend is loaded, causing errors like accessing unmanaged state
    let context = generate_context!();
    let app_data_dir = api::path::app_data_dir(context.config()).unwrap();
    let widget_base_dir = app_data_dir.join("widgets");
    if !widget_base_dir.exists() {
        create_dir_all(&widget_base_dir).unwrap();
    }

    tauri::Builder::default()
        .manage(states::WidgetBaseDirectoryState(widget_base_dir))
        .manage(states::WidgetCollectionState::default())
        .setup(|app| {
            // on click, do something
            let app_handle = app.handle();
            // let main_window: Window = app_handle.get_window("main").unwrap(); // Get the window by label
            // make_window_non_activatable(main_window.clone()); // Make the window non-activatable
            // create widget canvas window to display the widgets
            create_widget_canvas_window(app_handle.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bundle_widget,
            commands::open_widget_base,
            commands::refresh_widget_collection,
            greet,
            sink_canvas,
            float_canvas,
            test,
            // set_window_zindex,
            // create_widget_window,
            // config_window,
            // config_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
