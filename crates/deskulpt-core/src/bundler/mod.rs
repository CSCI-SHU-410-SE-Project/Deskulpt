//! Bundler for Deskulpt widgets.

mod alias;

use std::path::PathBuf;
use std::sync::Arc;

use alias::AliasPlugin;
use anyhow::{anyhow, Result};
use either::Either;
use rolldown::{
    Bundler, BundlerOptions, BundlerTransformOptions, JsxOptions, OutputFormat, Platform,
    RawMinifyOptions,
};

/// Builder for the Deskulpt widget bundler.
pub struct WidgetBundlerBuilder {
    /// Absolute path to the widget directory.
    root: PathBuf,
    /// Entry file relative to the widget directory.
    entry: String,
}

impl WidgetBundlerBuilder {
    /// Create a new widget bundler builder instance.
    pub fn new(root: PathBuf, entry: String) -> Self {
        Self { root, entry }
    }

    /// Build the Deskulpt widget bundler.
    pub fn build(self) -> WidgetBundler {
        const JSX_RUNTIME_URL: &str = "__DESKULPT_BASE_URL__/gen/jsx-runtime.js";
        const RAW_APIS_URL: &str = "__DESKULPT_BASE_URL__/gen/raw-apis.js";
        const REACT_URL: &str = "__DESKULPT_BASE_URL__/gen/react.js";
        const UI_URL: &str = "__DESKULPT_BASE_URL__/gen/ui.js";
        const APIS_BLOB_URL: &str = "__DESKULPT_APIS_BLOB_URL__";

        let bundler_options = BundlerOptions {
            input: Some(vec![self.entry.into()]),
            cwd: Some(self.root),
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
        let alias_plugin = AliasPlugin(
            [
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
            .into(),
        );

        WidgetBundler {
            bundler: Bundler::with_plugins(bundler_options, vec![Arc::new(alias_plugin)]),
        }
    }
}

/// The Deskulpt widget bundler.
pub struct WidgetBundler {
    bundler: Bundler,
}

impl WidgetBundler {
    /// Bundle the widget.
    pub async fn write(&mut self) -> Result<()> {
        self.bundler.write().await.map_err(|e| {
            anyhow!(e
                .into_vec()
                .iter()
                .map(|diagnostic| diagnostic.to_diagnostic().to_string())
                .collect::<Vec<String>>()
                .join("\n"))
        })?;
        Ok(())
    }
}
