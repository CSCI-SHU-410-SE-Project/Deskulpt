use std::collections::HashMap;
use std::fs::read_dir;

use serde::Serialize;
use tauri::{command, AppHandle, Runtime};
use uuid::Uuid;

use super::error::CmdResult;
use crate::config::WidgetConfig;
use crate::path::PathExt;
use crate::states::StatesExtWidgetConfigMap;

/// Output type of the [`rescan_widgets`] command.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RescanWidgetsOutput {
    /// All widgets discovered in the widgets directory.
    config_map: HashMap<String, WidgetConfig>,
    /// IDs of widgets that were newly discovered.
    added_ids: Vec<String>,
    /// IDs of widgets that were removed.
    removed_ids: Vec<String>,
}

/// Rescan the widgets directory and update the widget collection.
///
/// ### Errors
///
/// - Error traversing the widgets directory.
/// - Error inferring widget ID from the directory entry.
#[command]
pub async fn rescan_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
) -> CmdResult<RescanWidgetsOutput> {
    let widgets_dir = app_handle.widgets_dir();
    let mut new_config_map = HashMap::new();

    let entries = read_dir(widgets_dir)?;
    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        if !path.is_dir() {
            continue; // Non-directory entries are not widgets, skip
        }

        if let Some(widget_config) = WidgetConfig::load(&path) {
            // Generate a unique ID for the widget based on its directory; we
            // use UUID v5 because it is deterministic
            let dir_encoded = widget_config.dir().as_os_str().as_encoded_bytes();
            let id = Uuid::new_v5(&Uuid::NAMESPACE_URL, dir_encoded).to_string();
            new_config_map.insert(id, widget_config);
        }
    }

    let (added_ids, removed_ids) = app_handle.with_widget_config_map_mut(|config_map| {
        let added_ids = new_config_map
            .keys()
            .filter(|&id| !config_map.contains_key(id))
            .cloned()
            .collect::<Vec<_>>();
        let removed_ids = config_map
            .keys()
            .filter(|&id| !new_config_map.contains_key(id))
            .cloned()
            .collect::<Vec<_>>();

        // Update the widget configuration map state
        config_map.clone_from(&new_config_map);
        (added_ids, removed_ids)
    });

    Ok(RescanWidgetsOutput {
        config_map: new_config_map,
        added_ids,
        removed_ids,
    })
}
