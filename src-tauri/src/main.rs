// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, fs::read_dir, path::PathBuf, sync::Mutex};

use anyhow::{Context, Error};
use serde::Serialize;
use tauri::{
    command, generate_context, generate_handler, App, AppHandle, Builder, Manager,
};

mod bundler;
mod config;

// --- Utilities -----------------------------------------------------------------------
// All utility functions, structs, types, enums, etc.

type WidgetMapping = HashMap<String, config::WidgetConfig>;

// --- Command payload definitions -----------------------------------------------------
// These should have corresponding frontend declarations. They should only be used as
// the output type of commands or for constructing outputs. They should not be used as
// input types for commands.

#[derive(Serialize)]
struct BundleWidgetCommandPayload {
    success: bool,
    message: String,
}

#[derive(Serialize)]
struct RefreshWidgetCollectionCommandPayload(HashMap<String, config::WidgetConfig>);

// --- States managed by Tauri ---------------------------------------------------------
// https://tauri.app/v1/guides/features/command/#accessing-managed-state

#[derive(Default)]
struct WidgetCollection(Mutex<HashMap<String, config::WidgetConfig>>);

struct WidgetBaseDirectory(PathBuf);

// --- Tauri commands ------------------------------------------------------------------
// https://tauri.app/v1/guides/features/command/

/// Refresh the widget collection state by scanning through the widget base directory.
/// This also wraps the updated widget collection in a command payload and returns it
/// to be used by the frontend.
#[command]
fn refresh_widget_collection(
    app_handle: AppHandle,
) -> RefreshWidgetCollectionCommandPayload {
    let widget_base = &app_handle.state::<WidgetBaseDirectory>().0;

    // Recollect the widget collection by scanning through the base widget directory
    let new_widget_collection: HashMap<String, config::WidgetConfig> =
        match read_dir(widget_base) {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .filter_map(|entry| {
                    let path = entry.path();
                    let widget_config = config::read_widget_config(&path).ok()?;
                    let widget_id = path.file_name()?.to_string_lossy().to_string();
                    Some((widget_id, widget_config))
                })
                .collect(),
            Err(_) => HashMap::new(),
        };

    // Update the widget collection state
    let wigdet_collection = app_handle.state::<WidgetCollection>();
    *wigdet_collection.0.lock().unwrap() = new_widget_collection.clone();
    RefreshWidgetCollectionCommandPayload(new_widget_collection)
}

/// Bundle the widget with the given ID, which should correspond to the maintained
/// widget collection.
#[command]
fn bundle_widget(app_handle: AppHandle, widget_id: String) -> BundleWidgetCommandPayload {
    let widget_collection = &app_handle.state::<WidgetCollection>().0;
    let widget_collection = widget_collection.lock().unwrap();

    // Bundle the widget if it exists in the widget collection
    if let Some(widget_config) = widget_collection.get(&widget_id) {
        let widget_entry =
            &widget_config.directory.join(&widget_config.deskulpt_conf.entry);
        return match bundler::bundle(
            widget_entry,
            widget_config
                .package_json
                .as_ref()
                .map(|package_json| &package_json.dependencies),
        ) {
            bundler::BundlerOutput::Code(code) => {
                BundleWidgetCommandPayload { success: true, message: code }
            },
            bundler::BundlerOutput::Error(error) => {
                BundleWidgetCommandPayload { success: false, message: error }
            },
        };
    }

    BundleWidgetCommandPayload {
        success: false,
        message: format!("Widget (id={}) not found", widget_id),
    }
}

// Application setup for Deskulpt
fn deskulpt_setup(app: &mut App) -> Result<(), Error> {
    app.manage(WidgetBaseDirectory(
        app.path_resolver().app_data_dir().unwrap().join("widgets"),
    ));
    app.manage(WidgetCollection::default());
    Ok(())
}

fn main() {
    Builder::default()
        .setup(|app| {
            deskulpt_setup(app).context("\nFailed to setup Deskulpt")?;
            Ok(())
        })
        .invoke_handler(generate_handler![bundle_widget, refresh_widget_collection])
        .run(generate_context!())
        .expect("FATAL");
}
