use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{dispatch, EngineInterface, PluginCommand};
use serde::Deserialize;

use crate::FsPlugin;

pub struct RemoveFile;

#[derive(Deserialize)]
pub struct RemoveFileInputPayload {
    path: PathBuf,
}

impl PluginCommand for RemoveFile {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "remove_file"
    }

    #[dispatch]
    fn run(
        &self,
        widget_id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: RemoveFileInputPayload,
    ) -> Result<()> {
        let path = engine.widget_dir(widget_id.as_str()).join(input.path);
        std::fs::remove_file(&path)?;
        Ok(())
    }
}
