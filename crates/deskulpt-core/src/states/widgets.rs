//! State management for the widgets.

use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use anyhow::{anyhow, Context, Result};
use tauri::{App, AppHandle, Emitter, Manager, Runtime};

use crate::bundler::{WidgetBundler, WidgetBundlerBuilder};
use crate::config::{WidgetConfig, WidgetConfigRegistry};
use crate::path::PathExt;

/// Managed state for the widgets.
#[derive(Default)]
struct WidgetsState {
    configs: RwLock<WidgetConfigRegistry>,
    /// Bundlers for valid widgets.
    ///
    /// To avoid deadlock, always lock `bundlers` before `configs` if both need
    /// to be held. This order is because `configs` are accessed more frequently
    /// than `bundlers` and it needs a smaller critical section.
    bundlers: Mutex<HashMap<String, WidgetBundler>>,
}

/// Extension trait for operations on widgets state.
#[allow(async_fn_in_trait)]
pub trait WidgetsStateExt<R: Runtime>: Manager<R> + Emitter<R> + PathExt<R> + Sized {
    /// Initialize state management for the widgets.
    fn manage_widgets(&self) {
        self.manage(WidgetsState::default());
    }

    fn set_widgets(&self, configs: WidgetConfigRegistry) -> Result<()> {
        let state = self.state::<WidgetsState>();

        let mut bundlers = state.bundlers.lock().unwrap();
        bundlers.clear();
        for (id, widget) in configs.0.iter() {
            if let WidgetConfig::Ok { entry, .. } = widget {
                let root = self.widget_dir(id)?;
                let bundler = WidgetBundlerBuilder::new(root, entry.clone()).build();
                bundlers.insert(id.to_string(), bundler);
            }
        }
        // Hold the bundlers lock till after configs are updated for consistency

        let mut write_guard = state.configs.write().unwrap();
        *write_guard = configs.clone();
        Ok(())
    }

    async fn bundle_widget(&self, id: &str) -> Result<String> {
        let state = self.state::<WidgetsState>();
        let bundler = {
            let guard = state.bundlers.lock().unwrap();
            guard
                .get(id)
                .cloned()
                .ok_or_else(|| anyhow!("No bundler found for widget (id={id})"))?
        };

        bundler
            .bundle()
            .await
            .with_context(|| format!("Failed to bundle widget (id={id})"))
    }

    async fn bundle_widgets(&self) -> Vec<(String, Result<String>)> {
        let state = self.state::<WidgetsState>();
        let bundlers = {
            let guard = state.bundlers.lock().unwrap();
            guard
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<_>>()
        };

        let futs = bundlers.iter().map(|(id, bundler)| async move {
            let code = bundler
                .bundle()
                .await
                .with_context(|| format!("Failed to bundle widget (id={id})"));
            (id.clone(), code)
        });
        futures::future::join_all(futs).await
    }
}

impl<R: Runtime> WidgetsStateExt<R> for App<R> {}
impl<R: Runtime> WidgetsStateExt<R> for AppHandle<R> {}
