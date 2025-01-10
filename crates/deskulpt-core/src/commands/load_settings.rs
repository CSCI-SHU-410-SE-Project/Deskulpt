use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::path::PathExt;
use crate::settings::Settings;

/// Load the persisted settings.
///
/// This command never returns an error despite the return type. On any error it
/// will return the default settings instead.
#[command]
pub async fn load_settings<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<Settings> {
    let persist_dir = app_handle.persist_dir();

    let settings = match Settings::load(persist_dir) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Failed to load persisted settings: {:?}", e);
            Default::default()
        },
    };
    Ok(settings)
}
