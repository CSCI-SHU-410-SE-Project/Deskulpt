use anyhow::Context;
use tauri::{command, AppHandle, Runtime};

use super::error::{cmderr, CmdResult};
use crate::bundler::WidgetBundlerBuilder;
use crate::path::PathExt;
use crate::states::StatesExtWidgetCollection;

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

    let mut bundler = app_handle.with_widget_collection(|collection| {
        collection
            .get(&id)
            .ok_or_else(|| cmderr!("Widget (id={}) does not exist in the collection", id))?
            .as_ref()
            .map(|config| {
                let builder = WidgetBundlerBuilder::new(
                    widget_dir.to_path_buf(),
                    config.entry(),
                    base_url,
                    apis_blob_url,
                );
                builder.build()
            })
            // Propagate the configuration error message
            .map_err(|e| cmderr!(e.to_string()))
    })?;

    let code = bundler
        .bundle()
        .await
        .context(format!("Failed to bundle widget (id={})", id))?;
    Ok(code)
}
