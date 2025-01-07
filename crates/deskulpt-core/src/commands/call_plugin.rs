use tauri::{command, AppHandle};

use super::error::{cmdbail, CmdResult};

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
    args: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    match plugin.as_str() {
        "fs" => {
            let plugin = deskulpt_plugin_fs::FsPlugin;
            let result = deskulpt_plugin::call_plugin(
                app_handle,
                plugin,
                command.as_str(),
                widget_id,
                args,
            )?;
            Ok(result)
        },
        _ => cmdbail!("Unknown plugin: {}", plugin),
    }
}
