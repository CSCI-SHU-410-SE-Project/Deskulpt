use std::path::PathBuf;

use deskulpt_plugin::PluginCommand;

pub struct Exists;

impl PluginCommand for Exists {
    type Input = PathBuf;
    type Output = bool;

    fn name(&self) -> &str {
        "exists"
    }

    fn run(&self, input: Self::Input) -> anyhow::Result<Self::Output> {
        println!("Exists: {:?}", input);
        Ok(input.exists())
    }
}
