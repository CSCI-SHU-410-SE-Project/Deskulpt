use anyhow::Context;
use tauri::{command, AppHandle, Runtime};
use tracing::{info, warn};

use super::error::{cmderr, CmdResult};
use crate::bundler::{WidgetBuildContext, WidgetBundlerBuilder};
use crate::config::WidgetConfig;
use crate::path::PathExt;
use crate::states::WidgetCatalogStateExt;

/// Bundle a widget.
///
/// ### Errors
///
/// - Failed to access the widgets directory.
/// - Widget ID does not exist in the configuration map.
/// - Widget has a configuration error.
/// - Error bundling the widget.
#[command]
#[specta::specta]
pub async fn bundle_widget<R: Runtime>(app_handle: AppHandle<R>, id: String) -> CmdResult<String> {
    info!(widget_id = %id, "Received widget bundling request");
    let widgets_dir = app_handle.widgets_dir()?;
    let widget_dir = widgets_dir.join(&id);

    let widget_config = app_handle.with_widget_catalog(|catalog| catalog.0.get(&id).cloned());
    let widget_config =
        widget_config.ok_or_else(|| cmderr!(format!("Widget (id={id}) does not exist")))?;

    let context = match widget_config {
        WidgetConfig::Ok {
            name,
            entry,
            dependencies,
        } => {
            info!(
                widget_id = %id,
                widget_name = %name,
                entry = %entry,
                "Preparing widget bundler context"
            );
            WidgetBuildContext {
                id: id.clone(),
                name,
                root: widget_dir,
                entry,
                dependencies,
            }
        },
        WidgetConfig::Err { error } => {
            warn!(widget_id = %id, error = %error, "Widget configuration error encountered");
            return Err(cmderr!(error));
        },
    };

    let mut bundler = WidgetBundlerBuilder::new(context)
        .build()
        .context("Failed to build widget bundler")?;

    bundler.log_dependency_status();

    let code = bundler
        .bundle()
        .await
        .context(format!("Failed to bundle widget (id={id})"))?;

    info!(widget_id = %id, "Widget bundled successfully");
    Ok(code)
}
