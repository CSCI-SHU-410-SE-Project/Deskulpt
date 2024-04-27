// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler, Builder};

mod bundler;
mod commands;
mod config;
mod setup;
mod states;
mod widget_api;

#[cfg(test)]
mod testing;

/// Main entry point of Deskulpt.
fn main() {
    let context = generate_context!();
    let config = context.config();

    Builder::default()
        // Additional application setup
        .system_tray(setup::get_system_tray())
        .on_system_tray_event(setup::listen_to_system_tray)
        .on_window_event(setup::listen_to_windows)
        // Initialize state management
        .manage(states::WidgetBaseDirectoryState::init(config))
        .manage(states::WidgetConfigCollectionState::default())
        // Register internal command handlers
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::exit_app,
            commands::init_widget_internals,
            commands::open_widget_base,
            commands::refresh_widget_collection,
        ])
        // Register widget API plugins
        .plugin(widget_api::fs::init())
        .run(context)
        .expect("Error running the Deskulpt application");
}
