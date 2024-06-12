//! This module contains the common bundling routines and utilities.

use super::{path_loader::PathLoader, path_resolver::PathResolver};
use anyhow::{bail, Error};
use std::{
    collections::{HashMap, HashSet},
    io::Write,
    path::Path,
};
use swc_atoms::Atom;
use swc_bundler::{Bundler, Hook, ModuleRecord};
use swc_common::{
    comments::SingleThreadedComments, sync::Lrc, FileName, Globals, Mark, SourceMap,
    Span,
};
use swc_ecma_ast::{KeyValueProp, Module, Program};
use swc_ecma_codegen::{
    text_writer::{JsWriter, WriteJs},
    Emitter,
};
use swc_ecma_transforms_react::jsx;
use swc_ecma_transforms_typescript::typescript;
use swc_ecma_visit::FoldWith;

/// Bundle the entry point into a raw module without further transforms.
///
/// If `replace_external` is true, import statements of external dependencies will be
/// converted into named imports from the external bundle file (without checking if it
/// exists). This should be used when aiming to bundle the widget source code. If
/// `replace_external` is false, the external dependencies will be left as is, same as
/// for default dependencies. This should be used when aiming to bundle external
/// dependencies which requires knowledge of which external dependencies are used.
pub(super) fn bundle_into_raw_module(
    root: &Path,
    target: &Path,
    dependency_map: &HashMap<String, String>,
    replace_external: bool,
    globals: &Globals,
    cm: Lrc<SourceMap>,
) -> Result<Module, Error> {
    if !target.exists() {
        bail!("Entry point does not exist: '{}'", target.display());
    }

    // Get the list of external modules not to resolve; this should include default
    // dependencies and (if any) external dependencies
    let external_modules = {
        let mut dependencies = HashSet::from([
            Atom::from("@deskulpt-test/apis"),
            Atom::from("@deskulpt-test/react"),
            Atom::from("@deskulpt-test/ui"),
        ]);
        dependencies.extend(dependency_map.keys().map(|k| Atom::from(k.clone())));
        Vec::from_iter(dependencies)
    };

    let mut bundler = Bundler::new(
        globals,
        cm.clone(),
        PathLoader { root, cm: cm.clone(), dependency_map, replace_external },
        // The path resolver produces paths with the \\?\ prefix on Windows, and since
        // we need to compare paths with the root we canonicalize the root path here to
        // get the same prefix; XXX not sure if there will be symlink issues
        PathResolver { root: root.canonicalize()?.to_path_buf() },
        // Do not resolve the external modules
        swc_bundler::Config { external_modules, ..Default::default() },
        Box::new(NoopHook),
    );

    // SWC bundler requires a map of entries to bundle, so we use the target path as the
    // key for convenience
    let entries = HashMap::from([(
        target.to_string_lossy().to_string(),
        FileName::Real(target.to_path_buf()),
    )]);

    // We expect a single bundle from a single entry point
    let mut bundles = bundler.bundle(entries)?;
    if bundles.len() != 1 {
        bail!("Expected a single bundle, got {}", bundles.len());
    }
    Ok(bundles.pop().unwrap().module)
}

/// Apply common transforms to a module.
///
/// This is initially designed to be used on the bundled module AST. It first applies
/// the Typescript transform to strip off types, then the JSX transform via the new
/// automatic runtime transform to interpret JSX syntax.
pub(super) fn apply_common_transforms(module: Module, cm: Lrc<SourceMap>) -> Module {
    let top_level_mark = Mark::new();
    let unresolved_mark = Mark::new();

    // Transform that removes TypeScript types; weirdly, this must be applied on a
    // program rather than a module; note that we use the verbatim module syntax to
    // avoid removing unused import statements in case they are needed
    let mut ts_transform = typescript::typescript(
        typescript::Config { verbatim_module_syntax: true, ..Default::default() },
        top_level_mark,
    );
    let program = Program::Module(module);
    let module = program.fold_with(&mut ts_transform).expect_module();

    // We use the automatic JSX transform (in contrast to the classic transform)
    // here so that there is no need to bring anything into scope just for syntax
    // which could be unused; to enable the `css` prop from Emotion, we specify the
    // import source to be `@deskulpt-test/emotion`, so that `jsx`, `jsxs`, and
    // `Fragment` will be imported from `@deskulpt-test/emotion/jsx-runtime`, which
    // will then be redirected to `src/.scripts/emotion-react-jsx-runtime.js` that
    // re-exports necessary runtime functions
    let mut jsx_transform = jsx::<SingleThreadedComments>(
        cm.clone(),
        None,
        swc_ecma_transforms_react::Options {
            runtime: Some(swc_ecma_transforms_react::Runtime::Automatic),
            import_source: Some("@deskulpt-test/emotion".to_string()),
            ..Default::default()
        },
        top_level_mark,
        unresolved_mark,
    );

    module.fold_with(&mut jsx_transform)
}

/// Emit a module into a buffer.
pub(super) fn emit_module_to_buf<W: Write>(module: Module, cm: Lrc<SourceMap>, buf: W) {
    let wr = JsWriter::new(cm.clone(), "\n", buf, None);
    let mut emitter = Emitter {
        cfg: swc_ecma_codegen::Config::default().with_minify(true),
        cm: cm.clone(),
        comments: None,
        wr: Box::new(wr) as Box<dyn WriteJs>,
    };
    emitter.emit_module(&module).unwrap();
}

/// A no-op hook for SWC bundler.
struct NoopHook;

impl Hook for NoopHook {
    fn get_import_meta_props(
        &self,
        _: Span,
        _: &ModuleRecord,
    ) -> Result<Vec<KeyValueProp>, Error> {
        // XXX: figure out a better way than panicking
        unimplemented!();
    }
}
