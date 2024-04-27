mod sys;
pub(crate) mod utils;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// The registered APIs can be invoked as `plugin:widget_api.fs|<api_name>``.
pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("widget_api.sys")
        .invoke_handler(tauri::generate_handler![sys::get_system_info])
        .build()
}
