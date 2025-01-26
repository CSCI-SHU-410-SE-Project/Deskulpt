use anyhow::anyhow;
use once_cell::sync::Lazy;
use serde_json::Value as JsonValue;
use tauri::{command, AppHandle};
use tokio::sync::Mutex;

use super::error::{cmdbail, CmdResult};
use crate::{PathExt, StatesExtWidgetConfigMap};

// TODO: Remove this temporary implementation
static FS_PLUGIN: Lazy<Mutex<deskulpt_plugin_fs::FsPlugin>> =
    Lazy::new(|| Mutex::new(deskulpt_plugin_fs::FsPlugin));

// TODO: Remove this temporary implementation
static SYS_PLUGIN: Lazy<Mutex<deskulpt_plugin_sys::SysPlugin>> =
    Lazy::new(|| Mutex::new(Default::default()));

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
    id: String,
    payload: Option<JsonValue>,
) -> CmdResult<JsonValue> {
    match plugin.as_str() {
        "fs" => {
            let plugin = FS_PLUGIN.lock().await;
            let result = deskulpt_plugin::call_plugin(
                move |x: &str| {
                    let widgets_dir = app_handle.widgets_dir();
                    app_handle.with_widget_config_map(|config_map| {
                        config_map
                            .get(x)
                            .ok_or_else(|| anyhow!("WidgetConfig not found"))
                            .map(|config| widgets_dir.join(config.dir()))
                    })
                },
                &*plugin,
                command.as_str(),
                id,
                payload,
            )?;
            Ok(result)
        },
        "sys" => {
            let plugin = SYS_PLUGIN.lock().await;
            let result = deskulpt_plugin::call_plugin(
                move |x: &str| {
                    let widgets_dir = app_handle.widgets_dir();
                    app_handle.with_widget_config_map(|config_map| {
                        config_map
                            .get(x)
                            .ok_or_else(|| anyhow!("WidgetConfig not found"))
                            .map(|config| widgets_dir.join(config.dir()))
                    })
                },
                &*plugin,
                command.as_str(),
                id,
                payload,
            )?;
            Ok(result)
        },
        _ => cmdbail!("Unknown plugin: {}", plugin),
    }
}
