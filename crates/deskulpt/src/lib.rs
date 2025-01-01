#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use deskulpt_test_states::StatesExt;
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::{generate_handler, tauri_build_context, Builder};

mod commands;
mod setup;

/// Entry point for the Deskulpt application.
///
/// ```ignore
#[doc = include_str!("./main.rs")]
/// ```
pub fn run() {
    Builder::default()
        // Additional application setup
        .setup(|app| {
            app.manage_widget_collection();
            app.manage_widgets_dir();
            app.manage_canvas_click_through();

            setup::init_system_tray(app)?;
            setup::create_canvas(app)?;

            #[cfg(target_os = "macos")]
            // Hide the application from the dock on macOS because hide-from-taskbar is
            // not applicable for macOS; for Windows and Linux we have already hidden
            // the canvas window in `create_canvas`
            app.set_activation_policy(ActivationPolicy::Accessory);

            Ok(())
        })
        .on_window_event(setup::listen_to_windows)
        // Register internal command handlers
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::exit_app,
            commands::init_global_setting,
            commands::open_widget_resource,
            commands::refresh_widget_collection,
            commands::register_toggle_shortcut,
        ])
        // Register plugins
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(deskulpt_test_plugin_fs::init())
        .plugin(deskulpt_test_plugin_sys::init())
        .run(tauri_build_context!())
        .expect("Error running the Deskulpt application");
}
