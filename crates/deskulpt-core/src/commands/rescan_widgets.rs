use deskulpt_common::event::Event;
use deskulpt_common::window::DeskulptWindow;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::commands::bundle_widgets;
use crate::config::WidgetCatalog;
use crate::events::UpdateWidgetCatalogEvent;
use crate::path::PathExt;
use crate::states::WidgetCatalogStateExt;

/// Rescan the widgets directory to discover widgets.
///
/// This command will update the widget catalog with the newly discovered
/// widgets and emit an event to notify the frontend of the updated catalog. It
/// also implicitly triggers the bundling of all widgets in the updated catalog,
/// see the [`bundle_widgets`] command.
///
/// ### Errors
///
/// - Error accessing the widgets directory.
/// - Error loading the new widget catalog from the widgets directory.
/// - Error emitting the event to notify the frontend of the updated catalog.
/// - Error bundling the widgets.
#[command]
#[specta::specta]
pub async fn rescan_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    let catalog = WidgetCatalog::load(app_handle.widgets_dir()?)?;
    app_handle.with_widget_catalog_mut(|prev| *prev = catalog.clone());
    UpdateWidgetCatalogEvent(catalog).emit_to(&app_handle, DeskulptWindow::Manager)?;

    bundle_widgets(app_handle, None).await?;
    Ok(())
}
