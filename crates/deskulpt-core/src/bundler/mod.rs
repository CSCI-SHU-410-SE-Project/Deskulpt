//! Bundler for Deskulpt widgets.

mod alias;
mod telemetry;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use alias::AliasPlugin;
use anyhow::{anyhow, bail, Result};
use either::Either;
use rolldown::{
    Bundler, BundlerOptions, BundlerTransformOptions, JsxOptions, OutputFormat, Platform,
    RawMinifyOptions,
};
use rolldown_common::Output;
use telemetry::WidgetLoggingPlugin;
use tracing::{debug, error, info, warn};

/// Shared context describing the widget being bundled.
#[derive(Debug, Clone)]
pub struct WidgetBuildContext {
    /// Unique identifier of the widget (directory name).
    pub id: String,
    /// Human-readable widget name.
    pub name: String,
    /// Absolute path to the widget directory.
    pub root: PathBuf,
    /// Entry file relative to the widget root.
    pub entry: String,
    /// Declared dependencies from the widget's package.json.
    pub dependencies: HashMap<String, String>,
}

/// Builder for the Deskulpt widget bundler.
pub struct WidgetBundlerBuilder {
    context: WidgetBuildContext,
}

impl WidgetBundlerBuilder {
    /// Create a new widget bundler builder instance.
    pub fn new(context: WidgetBuildContext) -> Self {
        Self { context }
    }

