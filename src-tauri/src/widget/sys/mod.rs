use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod sysapi;

mod utils;

/// The registered APIs can be invoked as `plugin:widget-sys.fs|<api_name>``.
pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("widget-sys")
        .invoke_handler(generate_handler![sysapi::get_system_info])
        .build()
}
