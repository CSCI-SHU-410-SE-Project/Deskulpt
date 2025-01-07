#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

mod command;
mod interface;

use anyhow::{bail, Result};
#[doc(hidden)]
pub use command::BasePluginCommand;
pub use command::PluginCommand;
pub use interface::EngineInterface;
use tauri::AppHandle;

/// The API for a Deskulpt plugin.
pub trait Plugin {
    /// The version of the plugin.
    ///
    /// The default (recommended) implementation uses the version as specified
    /// in `Cargo.toml` for the plugin.
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// The commands provided by the plugin.
    fn commands(&self) -> Vec<Box<dyn BasePluginCommand>>;
}

/// Convenience macro to register commands in a Deskulpt plugin.
///
/// This macro provides an automatic implementation of the [`Plugin::commands`]
/// method. Each registered command must implement the [`PluginCommand`] trait.
///
/// ```no_run
/// # use deskulpt_plugin::{register_commands, Plugin};
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     register_commands![/* List of commands to register */];
/// }
/// ```
#[macro_export]
macro_rules! register_commands {
    ($($command:path),* $(,)?) => {
        fn commands(&self) -> Vec<Box<dyn $crate::BasePluginCommand>> {
            vec![$(Box::new($command),)*]
        }
    };
}

/// Call a Deskulpt plugin (🚧 TODO 🚧).
///
/// ### 🚧 TODO 🚧
///
/// This function should be completed removed and replaced with a `serve_plugin`
/// function that will serve as the entry point of the plugin, running it as a
/// standalone process that can interact with the Deskulpt core through IPC. See
/// [nushell](https://docs.rs/nu-plugin/0.101.0/nu_plugin/fn.serve_plugin.html)
/// for reference.
pub fn call_plugin<P: Plugin>(
    app_handle: AppHandle,
    plugin: P,
    command: &str,
    widget_id: String,
    args: Option<serde_json::Value>,
) -> Result<serde_json::Value> {
    let engine = EngineInterface::new(app_handle);

    for plugin_command in plugin.commands() {
        if plugin_command.name() == command {
            return plugin_command.dispatch(
                widget_id,
                &engine,
                args.unwrap_or(serde_json::Value::Null),
            );
        }
    }
    bail!("Unknown command: {}", command)
}
