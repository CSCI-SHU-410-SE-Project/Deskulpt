//! Deskulpt alias plugin for rolldown.
//!
//! This is a simplified version of the rolldown built-in alias plugin since we
//! only need a subset of its functionalities.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use rolldown::plugin::{
    HookResolveIdArgs, HookResolveIdOutput, HookResolveIdReturn, Plugin, PluginContext,
    PluginContextResolveOptions,
};

/// Deskulpt alias plugin.
#[derive(Debug)]
pub struct AliasPlugin(
    /// Alias mapping from original imports to aliased imports.
    pub HashMap<String, String>,
);

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
        let update_id = match self.0.get(importee) {
            Some(alias) => alias,
            None => return Ok(None),
        };

        Ok(ctx
            .resolve(
                &update_id,
                None,
                Some(PluginContextResolveOptions {
                    import_kind: args.kind,
                    skip_self: true,
                    custom: Arc::clone(&args.custom),
                }),
            )
            .await?
            .map(|resolved_id| {
                Some(HookResolveIdOutput {
                    id: resolved_id.id.to_string(),
                    ..Default::default()
                })
            })?)
    }
}
