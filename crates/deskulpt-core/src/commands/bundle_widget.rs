use anyhow::Context;
use tauri::{command, AppHandle, Runtime};

use super::error::{cmderr, CmdResult};
use crate::bundler::WidgetBundlerBuilder;
use crate::config::WidgetConfig;
use crate::path::PathExt;
use crate::states::WidgetConfigMapStateExt;

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
pub async fn bundle_widget<R: Runtime>(app_handle: AppHandle<R>, id: String) -> CmdResult<()> {
    let widgets_dir = app_handle.widgets_dir()?;

    let mut bundler = app_handle.with_widget_config_map(|config_map| {
        match config_map
            .get(&id)
            .ok_or_else(|| cmderr!("Widget (id={}) does not exist", id))?
        {
            WidgetConfig::Valid {
                dir, deskulpt_conf, ..
            } => {
                let builder =
                    WidgetBundlerBuilder::new(widgets_dir.join(dir), deskulpt_conf.entry.clone());
                Ok(builder.build())
            },
            WidgetConfig::Invalid { error, .. } => Err(cmderr!(error.clone())),
        }
    })?;

    bundler
        .write()
        .await
        .context(format!("Failed to bundle widget (id={id})"))?;
    Ok(())
}
