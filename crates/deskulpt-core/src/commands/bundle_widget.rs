use anyhow::Context;
use tauri::{command, AppHandle, Runtime};

use super::error::{cmderr, CmdResult};
use crate::bundler::WidgetBundler;
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
    widget_id: String,
    apis_blob_url: String,
) -> CmdResult<String> {
    let widgets_dir = app_handle.widgets_dir();
    let widget_dir = widgets_dir.join(&widget_id);

    let bundler = app_handle.with_widget_collection(|collection| {
        collection
            .get(&widget_id)
            .ok_or_else(|| cmderr!("Widget (id={}) does not exist in the collection", widget_id))?
            .as_ref()
            .map(|config| {
                WidgetBundler::new(
                    widget_dir.to_path_buf(),
                    widget_dir.join(config.entry()),
                    apis_blob_url,
                    config.external_deps(),
                )
            })
            // Propagate the configuration error message
            .map_err(|e| cmderr!(e.to_string()))
    })?;

    let code = bundler
        .bundle()
        .context(format!("Failed to bundle widget (id={})", widget_id))?;
    Ok(code)
}
