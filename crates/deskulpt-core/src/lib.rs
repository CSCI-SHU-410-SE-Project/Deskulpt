#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

mod bundler;
pub mod commands;
mod config;
pub mod events;
mod path;
mod settings;
mod shortcuts;
mod states;
mod tray;
mod window;

pub use path::PathExt;
pub use settings::Settings;
pub use shortcuts::ShortcutsExt;
pub use states::{StatesExtCanvasImode, StatesExtInitialRender, StatesExtWidgetConfigMap};
pub use tray::TrayExt;
pub use window::{on_window_event, DeskulptWindow, WindowExt};
