//! The `sys` plugin for `@deskulpt-test/apis`.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{generate_handler, Runtime};

mod apis;

/// Register the `sys` plugin for `@deskulpt-test/apis`.
///
/// The registered APIs can be invoked as `plugin:apis-sys|<api_name>``.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("apis-sys")
        .invoke_handler(generate_handler![apis::get_system_info])
        .build()
}
