//! The widget API plugin for `sys` in `@deskulpt-test/apis`.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{generate_handler, Runtime};

mod apis;

/// Build the `sys` plugin for `@deskulpt-test/apis`.
///
/// The registered APIs can be invoked as `plugin:apis-sys|<api_name>``.
pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("apis-sys")
        .invoke_handler(generate_handler![apis::get_system_info])
        .build()
}
