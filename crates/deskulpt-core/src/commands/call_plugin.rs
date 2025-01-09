use std::sync::RwLock;

use once_cell::sync::Lazy;
use tauri::{command, AppHandle};

use super::error::{cmdbail, CmdResult};

/// A single file system plugin instance (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// This is a temporary implementation and should be removed in the final
/// implementation. The Deskulpt core will not keep an instance of the plugin,
/// but start the plugin as a separate process and communicate with it
/// throughout its lifecycle. This, same as this temporary implementation, will
/// only instantiate the plugin once and reuse it for all plugin calls.
static FS_PLUGIN: Lazy<RwLock<deskulpt_plugin_fs::FsPlugin>> =
    Lazy::new(|| RwLock::new(deskulpt_plugin_fs::FsPlugin));

/// Call a plugin command (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// The Deskulpt core should keep a state of the registered plugins and call the
/// plugins dynamically. Also, instead of invoking the plugins directly, the
/// Deskulpt core should not depend on any of the plugins and should use IPC to
/// communicate with the plugins.
///
/// Also, in order to simplify the engine API for the plugin (because it is
/// a temporary implementation), `app_handle` is using the default runtime but
/// it should be a generic `R: Runtime` parameter in the final implementation.
#[command]
pub async fn call_plugin(
    app_handle: AppHandle,
    plugin: String,
    command: String,
    widget_id: String,
    payload: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    match plugin.as_str() {
        "fs" => {
            let plugin = FS_PLUGIN.read().unwrap();
            let result = deskulpt_plugin::call_plugin(
                app_handle,
                &*plugin,
                command.as_str(),
                widget_id,
                payload,
            )?;
            Ok(result)
        },
        _ => cmdbail!("Unknown plugin: {}", plugin),
    }
}