    /// Build the Deskulpt widget bundler.
    pub fn build(self) -> Result<WidgetBundler> {
        const JSX_RUNTIME_URL: &str = "__DESKULPT_BASE_URL__/gen/jsx-runtime.js";
        const RAW_APIS_URL: &str = "__DESKULPT_BASE_URL__/gen/raw-apis.js";
        const REACT_URL: &str = "__DESKULPT_BASE_URL__/gen/react.js";
        const UI_URL: &str = "__DESKULPT_BASE_URL__/gen/ui.js";
        const APIS_BLOB_URL: &str = "__DESKULPT_APIS_BLOB_URL__";

        let context = self.context;

        info!(
            widget_id = %context.id,
            widget_name = %context.name,
            entry = %context.entry,
            root = %context.root.display(),
            "Initializing widget bundler"
        );

        let bundler_options = BundlerOptions {
            input: Some(vec![context.entry.clone().into()]),
            cwd: Some(context.root.clone()),
            format: Some(OutputFormat::Esm),
            platform: Some(Platform::Browser),
            minify: Some(RawMinifyOptions::Bool(true)),
            // Use automatic runtime for JSX transforms, which will refer to
            // `@deskulpt-test/emotion/jsx-runtime`
            transform: Some(BundlerTransformOptions {
                jsx: Some(Either::Right(JsxOptions {
                    runtime: Some("automatic".to_string()),
                    import_source: Some("@deskulpt-test/emotion".to_string()),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            // Externalize default dependencies available at runtime
            external: Some(
                vec![
                    JSX_RUNTIME_URL.to_string(),
                    RAW_APIS_URL.to_string(),
                    REACT_URL.to_string(),
                    UI_URL.to_string(),
                    APIS_BLOB_URL.to_string(),
                ]
                .into(),
            ),
            ..Default::default()
        };

        // Alias the default dependencies to URLs resolvable at runtime
        let alias_mapping: HashMap<String, String> = [
            (
                "@deskulpt-test/emotion/jsx-runtime".to_string(),
                JSX_RUNTIME_URL.to_string(),
            ),
            (
                "@deskulpt-test/raw-apis".to_string(),
                RAW_APIS_URL.to_string(),
            ),
            ("@deskulpt-test/react".to_string(), REACT_URL.to_string()),
            ("@deskulpt-test/ui".to_string(), UI_URL.to_string()),
            ("@deskulpt-test/apis".to_string(), APIS_BLOB_URL.to_string()),
        ]
        .into();

        let widget_id = Arc::<str>::from(context.id.clone());
        let alias_plugin = AliasPlugin::new(widget_id.clone(), alias_mapping);
        let telemetry_plugin = WidgetLoggingPlugin::new(widget_id.clone());

        let bundler = Bundler::with_plugins(
            bundler_options,
            vec![Arc::new(alias_plugin), Arc::new(telemetry_plugin)],
        )?;

        info!(widget_id = %context.id, "Widget bundler ready");

        Ok(WidgetBundler { bundler, context })
    }
}

/// The Deskulpt widget bundler.
pub struct WidgetBundler {
    bundler: Bundler,
    context: WidgetBuildContext,
}

impl WidgetBundler {
    /// Emit dependency-related logs for the widget.
    pub fn log_dependency_status(&self) {
        let ctx = &self.context;

        info!(
            widget_id = %ctx.id,
            dependency_count = ctx.dependencies.len(),
            "Checking widget dependencies"
        );

        if ctx.dependencies.is_empty() {
            info!(
                widget_id = %ctx.id,
                "No dependencies declared; skipping dependency installation step"
            );
            return;
        }

        for (name, version) in &ctx.dependencies {
            debug!(
                widget_id = %ctx.id,
                dependency = %name,
                version = %version,
                "Widget dependency declared"
            );
        }

        let node_modules = ctx.root.join("node_modules");
        if node_modules.exists() {
            info!(
                widget_id = %ctx.id,
                node_modules = %node_modules.display(),
                "Detected node_modules directory; assuming dependencies are installed"
            );
        } else {
            warn!(
                widget_id = %ctx.id,
                node_modules = %node_modules.display(),
                "node_modules directory missing; install widget dependencies before bundling"
            );
        }

        info!(widget_id = %ctx.id, "Dependency check completed");
    }

    /// Bundle the widget into a single ESM code string.
    pub async fn bundle(&mut self) -> Result<String> {
        let ctx = &self.context;
        let started_at = Instant::now();

        info!(
            widget_id = %ctx.id,
            entry = %ctx.entry,
            "Starting widget bundling"
        );

        let result = match self.bundler.generate().await {
            Ok(result) => result,
            Err(errors) => {
                let messages: Vec<String> = errors
                    .into_vec()
                    .iter()
                    .map(|diagnostic| diagnostic.to_diagnostic().to_string())
                    .collect();

                for message in &messages {
                    error!(
                        widget_id = %ctx.id,
                        error = %message,
                        "Widget bundling failed"
                    );
                }

                return Err(anyhow!(messages.join("\n")));
            },
        };

        if !result.warnings.is_empty() {
            for warning in &result.warnings {
                let diagnostic = warning.to_diagnostic();
                warn!(
                    widget_id = %ctx.id,
                    warning = %diagnostic.to_string(),
                    "Warning emitted during widget bundling"
                );
            }
        }

        for asset in &result.assets {
            debug!(
                widget_id = %ctx.id,
                filename = asset.filename(),
                "Generated bundle asset"
            );
        }

        // We have supplied a single entry file, so we expect a single output
        // bundle; this can be broken if widget code contains e.g. dynamic
        // imports, which we do not allow
        if result.assets.len() != 1 {
            error!(
                widget_id = %ctx.id,
                asset_count = result.assets.len(),
                "Unexpected number of widget bundle assets"
            );
            bail!(
                "Expected 1 bundled output, found {}; ensure that widget code does not contain \
                 e.g. dynamic imports that may result in extra chunks",
                result.assets.len()
            );
        }

        let output = &result.assets[0];
        let code = match output {
            Output::Asset(asset) => asset.source.clone().try_into_string()?,
            Output::Chunk(chunk) => chunk.code.clone(),
        };

        info!(
            widget_id = %ctx.id,
            elapsed_ms = started_at.elapsed().as_millis() as u64,
            code_size = code.len(),
            "Widget bundling completed"
        );

        Ok(code)
    }
}
