use tauri::{command, AppHandle, Runtime};
use tauri_specta::Event;

use super::error::CmdResult;
use crate::commands::{bundle_widgets, BundleWidgetsKind};
use crate::config::WidgetConfigRegistry;
use crate::events::UpdateWidgetConfigRegistryEvent;
use crate::path::PathExt;
use crate::states::WidgetsStateExt;
use crate::window::DeskulptWindow;

/// Rescan the widgets directory.
///
/// This command scans the widgets directory for available widgets, loads them,
/// and updates the application's widgets state accordingly. An
/// [`UpdateWidgetsEvent`] is emitted to notify all windows of this update.
///
/// ### Errors
///
/// - Failed to access the widgets directory.
/// - Error traversing the widgets directory.
/// - Error inferring widget ID from the directory entry.
/// - Failed to emit the event.
#[command]
#[specta::specta]
pub async fn load_widgets<R: Runtime>(app_handle: AppHandle<R>) -> CmdResult<()> {
    let registry = WidgetConfigRegistry::load(app_handle.widgets_dir()?)?;
    let event = UpdateWidgetConfigRegistryEvent(registry.clone());
    app_handle.set_widgets(registry)?;
    event.emit_to(&app_handle, DeskulptWindow::Manager)?;

    bundle_widgets(app_handle, BundleWidgetsKind::All).await?;
    Ok(())
}
