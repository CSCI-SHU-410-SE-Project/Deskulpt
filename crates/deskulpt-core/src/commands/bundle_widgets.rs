use std::collections::BTreeMap;

use serde::Deserialize;
use tauri::{command, AppHandle, Runtime};
use tauri_specta::Event;

use super::error::CmdResult;
use crate::events::{RenderWidgetsEvent, UpdateSettingsEvent};
use crate::states::{SettingsStateExt, WidgetsStateExt};
use crate::window::DeskulptWindow;

/// Specifies which widgets to bundle.
#[derive(Debug, Deserialize, specta::Type)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum BundleWidgetsKind {
    /// Bundle all widgets.
    All,
    /// Bundle a single widget by its ID.
    Single(String),
}

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn bundle_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
    kind: BundleWidgetsKind,
) -> CmdResult<()> {
    let widgets = match kind {
        BundleWidgetsKind::All => app_handle
            .bundle_widgets()
            .await
            .into_iter()
            .filter_map(|(id, res)| {
                match res {
                    Ok(code) => Some((id, code)),
                    Err(e) => {
                        // TODO(Charlie-XIAO)
                        eprintln!("Failed to bundle widget {id}: {e:?}");
                        None
                    },
                }
            })
            .collect(),
        BundleWidgetsKind::Single(id) => match app_handle.bundle_widget(&id).await {
            Ok(code) => std::iter::once((id, code)).collect(),
            Err(e) => {
                // TODO(Charlie-XIAO)
                eprintln!("Failed to bundle widget {id}: {e:?}");
                BTreeMap::new()
            },
        },
    };

    {
        let mut settings = app_handle.get_settings_mut();
        settings.widgets.retain(|id, _| widgets.contains_key(id));
        for id in widgets.keys() {
            settings
                .widgets
                .entry(id.clone())
                .or_insert_with(Default::default);
        }
        UpdateSettingsEvent(settings.clone()).emit(&app_handle)?;
    }

    RenderWidgetsEvent(widgets).emit_to(&app_handle, DeskulptWindow::Canvas)?;
    Ok(())
}
