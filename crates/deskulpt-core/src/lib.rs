#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg"
)]

mod bundler;
pub mod commands;
mod config;
pub mod events;
pub mod path;
mod settings;
pub mod states;
pub mod tray;
pub mod window;

/// Re-exports for JSON schema generation.
pub mod schema {
    pub use crate::settings::Settings;
}
