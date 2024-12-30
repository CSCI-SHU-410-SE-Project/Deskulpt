//! The widget API plugin for `fs` in `@deskulpt-test/apis`.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{generate_handler, Runtime};

mod apis;
mod utils;

/// Build the `fs` plugin for `@deskulpt-test/apis`.
///
/// The registered APIs can be invoked as `plugin:apis-fs|<api_name>`.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("apis-fs")
        .invoke_handler(generate_handler![
            apis::is_file,
            apis::is_dir,
            apis::exists,
            apis::read_file,
            apis::write_file,
            apis::append_file,
            apis::remove_file,
            apis::create_dir,
            apis::remove_dir,
        ])
        .build()
}
