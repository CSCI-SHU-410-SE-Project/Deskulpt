use std::collections::HashMap;

use anyhow::Context;
use deskulpt_common::event::Event;
use deskulpt_common::outcome::Outcome;
use deskulpt_common::window::DeskulptWindow;
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::bundler::WidgetBundlerBuilder;
use crate::events::{RenderWidgetsEvent, UpdateSettingsEvent};
use crate::path::PathExt;
use crate::states::{SettingsStateExt, WidgetCatalogStateExt};

/// Bundle widgets.
///
/// TODO(Charlie-XIAO)
///
/// ### Parameters
///
/// - `ids`: If provided, only bundle the widgets with the specified IDs that
///   exist in the widget catalog. If `None`, bundle all widgets in the catalog.
///
/// ### Errors
///
/// - TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn bundle_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
    ids: Option<Vec<String>>,
) -> CmdResult<()> {
    let widgets_dir = app_handle.widgets_dir()?;

    let mut widgets = vec![];
    app_handle.with_widget_catalog(|catalog| match ids {
        Some(ids) => widgets.extend(
            ids.into_iter()
                .filter_map(|id| catalog.0.get(&id).cloned().map(|config| (id, config))),
        ),
        None => widgets.extend(
            catalog
                .0
                .iter()
                .map(|(id, config)| (id.clone(), config.clone())),
        ),
    });

    let futs = widgets.into_iter().map(|(id, config)| async move {
        match config {
            Outcome::Ok(config) => {
                let report =
                    match WidgetBundlerBuilder::new(widgets_dir.join(&id), config.entry.clone())
                        .build()
                        .context("Failed to build widget bundler")
                    {
                        Ok(mut bundler) => bundler
                            .bundle()
                            .await
                            .with_context(|| format!("Failed to bundle widget (id={id})"))
                            .map_or_else(|e| Outcome::Err(format!("{e:?}")), Outcome::Ok),
                        Err(e) => Outcome::Err(format!("{e:?}")),
                    };
                (id, report)
            },
            Outcome::Err(e) => (id, Outcome::Err(e.clone())),
        }
    });

    let reports = futures::future::join_all(futs)
        .await
        .into_iter()
        .collect::<HashMap<_, _>>();

    {
        let mut settings = app_handle.get_settings_mut();
        settings.widgets.retain(|id, _| reports.contains_key(id));
        for id in reports.keys() {
            settings
                .widgets
                .entry(id.clone())
                .or_insert_with(Default::default);
        }
        UpdateSettingsEvent(settings.clone()).emit(&app_handle)?;
    }

    RenderWidgetsEvent(reports).emit_to(&app_handle, DeskulptWindow::Canvas)?;
    Ok(())
}
