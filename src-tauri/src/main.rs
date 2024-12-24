#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! [![](https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/public/deskulpt-full.png)](https://csci-shu-410-se-project.github.io/Deskulpt/)
//!
//! The backend documentation for developers of Deskulpt.

#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src-tauri/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src-tauri/icons/icon.png"
)]

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;
use tauri::{generate_handler, tauri_build_context, Builder, Manager};

mod apis;
mod bundler;
mod commands;
mod config;
mod settings;
mod setup;
mod states;
mod utils;

#[cfg(test)]
mod testing;

/// Main entry point of Deskulpt.
fn main() {
    Builder::default()
        // Additional application setup
        .setup(|app| {
            app.manage(states::WidgetBaseDirectoryState::init(
                app.path().app_data_dir().unwrap(),
            ));
            setup::init_system_tray(app)?;
            setup::create_canvas(app)?;

            #[cfg(target_os = "macos")]
            // Hide the application from the dock on macOS because hide-from-taskbar is
            // not applicable for macOS; for Windows and Linux we have already hidden
            // the canvas window in `create_canvas`
            app.set_activation_policy(ActivationPolicy::Accessory);

            Ok(())
        })
        .manage(states::WidgetConfigCollectionState::default())
        .on_window_event(setup::listen_to_windows)
        // Register internal command handlers
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::exit_app,
            commands::init_settings,
            commands::open_widget_resource,
            commands::refresh_widget_collection,
            commands::register_toggle_shortcut,
        ])
        // Register plugins
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(apis::fs::init())
        .plugin(apis::sys::init())
        .run(tauri_build_context!())
        .expect("Error running the Deskulpt application");
}
