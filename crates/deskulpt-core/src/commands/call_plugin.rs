use std::io::Cursor;

use once_cell::sync::Lazy;
use rmp_serde::Deserializer;
use serde::Deserialize;
use tauri::ipc::{InvokeBody, Request, Response};
use tauri::{command, AppHandle};
use tokio::sync::Mutex;

use super::error::{cmdbail, CmdResult};
use crate::StatesExtWidgetConfigMap;

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
pub async fn call_plugin(app_handle: AppHandle, request: Request<'_>) -> CmdResult<Response> {
    let InvokeBody::Raw(data) = request.body() else {
        cmdbail!("Expected raw data in the request body");
    };

    let mut cursor = Cursor::new(data);
    let mut de = Deserializer::new(&mut cursor);
    let (plugin, command, id): (String, String, String) = Deserialize::deserialize(&mut de)?;

    let pos = cursor.position() as usize;
    let payload = &cursor.get_ref()[pos..];

    let widget_dir_fn = move |id: &str| app_handle.widget_dir(id);
    match plugin.as_str() {
        "fs" => {
            let plugin = FS_PLUGIN.lock().await;
            let result =
                deskulpt_plugin::call_plugin(widget_dir_fn, &*plugin, &command, &id, payload)?;
            Ok(Response::new(result))
        },
        "sys" => {
            let plugin = SYS_PLUGIN.lock().await;
            let result =
                deskulpt_plugin::call_plugin(widget_dir_fn, &*plugin, &command, &id, payload)?;
            Ok(Response::new(result))
        },
        _ => cmdbail!("Unknown plugin: {}", plugin),
    }
}
