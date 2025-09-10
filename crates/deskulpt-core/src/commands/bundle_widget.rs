use anyhow::Context;
use tauri::{command, AppHandle, Runtime};

use super::error::{cmderr, CmdResult};
use crate::bundler::WidgetBundlerBuilder;
use crate::commands::error::cmdbail;
use crate::path::PathExt;
use crate::states::WidgetsStateExt;
use crate::widgets::Widget;

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
pub async fn bundle_widget<R: Runtime>(
    app_handle: AppHandle<R>,
    id: String,
    base_url: String,
    apis_blob_url: String,
) -> CmdResult<String> {
    let widgets_dir = app_handle.widgets_dir()?;

    let mut bundler = {
        let widgets = app_handle.get_widgets();
        match widgets
            .get(&id)
            .ok_or_else(|| cmderr!("Widget (id={}) does not exist", id))?
        {
            Widget::Valid(valid) => {
                let builder = WidgetBundlerBuilder::new(
                    widgets_dir.join(&valid.dir),
                    valid.deskulpt_conf.entry.clone(),
                    base_url,
                    apis_blob_url,
                );
                builder.build()
            },
            Widget::Invalid(invalid) => cmdbail!(invalid.error.clone()),
        }
    };

    let code = bundler
        .bundle()
        .await
        .context(format!("Failed to bundle widget (id={id})"))?;
    Ok(code)
}
