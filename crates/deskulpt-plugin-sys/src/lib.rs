#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{generate_handler, Runtime};

mod apis;

/// Build the `sys` plugin for `@deskulpt-test/apis`.
///
/// The registered APIs can be invoked as `plugin:apis-sys|<api_name>``.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("apis-sys")
        .invoke_handler(generate_handler![apis::get_system_info])
        .build()
}
