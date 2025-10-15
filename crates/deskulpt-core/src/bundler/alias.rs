//! Deskulpt alias plugin for rolldown.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Error;
use rolldown::plugin::{
    HookResolveIdArgs, HookResolveIdOutput, HookResolveIdReturn, HookUsage, Plugin, PluginContext,
    PluginContextResolveOptions,
};
use tracing::{debug, error};

/// Deskulpt alias plugin.
///
/// This is a simplified version of the rolldown built-in alias plugin since we
/// only need a subset of its functionalities.
///
/// Based on the given alias mapping, this plugin will replace the original
/// imports with the aliased imports. Note that the aliased imports need to be
/// either resolvable or externalized to avoid bundling errors.
#[derive(Debug)]
pub struct AliasPlugin {
    /// Widget identifier for logging context.
    widget_id: Arc<str>,
    /// The alias mapping from original imports to aliased imports.
    mapping: HashMap<String, String>,
}

impl AliasPlugin {
    pub fn new(widget_id: Arc<str>, mapping: HashMap<String, String>) -> Self {
        Self { widget_id, mapping }
    }
}

impl Plugin for AliasPlugin {
    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("deskulpt:alias")
    }

    async fn resolve_id(
        &self,
        ctx: &PluginContext,
        args: &HookResolveIdArgs<'_>,
    ) -> HookResolveIdReturn {
        let importee = args.specifier;
        let alias = match self.mapping.get(importee) {
            Some(alias) => alias,
            None => return Ok(None),
        };

        let importer = args
            .importer
            .map(str::to_string)
            .unwrap_or_else(|| "<entry>".to_string());
        debug!(
            widget_id = %self.widget_id,
            importer = %importer,
            specifier = %importee,
            alias = %alias,
            "Applying widget alias"
        );

        let resolved_result = ctx
            .resolve(
                alias,
                args.importer,
                Some(PluginContextResolveOptions {
                    import_kind: args.kind,
                    is_entry: args.is_entry,
                    skip_self: true,
                    custom: Arc::clone(&args.custom),
                }),
            )
            .await;

        let resolved_result = match resolved_result {
            Ok(result) => result,
            Err(err) => {
                error!(
                    widget_id = %self.widget_id,
                    importer = %importer,
                    specifier = %importee,
                    alias = %alias,
                    error = ?err,
                    "Failed to request alias resolution"
                );
                return Err(err);
            },
        };

        let resolved_id = match resolved_result {
            Ok(resolved) => resolved,
            Err(err) => {
                error!(
                    widget_id = %self.widget_id,
                    importer = %importer,
                    specifier = %importee,
                    alias = %alias,
                    error = %err,
                    "Alias resolution failed"
                );
                return Err(Error::from(err));
            },
        };

        debug!(
            widget_id = %self.widget_id,
            importer = %importer,
            specifier = %importee,
            alias = %alias,
            resolved = %resolved_id.id,
            "Alias resolved"
        );

        Ok(Some(HookResolveIdOutput {
            id: resolved_id.id,
            ..Default::default()
        }))
    }

    fn register_hook_usage(&self) -> HookUsage {
        HookUsage::ResolveId
    }
}
