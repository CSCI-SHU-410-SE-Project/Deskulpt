use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{EngineInterface, PluginCommand};

pub struct Exists;

impl PluginCommand for Exists {
    type Input = PathBuf;
    type Output = bool;

    fn name(&self) -> &str {
        "exists"
    }

    fn run(
        &self,
        widget_id: String,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let path = engine.widget_dir(widget_id.as_str()).join(input);
        Ok(path.exists())
    }
}

pub struct IsFile;

impl PluginCommand for IsFile {
    type Input = PathBuf;
    type Output = bool;

    fn name(&self) -> &str {
        "is_file"
    }

    fn run(
        &self,
        widget_id: String,
        engine: &EngineInterface,
        input: Self::Input,
    ) -> Result<Self::Output> {
        let path = engine.widget_dir(widget_id.as_str()).join(input);
        Ok(path.is_file())
    }
}
