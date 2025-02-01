use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{dispatch, EngineInterface, PluginCommand};
use serde::Deserialize;

use crate::FsPlugin;

pub struct Exists;

#[derive(Deserialize)]
pub struct ExistsInputPayload {
    path: PathBuf,
}

impl PluginCommand for Exists {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "exists"
    }

    #[dispatch]
    fn run(
        &self,
        id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: ExistsInputPayload,
    ) -> Result<bool> {
        let path = engine.widget_dir(id)?.join(input.path);
        Ok(path.exists())
    }
}
