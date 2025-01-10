use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{dispatch, EngineInterface, PluginCommand};
use serde::Deserialize;

use crate::FsPlugin;

pub struct IsFile;

#[derive(Deserialize)]
pub struct IsFileInputPayload {
    path: PathBuf,
}

impl PluginCommand for IsFile {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "is_file"
    }

    #[dispatch]
    fn run(
        &self,
        widget_id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: IsFileInputPayload,
    ) -> Result<bool> {
        let path = engine.widget_dir(widget_id.as_str()).join(input.path);
        Ok(path.is_file())
    }
}
