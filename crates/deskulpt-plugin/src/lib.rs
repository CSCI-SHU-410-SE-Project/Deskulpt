#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

mod command;

use anyhow::{bail, Result};
#[doc(hidden)]
pub use command::BasePluginCommand;
pub use command::PluginCommand;

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

/// TODO: To be removed.
#[doc(hidden)]
pub fn call_plugin<P: Plugin>(
    plugin: P,
    command: &str,
    args: Option<serde_json::Value>,
) -> Result<serde_json::Value> {
    for plugin_command in plugin.commands() {
        if plugin_command.name() == command {
            return plugin_command.dispatch(args.unwrap_or(serde_json::Value::Null));
        }
    }
    bail!("Unknown command: {}", command)
}
