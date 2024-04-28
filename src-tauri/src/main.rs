#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_handler, tauri_build_context, Builder, Manager};

mod apis;
mod bundler;
mod commands;
mod config;
mod setup;
mod states;

#[cfg(test)]
mod testing;

/// Main entry point of Deskulpt.
fn main() {
    Builder::default()
        // State management
        .manage(states::WidgetConfigCollectionState::default())
        .manage(states::CanvasClickThroughState::default())
        // Additional application setup
        .setup(|app| {
            app.manage(states::WidgetBaseDirectoryState::init(app));
            setup::init_system_tray(app)?;
            setup::create_canvas(app)?;
            Ok(())
        })
        .on_window_event(setup::listen_to_windows)
        // Register internal command handlers
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::open_widget_base,
            commands::refresh_widget_collection,
        ])
        // Register plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(apis::fs::init())
        .run(tauri_build_context!())
        .expect("Error running the Deskulpt application");
}
