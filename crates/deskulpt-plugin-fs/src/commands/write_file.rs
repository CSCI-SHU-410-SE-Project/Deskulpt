use std::path::PathBuf;

use anyhow::Result;
use deskulpt_plugin::{dispatch, EngineInterface, PluginCommand};
use serde::Deserialize;

use crate::FsPlugin;

pub struct WriteFile;

#[derive(Deserialize)]
pub struct WriteFileInputPayload {
    path: PathBuf,
    content: String,
}

impl PluginCommand for WriteFile {
    type Plugin = FsPlugin;

    fn name(&self) -> &str {
        "write_file"
    }

    #[dispatch]
    fn run(
        &self,
        widget_id: String,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        input: WriteFileInputPayload,
    ) -> Result<()> {
        let path = engine.widget_dir(widget_id.as_str()).join(input.path);
        std::fs::write(&path, input.content)?;
        Ok(())
    }
}
