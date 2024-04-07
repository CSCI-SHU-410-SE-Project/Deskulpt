//! This file defines a dummy custom Tauri plugin for the widget API.
//!
//! We use a tauri plugin instead of directly registering tauri commands because
//! this allow us to invoke commands with namespace, e.g. invoke('plugin:widget_api|get_dummy_info')

mod apis;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

// The registered apis can be invoked as "plugin:widget_api.dummy|<api_name>"
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("widget_api.str")
        .invoke_handler(tauri::generate_handler![apis::get_text,])
        .build()
}
