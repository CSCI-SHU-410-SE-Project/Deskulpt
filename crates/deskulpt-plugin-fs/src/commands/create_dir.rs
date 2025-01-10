use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{dispatch, EngineInterface, PluginCommand};
use serde::Deserialize;

use crate::FsPlugin;

pub struct CreateDir;

#[derive(Deserialize)]
pub struct CreateDirInputPayload {
    path: PathBuf,
}

impl PluginCommand for CreateDir {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "create_dir"
    }

    #[dispatch]
    fn run(
        &self,
        widget_id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: CreateDirInputPayload,
    ) -> Result<()> {
        let path = engine.widget_dir(widget_id.as_str()).join(input.path);
        std::fs::create_dir_all(&path)?;
        Ok(())
    }
}
