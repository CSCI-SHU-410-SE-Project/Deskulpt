use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::path::PathExt;
use crate::settings::Settings;

/// Exit the application with cleanup.
///
/// This command never returns an error; in other words it will always exit the
/// application in the end. Prior to exiting, it will try to dump the settings
/// for persistence, but failure to do so will not prevent exiting.
#[command]
pub async fn exit_app<R: Runtime>(app_handle: AppHandle<R>, settings: Settings) -> CmdResult<()> {
    let persist_dir = app_handle.persist_dir();
    if let Err(e) = settings.dump(persist_dir) {
        eprintln!("Failed to dump settings: {e}");
    }

    app_handle.exit(0);
    Ok(())
}
