// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;
use tauri::{api, generate_context, generate_handler, Builder};

mod bundler;
mod canvas;
mod commands;
mod config;
mod setup;
mod states;
mod widget_api;

#[cfg(test)]
mod testing;

/// Main entry point of Deskulpt.
fn main() {
    // Get the widget base directory in advance; it seems that `.setup` may not finish
    // before the frontend is loaded, causing errors like accessing unmanaged state
    let context = generate_context!();
    let app_data_dir = api::path::app_data_dir(context.config()).unwrap();
    let widget_base_dir = app_data_dir.join("widgets");
    if !widget_base_dir.exists() {
        create_dir_all(&widget_base_dir).unwrap();
    }

    Builder::default()
        // Additional application setup
        .system_tray(setup::get_system_tray())
        .on_system_tray_event(setup::listen_to_system_tray)
        .on_window_event(setup::listen_to_windows)
        // Initialize state management
        .manage(states::WidgetBaseDirectoryState(widget_base_dir))
        .manage(states::WidgetCollectionState::default())
        // Register internal command handlers
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::open_widget_base,
            commands::refresh_widget_collection,
            canvas::commands::sink_canvas,
            canvas::commands::float_canvas,
            canvas::commands::set_canvas_to_bottom,
            canvas::commands::set_canvas_always_to_bottom,
        ])
        // Register widget API plugins
        .plugin(widget_api::fs::init())
        .run(context)
        .expect("Error running the Deskulpt application");
}
