#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

mod bundler;
pub mod commands;
mod config;
pub mod events;
pub mod path;
pub mod settings;
pub mod shortcuts;
pub mod states;
pub mod tray;
pub mod window;
