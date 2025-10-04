#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg"
)]

use tauri::plugin::TauriPlugin;
use tauri::{generate_handler, Runtime};

mod bundler;
mod commands;
mod config;
pub mod events;
pub mod path;
mod settings;
pub mod states;
pub mod tray;
pub mod window;

deskulpt_common::bindings::configure_bindings_builder!();

/// Initialize the `deskulpt-core` plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    deskulpt_common::init::init_builder!().build()
}

/// Re-exports for JSON schema generation.
pub mod schema {
    pub use crate::settings::Settings;
}
