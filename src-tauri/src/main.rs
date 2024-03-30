// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;
use tauri::{generate_context, generate_handler, Builder, Manager};

mod bundler;
mod commands;
mod config;
mod states;

/// Main entry point of Deskulpt.
fn main() {
    Builder::default()
        .setup(|app| {
            // Create the widget base directory if it does not yet exist
            let widget_base_dir =
                app.path_resolver().app_data_dir().unwrap().join("widgets");
            if !widget_base_dir.exists() {
                create_dir_all(&widget_base_dir).unwrap();
            }

            // Tauri state management
            app.manage(states::WidgetBaseDirectoryState(widget_base_dir));
            app.manage(states::WidgetCollectionState::default());

            Ok(())
        })
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::refresh_widget_collection,
        ])
        .run(generate_context!())
        .expect("FATAL");
}
