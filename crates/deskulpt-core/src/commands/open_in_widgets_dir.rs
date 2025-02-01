use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::path::PathExt;

/// Open a path in the widgets directory.
///
/// The components are joined to form a path relative to the widgets directory.
/// If the components are empty, this will open the widgets directory itself.
/// Note that this command trusts the caller to provide a valid path and does
/// not perform any validation.
///
/// ### Errors
///
/// - Error opening the specified path.
#[command]
pub async fn open_in_widgets_dir<R: Runtime>(
    app_handle: AppHandle<R>,
    components: Vec<String>,
) -> CmdResult<()> {
    let mut open_path = app_handle.widgets_dir().to_path_buf();
    open_path.extend(components);
    open::that_detached(open_path)?;

    Ok(())
}
