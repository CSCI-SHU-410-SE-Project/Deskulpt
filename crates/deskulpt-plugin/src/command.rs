//! Plugin command APIs.

use anyhow::{Context, Result};

/// The wrapper trait for [`PluginCommand`].
///
/// This trait is used to provide type-erased plugin command execution. This
/// allows plugin commands with different input and output types to be stored
/// in the same collection.
pub trait BasePluginCommand {
    /// Same as [`PluginCommand::name`].
    fn name(&self) -> &str;

    /// Dispatch the command.
    ///
    /// This method handles the serialization boundary between the plugin system
    /// and individual commands, converting JSON values to and from the concrete
    /// types specified by the command implementation.
    fn dispatch(&self, input: serde_json::Value) -> Result<serde_json::Value>;
}

/// The API for a Deskulpt plugin command.
pub trait PluginCommand: BasePluginCommand {
    /// The input type of the command.
    type Input: serde::de::DeserializeOwned;

    /// The output type of the command.
    type Output: serde::Serialize;

    /// The name of the command.
    fn name(&self) -> &str;

    /// The implementation of the command.
    fn run(&self, input: Self::Input) -> Result<Self::Output>;
}

impl<T: PluginCommand> BasePluginCommand for T {
    fn name(&self) -> &str {
        PluginCommand::name(self)
    }

    fn dispatch(&self, input: serde_json::Value) -> Result<serde_json::Value> {
        let input: T::Input = serde_json::from_value(input).context(format!(
            "Failed to deserialize input to plugin command: {}",
            PluginCommand::name(self)
        ))?;
        let output = self.run(input).context(format!(
            "Failed to run plugin command: {}",
            PluginCommand::name(self)
        ))?;
        let output = serde_json::to_value(output).context(format!(
            "Failed to serialize output from plugin command: {}",
            PluginCommand::name(self)
        ))?;
        Ok(output)
    }
}
