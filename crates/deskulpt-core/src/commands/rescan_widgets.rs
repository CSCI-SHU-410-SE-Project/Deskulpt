use deskulpt_common::event::Event;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::config::WidgetCatalog;
use crate::events::UpdateSettingsEvent;
use crate::path::PathExt;
use crate::states::{SettingsStateExt, WidgetCatalogStateExt};

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
pub async fn rescan_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<WidgetCatalog> {
    let widgets_dir = app_handle.widgets_dir()?;
    let new_catalog = WidgetCatalog::load(widgets_dir)?;

    {
        let mut settings = app_handle.get_settings_mut();
        settings
            .widgets
            .retain(|id, _| new_catalog.0.contains_key(id));
        for id in new_catalog.0.keys() {
            settings
                .widgets
                .entry(id.clone())
                .or_insert_with(Default::default);
        }
        UpdateSettingsEvent(settings.clone()).emit(&app_handle)?;
    }

    app_handle.with_widget_catalog_mut(|catalog| {
        *catalog = new_catalog.clone();
    });
    Ok(new_catalog)
}
