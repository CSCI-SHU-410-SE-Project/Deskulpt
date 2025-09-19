use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Runtime};

use super::error::CmdResult;
use crate::states::WidgetsStateExt;

#[derive(Debug, Deserialize, specta::Type)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum BundleWidgetsKind {
    All,
    Single(String),
}

#[derive(Debug, Serialize, specta::Type)]
#[serde(tag = "type", content = "content", rename_all = "camelCase")]
pub enum BundleWidgetsResult {
    Ok(String),
    Err(String),
}

impl From<anyhow::Result<String>> for BundleWidgetsResult {
    fn from(res: anyhow::Result<String>) -> Self {
        match res {
            Ok(code) => BundleWidgetsResult::Ok(code),
            Err(e) => BundleWidgetsResult::Err(format!("{e:?}")),
        }
    }
}

/// TODO(Charlie-XIAO)
#[command]
#[specta::specta]
pub async fn bundle_widgets<R: Runtime>(
    app_handle: AppHandle<R>,
    kind: BundleWidgetsKind,
) -> CmdResult<HashMap<String, BundleWidgetsResult>> {
    let results = match kind {
        BundleWidgetsKind::All => app_handle
            .bundle_widgets()
            .await
            .into_iter()
            .map(|(id, res)| (id, res.into()))
            .collect(),
        BundleWidgetsKind::Single(id) => {
            std::iter::once((id.clone(), app_handle.bundle_widget(&id).await.into())).collect()
        },
    };
    Ok(results)
}
