use std::collections::BTreeMap;
use std::fs::read_dir;

use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::path::PathExt;
use crate::states::WidgetsStateExt;
use crate::widgets::Widget;

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
#[specta::specta]
pub async fn rescan_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
) -> CmdResult<BTreeMap<String, Widget>> {
    let widgets_dir = app_handle.widgets_dir()?;
    let mut widgets = BTreeMap::new();

    let entries = read_dir(widgets_dir)?;
    for entry in entries {
        let entry = entry?;

        let path = entry.path();
        if !path.is_dir() {
            continue; // Non-directory entries are not widgets, skip
        }

        if let Some(widget) = Widget::load(&path) {
            let id = widget.id();
            widgets.insert(id, widget);
        }
    }

    let mut old_widgets = app_handle.get_widgets_mut();
    old_widgets.clone_from(&widgets);
    Ok(widgets)
}
