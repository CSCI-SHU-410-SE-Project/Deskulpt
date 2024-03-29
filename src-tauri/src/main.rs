// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use anyhow::{Context, Error};
use serde::Serialize;
use tauri::{generate_context, generate_handler, App, Builder, Manager};

mod bundler;
mod commands;
mod config;
mod states;

#[derive(Serialize)]
struct RefreshWidgetCollectionCommandPayload(HashMap<String, config::WidgetConfig>);

/// Deskulpt application setup.
///
/// This is used for wrapping the setup phase in an additional layer of error context.
fn deskulpt_setup(app: &mut App) -> Result<(), Error> {
    let widget_base_dir = app.path_resolver().app_data_dir().unwrap().join("widgets");

    app.manage(states::WidgetBaseDirectoryState(widget_base_dir));
    app.manage(states::WidgetCollectionState::default());

    Ok(())
}

/// Main entry point of Deskulpt.
fn main() {
    Builder::default()
        .setup(|app| {
            deskulpt_setup(app).context("\nFailed to setup Deskulpt")?;
            Ok(())
        })
        .invoke_handler(generate_handler![
            commands::bundle_widget,
            commands::refresh_widget_collection,
        ])
        .run(generate_context!())
        .expect("FATAL");
}
