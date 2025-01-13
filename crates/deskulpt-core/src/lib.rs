#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

mod bundler;
pub mod commands;
mod config;
mod events;
mod path;
mod settings;
mod states;
mod tray;
mod window;

pub use events::EventsExt;
pub use path::PathExt;
pub use states::{StatesExtCanvasClickThrough, StatesExtWidgetCollection};
pub use tray::TrayExt;
pub use window::{on_window_event, WindowExt};
