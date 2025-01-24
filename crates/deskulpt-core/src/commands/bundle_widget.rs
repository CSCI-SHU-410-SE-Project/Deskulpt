use anyhow::Context;
use tauri::{command, AppHandle, Runtime};

use super::error::{cmderr, CmdResult};
use crate::bundler::WidgetBundlerBuilder;
use crate::config::WidgetConfig;
use crate::path::PathExt;
use crate::states::StatesExtWidgetConfigMap;

/// Bundle a widget.
///
/// ### Errors
///
/// - Widget ID does not exist in the collection.
/// - Widget has a configuration error.
/// - Error bundling the widget.
#[command]
pub async fn bundle_widget<R: Runtime>(
    app_handle: AppHandle<R>,
    id: String,
    base_url: String,
    apis_blob_url: String,
) -> CmdResult<String> {
    let widgets_dir = app_handle.widgets_dir();
    let widget_dir = widgets_dir.join(&id);

    let mut bundler = app_handle.with_widget_config_map(|collection| {
        let config = collection
            .get(&id)
            .ok_or_else(|| cmderr!("Widget (id={}) does not exist in the collection", id))?;

        match config {
            WidgetConfig::Valid { entry, .. } => {
                let builder = WidgetBundlerBuilder::new(
                    widget_dir.to_path_buf(),
                    entry.clone(),
                    base_url,
                    apis_blob_url,
                );
                Ok(builder.build())
            },
            WidgetConfig::Invalid(msg) => Err(cmderr!(msg.clone())),
        }
    })?;

    let code = bundler
        .bundle()
        .await
        .context(format!("Failed to bundle widget (id={})", id))?;
    Ok(code)
}
