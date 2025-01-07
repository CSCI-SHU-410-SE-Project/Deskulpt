use tauri::command;

use super::error::{cmdbail, CmdResult};

/// 🚧 TODO 🚧
#[command]
pub async fn call_plugin(
    plugin: String,
    command: String,
    args: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    match plugin.as_str() {
        "fs" => {
            let plugin = deskulpt_plugin_fs::FsPlugin;
            let result = deskulpt_plugin::call_plugin(plugin, command.as_str(), args)?;
            Ok(result)
        },
        _ => cmdbail!("Unknown plugin: {}", plugin),
    }
}
