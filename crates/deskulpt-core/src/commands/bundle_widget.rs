use anyhow::Context;
use tauri::{command, AppHandle, Runtime};

use super::error::{cmderr, CmdResult};
use crate::bundler::WidgetBundlerBuilder;
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
    let widgets_dir = app_handle.widgets_dir()?;

    let mut bundler = app_handle.with_widget_catalog(|catalog| {
        match catalog
            .0
            .get(&id)
            .ok_or_else(|| cmderr!("Widget (id={}) does not exist", id))?
        {
            WidgetConfig::Ok { entry, .. } => {
                let builder = WidgetBundlerBuilder::new(widgets_dir.join(&id), entry.clone());
                Ok(builder.build().context("Failed to build widget bundler")?)
            },
            WidgetConfig::Err { error } => Err(cmderr!(error.clone())),
        }
    })?;

    let code = bundler
        .bundle()
        .await
        .context(format!("Failed to bundle widget (id={id})"))?;
    Ok(code)
}
