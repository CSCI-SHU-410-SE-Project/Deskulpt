use std::collections::HashMap;
use std::fs::read_dir;

use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::config::WidgetConfig;
use crate::path::PathExt;
use crate::states::WidgetConfigMapStatesExt;

/// Rescan the widgets directory and update the widget configuration map.
///
/// This will update the widget configuration map state and return the updated
/// configuration map as well.
///
/// ### Errors
///
/// - Failed to access the widgets directory.
/// - Error traversing the widgets directory.
/// - Error inferring widget ID from the directory entry.
#[command]
pub async fn rescan_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
) -> CmdResult<HashMap<String, WidgetConfig>> {
    let widgets_dir = app_handle.widgets_dir()?;
    let mut new_config_map = HashMap::new();

    let entries = read_dir(widgets_dir)?;
    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        if !path.is_dir() {
            continue; // Non-directory entries are not widgets, skip
        }

        if let Some(widget_config) = WidgetConfig::load(&path) {
            let id = widget_config.id();
            new_config_map.insert(id, widget_config);
        }
    }

    app_handle.with_widget_config_map_mut(|config_map| {
        config_map.clone_from(&new_config_map);
    });
    Ok(new_config_map)
}
