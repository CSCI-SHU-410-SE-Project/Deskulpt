// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Error};
use tauri::{generate_context, App};

/// Application setup for Deskulpt.
///
/// This function is responsible for setting up the Tauri builder, and the reason for
/// wrapping in a function is to be able to add an error context.
fn deskulpt_setup(_app: &mut App) -> Result<(), Error> {
    // TODO: add setup code here
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            deskulpt_setup(app).context("\nFailed to setup Deskulpt")?;
            Ok(())
        })
        .run(generate_context!())
        .expect("FATAL");
}
