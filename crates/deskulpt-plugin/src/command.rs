//! Plugin command APIs.

use std::fmt::Debug;

use anyhow::{Context, Result};

use crate::interface::EngineInterface;

/// The wrapper trait for [`PluginCommand`].
///
/// This trait is used to provide type-erased plugin command execution. This
/// allows plugin commands with different input and output types to be stored
/// in the same collection.
///
/// ⚠️ This trait is not part of the public API and should not be implemented
/// directly. Implement [`PluginCommand`] instead.
pub trait BasePluginCommand {
    /// Same as [`PluginCommand::name`].
    fn name(&self) -> &str;

    /// Dispatch the command.
    ///
    /// This method handles the serialization boundary between the plugin system
    /// an [`PluginCommand::run`], converting JSON values to and from the
    /// concrete types specified by the command implementation.
    fn dispatch(
        &self,
        widget_id: String,
        api: &EngineInterface,
        input: serde_json::Value,
    ) -> Result<serde_json::Value>;
}

/// The API for a Deskulpt plugin command.
pub trait PluginCommand: BasePluginCommand {
    /// The input type of the command.
    type Input: serde::de::DeserializeOwned;

    /// The output type of the command.
    type Output: serde::Serialize + Debug;

    /// The name of the command.
    fn name(&self) -> &str;

    /// The implementation of the command.
    fn run(
        &self,
        widget_id: String,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output>;
}

impl<T: PluginCommand> BasePluginCommand for T {
    fn name(&self) -> &str {
        PluginCommand::name(self)
    }

    fn dispatch(
        &self,
        widget_id: String,
        engine: &EngineInterface,
        input: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let context = format!(
            "Failed to deserialize input to plugin command: {}\nInput: {:?}",
            PluginCommand::name(self),
            input
        );
        let input: T::Input = serde_json::from_value(input).context(context)?;

        let output = self.run(widget_id, engine, input).context(format!(
            "Failed to run plugin command: {}",
            PluginCommand::name(self)
        ))?;

        let context = format!(
            "Failed to serialize output from plugin command: {}\nOutput: {:?}",
            PluginCommand::name(self),
            output
        );
        let output = serde_json::to_value(output).context(context)?;

        Ok(output)
    }
}
