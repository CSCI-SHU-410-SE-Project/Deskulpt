//! Bundler for Deskulpt widgets.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use oxc::transformer::{JsxOptions, JsxRuntime};
use rolldown::{Bundler, BundlerOptions, Jsx, OutputFormat, Platform};
use rolldown_common::Output;
use rolldown_plugin_alias::{Alias, AliasPlugin};
use rolldown_utils::pattern_filter::StringOrRegex;

/// Builder for the Deskulpt widget bundler.
pub struct WidgetBundlerBuilder {
    /// Absolute path to the widget directory.
    root: PathBuf,
    /// Entry file relative to the widget directory.
    entry: String,
    /// The base URL to resolve local path imports.
    base_url: String,
    /// URL to the widget APIs blob.
    apis_blob_url: String,
}

impl WidgetBundlerBuilder {
    /// Create a new widget bundler builder instance.
    pub fn new(root: PathBuf, entry: String, base_url: String, apis_blob_url: String) -> Self {
        Self {
            root,
            entry,
            base_url,
            apis_blob_url,
        }
    }

    /// Build a Deskulpt widget bundler.
    pub fn build(self) -> WidgetBundler {
        let jsx_runtime_url = self.base_url.clone() + "/.scripts/jsx-runtime.js";
        let raw_apis_url = self.base_url.clone() + "/.scripts/raw-apis.js";
        let react_url = self.base_url.clone() + "/.scripts/react.js";
        let ui_url = self.base_url.clone() + "/.scripts/ui.js";

        let bundler_options = BundlerOptions {
            input: Some(vec![self.entry.into()]),
            cwd: Some(self.root),
            format: Some(OutputFormat::Esm),
            platform: Some(Platform::Browser),
            minify: Some(true),
            jsx: Some(Jsx::Enable(JsxOptions {
                runtime: JsxRuntime::Automatic,
                import_source: Some("@deskulpt-test/emotion".to_string()),
                ..Default::default()
            })),
            external: Some(
                vec![
                    jsx_runtime_url.clone(),
                    raw_apis_url.clone(),
                    react_url.clone(),
                    ui_url.clone(),
                    self.apis_blob_url.clone(),
                ]
                .into(),
            ),
            ..Default::default()
        };

        let alias_plugin = AliasPlugin {
            entries: vec![
                Alias {
                    find: StringOrRegex::String("@deskulpt-test/emotion/jsx-runtime".to_string()),
                    replacement: jsx_runtime_url,
                },
                Alias {
                    find: StringOrRegex::String("@deskulpt-test/raw-apis".to_string()),
                    replacement: raw_apis_url,
                },
                Alias {
                    find: StringOrRegex::String("@deskulpt-test/react".to_string()),
                    replacement: react_url,
                },
                Alias {
                    find: StringOrRegex::String("@deskulpt-test/ui".to_string()),
                    replacement: ui_url,
                },
                Alias {
                    find: StringOrRegex::String("@deskulpt-test/apis".to_string()),
                    replacement: self.apis_blob_url,
                },
            ],
        };

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
    /// Bundle the widget into a single ESM code string.
    pub async fn bundle(&mut self) -> Result<String> {
        let result = self.bundler.generate().await.map_err(|e| {
            anyhow!(e
                .into_vec()
                .iter()
                .map(|diagnostic| diagnostic.to_diagnostic().to_string())
                .collect::<Vec<String>>()
                .join("\n"))
        })?;
        assert!(
            result.assets.len() == 1,
            "Expected 1 bundled output, found {}",
            result.assets.len()
        );
        let output = &result.assets[0];
        let code = match output {
            Output::Asset(asset) => asset.source.clone().try_into_string()?,
            Output::Chunk(chunk) => chunk.code.clone(),
        };
        Ok(code)
    }
}
