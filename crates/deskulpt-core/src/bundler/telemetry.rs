//! Instrumentation plugin for widget bundling.

use std::borrow::Cow;
use std::sync::Arc;

use rolldown::plugin::{
    HookBuildEndArgs, HookBuildStartArgs, HookLoadArgs, HookNoopReturn, HookRenderStartArgs,
    HookResolveIdArgs, HookResolveIdReturn, HookUsage, Plugin, PluginContext,
};
use tracing::{debug, error, info};

/// Logs high-level bundling progress and module resolution details.
#[derive(Debug)]
pub struct WidgetLoggingPlugin {
    widget_id: Arc<str>,
}

impl WidgetLoggingPlugin {
    pub fn new(widget_id: Arc<str>) -> Self {
        Self { widget_id }
    }
}

impl Plugin for WidgetLoggingPlugin {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("deskulpt:widget-telemetry")
    }

    fn register_hook_usage(&self) -> HookUsage {
        HookUsage::BuildStart
            | HookUsage::BuildEnd
            | HookUsage::RenderStart
            | HookUsage::ResolveId
            | HookUsage::Load
    }

    fn build_start(
        &self,
        _ctx: &PluginContext,
        _args: &HookBuildStartArgs<'_>,
    ) -> impl std::future::Future<Output = HookNoopReturn> + Send {
        let widget_id = self.widget_id.clone();
        async move {
            info!(widget_id = %widget_id, "Widget bundle build started");
            Ok(())
        }
    }

    fn render_start(
        &self,
        _ctx: &PluginContext,
        _args: &HookRenderStartArgs<'_>,
    ) -> impl std::future::Future<Output = HookNoopReturn> + Send {
        let widget_id = self.widget_id.clone();
        async move {
            info!(widget_id = %widget_id, "Generating widget bundle output");
            Ok(())
        }
    }

    fn build_end(
        &self,
        _ctx: &PluginContext,
        args: Option<&HookBuildEndArgs>,
    ) -> impl std::future::Future<Output = HookNoopReturn> + Send {
        let widget_id = self.widget_id.clone();
        let errors: Option<Vec<String>> = args.map(|details| {
            details
                .errors
                .iter()
                .map(|diagnostic| diagnostic.to_diagnostic().to_string())
                .collect()
        });
        async move {
            if let Some(errors) = errors {
                for error in errors {
                    error!(widget_id = %widget_id, error = %error, "Widget bundle build error");
                }
            } else {
                info!(widget_id = %widget_id, "Widget bundle build completed");
            }
            Ok(())
        }
    }

    fn resolve_id(
        &self,
        _ctx: &PluginContext,
        args: &HookResolveIdArgs<'_>,
    ) -> impl std::future::Future<Output = HookResolveIdReturn> + Send {
        let widget_id = self.widget_id.clone();
        let specifier = args.specifier.to_string();
        let importer = args
            .importer
            .map(str::to_string)
            .unwrap_or_else(|| "<entry>".to_string());

        async move {
            debug!(
                widget_id = %widget_id,
                importer = %importer,
                specifier = %specifier,
                "Resolving module specifier"
            );
            Ok(None)
        }
    }

    fn load(
        &self,
        _ctx: &PluginContext,
        args: &HookLoadArgs<'_>,
    ) -> impl std::future::Future<Output = rolldown::plugin::HookLoadReturn> + Send {
        let widget_id = self.widget_id.clone();
        let module_id = args.id.to_string();
        async move {
            debug!(widget_id = %widget_id, module = %module_id, "Module loaded");
            Ok(None)
        }
    }
}
