mod apis;
pub mod utils;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

// The registered apis can be invoked as "plugin:widget_api.fs|<api_name>"
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("widget_api.fs")
        .invoke_handler(tauri::generate_handler![
            apis::read_file,
            apis::write_file,
            apis::append_file,
            apis::remove_file,
        ])
        .build()
}
