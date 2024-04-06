//! This file defines a custom Tauri plugin for the widget API.
//!
//! We use a tauri plugin instead of directly registering tauri commands because
//! this allow us to invoke commands with namespace, e.g. invoke('plugin:widget_api|get_dummy_info')

mod dummy;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("widget_api")
        .invoke_handler(tauri::generate_handler![
            dummy::get_dummy_info,
            dummy::shout_text,
        ])
        .build()
}
