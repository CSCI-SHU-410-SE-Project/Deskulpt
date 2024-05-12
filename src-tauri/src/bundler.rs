//! The module implements the Deskulpt bundler based on SWC.
//!
//! Note that this is not a general-purpose bundler; it is specifically designed for
//! the use case of bundling Deskulpt widgets which has a custom set of dependency
//! rules and are (at least recommended to be) small.

use anyhow::{bail, Context, Error};
use path_clean::PathClean;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::{Component, Path, PathBuf},
};
use swc_atoms::Atom;
use swc_bundler::{Bundler, Hook, Load, ModuleData, ModuleRecord, Resolve};
use swc_common::{
    comments::SingleThreadedComments, errors::Handler, pass::Repeat, sync::Lrc,
    FileName, FilePathMapping, Globals, Mark, SourceMap, Span, GLOBALS,
};
use swc_ecma_ast::{KeyValueProp, Module, ModuleDecl, Program};
use swc_ecma_codegen::{
    text_writer::{JsWriter, WriteJs},
    Emitter,
};
use swc_ecma_loader::resolve::Resolution;
use swc_ecma_parser::{parse_file_as_module, EsConfig, Syntax, TsConfig};
use swc_ecma_transforms_optimization::simplify::dce;
use swc_ecma_transforms_react::jsx;
use swc_ecma_transforms_typescript::typescript;
use swc_ecma_visit::{
    as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith,
};
use tempfile::NamedTempFile;

/// The file extensions to try when an import is given without an extension
static EXTENSIONS: &[&str] = &["js", "jsx", "ts", "tsx"];

/// Bundle a widget into a single ESM string given its entry point.
///
/// The `dependency_map` argument is an optional mapping with keys being the module
/// specifiers to ignore. The import statements with these module specifiers will be
/// left as is in the bundled code without path resolution. This should commonly be the
/// list of external dependencies, since Deskulpt requires widget developers to bundle
/// their external dependencies (if any) to be included directly in the Webview.
pub(crate) fn bundle(
    root: &Path,
    target: &Path,
    apis_blob_url: String,
    dependency_map: &HashMap<String, String>,
) -> Result<String, Error> {
    if !target.exists() {
        bail!("Entry point does not exist: '{}'", target.display());
    }

    let globals = Globals::default();
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

    // Get the list of external modules not to resolve; this should include default
    // dependencies and (if any) external dependencies obtained from the dependency map
    let external_modules = {
        let mut dependencies = HashSet::from([
            Atom::from("@deskulpt-test/react"),
            Atom::from("@deskulpt-test/apis"),
        ]);
        dependencies.extend(dependency_map.keys().map(|k| Atom::from(k.clone())));
        Vec::from_iter(dependencies)
    };

    let mut bundler = Bundler::new(
        &globals,
        cm.clone(),
        PathLoader(cm.clone()),
        // The path resolver produces paths with the \\?\ prefix on Windows, and since
        // we need to compare paths with the root we canonicalize the root path here to
        // get the same prefix; XXX not sure if there will be symlink issues
        PathResolver { root: root.canonicalize()?.to_path_buf() },
        // We must disabled the default tree-shaking by the SWC bundler, otherwise it
        // will remove unused `React` variables, which is required by the JSX transform
        swc_bundler::Config {
            external_modules,
            disable_dce: true,
            ..Default::default()
        },
        Box::new(NoopHook),
    );

    // SWC bundler requires a map of entries to bundle; we provide a single entry point
    // and expect there to be only one generated bundle; we use the target path as the
    // key for convenience
    let mut entries = HashMap::new();
    entries.insert(
        target.to_string_lossy().to_string(),
        FileName::Real(target.to_path_buf()),
    );

    let mut bundles = bundler.bundle(entries)?;
    if bundles.len() != 1 {
        bail!("Expected a single bundle, got {}", bundles.len());
    }
    let module = bundles.pop().unwrap().module;

    let code = GLOBALS.set(&globals, || {
        let top_level_mark = Mark::new();
        let unresolved_mark = Mark::new();

        // Tree-shaking optimization in the bundler is disabled, so we need to manually
        // apply the transform; we need to retain the top level mark `React` because it
        // is needed by the JSX transform even if not explicitly used in the code
        let mut tree_shaking = Repeat::new(dce::dce(
            dce::Config { top_retain: vec![Atom::from("React")], ..Default::default() },
            unresolved_mark,
        ));

        // There are two types of JSX transforms ("classic" and "automatic"), see
        // https://legacy.reactjs.org/blog/2020/09/22/introducing-the-new-jsx-transform.html
        //
        // The "automatic" transform automatically imports from "react/jsx-runtime", but
        // this module is not available when runnning the bundled code in the browser,
        // so we have to use the "classic" transform instead. The "classic" transform
        // requires `React` to be in scope, which we can require users to bring into
        // scope by importing `import React from "@deskulpt-test/react";`.
        let mut jsx_transform = jsx::<SingleThreadedComments>(
            cm.clone(),
            None,
            Default::default(), // options, where runtime defaults to "classic"
            top_level_mark,
            unresolved_mark,
        );

        // Transform that removes TypeScript types; weirdly, this must be applied on a
        // program rather than a module; note that we use the verbatim module syntax to
        // avoid removing unused import statements (particularly the `React` import)
        let mut ts_transform = typescript::typescript(
            typescript::Config { verbatim_module_syntax: true, ..Default::default() },
            top_level_mark,
        );

        // We need to rename the imports of `@deskulpt-test/apis` to the blob URL which
        // wraps the widget APIs to avoid exposing the raw APIs that allow specifying
        // widget IDs; note that this transform should be done last to avoid messing up
        // with import resolution
        let mut wrap_apis = as_folder(ImportRenamer(
            [("@deskulpt-test/apis".to_string(), apis_blob_url)].into(),
        ));

        // Apply the module transformations
        let module = module
            .into_program()
            .fold_with(&mut ts_transform)
            .expect_module()
            .fold_with(&mut tree_shaking)
            .fold_with(&mut jsx_transform)
            .fold_with(&mut wrap_apis);

        // Emit the bundled module as string into a buffer
        let mut buf = vec![];
        {
            let wr = JsWriter::new(cm.clone(), "\n", &mut buf, None);
            let mut emitter = Emitter {
                cfg: swc_ecma_codegen::Config::default().with_minify(true),
                cm: cm.clone(),
                comments: None,
                wr: Box::new(wr) as Box<dyn WriteJs>,
            };
            emitter.emit_module(&module).unwrap();
        }
        String::from_utf8_lossy(&buf).to_string()
    });

    Ok(code)
}

/// An AST transformer that renames import module specifiers.
///
/// This should be wrapped within [`as_folder`].
struct ImportRenamer(HashMap<String, String>);

impl VisitMut for ImportRenamer {
    noop_visit_mut_type!();

    fn visit_mut_module_decl(&mut self, n: &mut ModuleDecl) {
        n.visit_mut_children_with(self);

        if let ModuleDecl::Import(import_decl) = n {
            let src = import_decl.src.value.to_string();
            if let Some(new_src) = self.0.get(&src) {
                import_decl.src.value = Atom::from(new_src.clone());
            }
        }
    }
}

/// Deskulpt-customized path loader for SWC bundler.
///
/// It is in charge of parsing the source file into a module AST. Note that transforms
/// are not applied here to avoid messing up per-file ASTs that can cause unexpected
/// bundling results.
struct PathLoader(Lrc<SourceMap>);

impl Load for PathLoader {
    fn load(&self, file: &FileName) -> Result<ModuleData, Error> {
        let path = match file {
            FileName::Real(path) => path,
            _ => unreachable!(),
        };
        let fm = self.0.load_file(path)?;

        let syntax = match path.extension() {
            Some(ext) if ext == "ts" || ext == "tsx" => {
                Syntax::Typescript(TsConfig { tsx: true, ..Default::default() })
            },
            _ => Syntax::Es(EsConfig { jsx: true, ..Default::default() }),
        };

        // Parse the file as a module; note that transformations are not applied here,
        // because per-file transformations may lead to unexpected results when bundled
        // together; instead, transformations are postponed until the bundling phase
        match parse_file_as_module(&fm, syntax, Default::default(), None, &mut vec![]) {
            Ok(module) => Ok(ModuleData { fm, module, helpers: Default::default() }),
            Err(err) => {
                // The error handler requires a destination for the emitter writer that
                // implements `Write`; a buffer implements `Write` but its borrowed
                // reference does not, causing the handler to take ownership of the
                // buffer, making us unable to read from it later (and the buffer is
                // made private in the handler); the workaround is to use a temporary
                // file and access its content later by its path (we require the file to
                // live only for a short time so this is relatively safe)
                let mut err_msg = String::new();
                {
                    let context = format!(
                        "Parsing error occurred but failed to emit the formatted error \
                        analysis; falling back to raw version: {err:?}"
                    );
                    let buffer = NamedTempFile::new().context(context.clone())?;
                    let buffer_path = buffer.path().to_path_buf();
                    let handler = Handler::with_emitter_writer(
                        Box::new(buffer),
                        Some(self.0.clone()),
                    );
                    err.into_diagnostic(&handler).emit();
                    File::open(buffer_path)
                        .context(context.clone())?
                        .read_to_string(&mut err_msg)
                        .context(context.clone())?;
                }
                bail!(err_msg);
            },
        }
    }
}

/// The Deskulpt-customized path resolver for SWC bundler.
///
/// It is in charge of resolving the module specifiers in the import statements. Note
/// that module specifiers that are ignored in the first place will not go through this
/// resolver at all.
///
/// This path resolver intends to resolve the following types of imports:
///
/// - Extension-less relative paths, e.g., `import foo from "./foo"`
/// - Relative paths, e.g., `import foo from "./foo.js"`
///
/// It is not designed to resolve the following types of imports:
///
/// - Absolute path imports, e.g., `import foo from "/foo"`
/// - URL imports, e.g., `import foo from "https://example.com/foo"`
/// - Node resolution imports, e.g., `import globals from "globals"`
/// - Relative imports that go beyond the root
struct PathResolver {
    root: PathBuf,
}

impl PathResolver {
    /// Helper function for resolving a path by treating it as a file.
    ///
    /// If `path` refers to a file then it is directly returned. Otherwise, `path` with
    /// each extension in [`EXTENSIONS`] is tried in order.
    fn resolve_as_file(&self, path: &Path) -> Result<PathBuf, Error> {
        if path.is_file() {
            // Early return if `path` is directly a file
            return Ok(path.to_path_buf());
        }

        if let Some(name) = path.file_name() {
            let mut ext_path = path.to_path_buf();
            let name = name.to_string_lossy();

            // Try all extensions we support for importing
            for ext in EXTENSIONS {
                ext_path.set_file_name(format!("{name}.{ext}"));
                if ext_path.is_file() {
                    return Ok(ext_path);
                }
            }
        }
        bail!("File resolution failed")
    }

    /// Helper function for the [`Resolve`] trait.
    ///
    /// Note that errors emitted here do not need to provide information about `base`
    /// and `module_specifier` because the call to this function should have already
    /// been wrapped in an SWC context that provides this information.
    fn resolve_filename(
        &self,
        base: &FileName,
        module_specifier: &str,
    ) -> Result<FileName, Error> {
        let base = match base {
            FileName::Real(v) => v,
            _ => bail!("Invalid base for resolution: '{base}'"),
        };

        // Determine the base directory (or `base` itself if already a directory)
        let base_dir = if base.is_file() {
            // If failed to get the parent directory then use the cwd
            base.parent().unwrap_or_else(|| Path::new("."))
        } else {
            base
        };

        let spec_path = Path::new(module_specifier);
        // Absolute paths are not supported
        if spec_path.is_absolute() {
            bail!("Absolute imports are not supported; use relative imports instead");
        }

        // If not absolute, then it should be either relative, a node module, or a URL;
        // we support only relative import among these types
        let mut components = spec_path.components();
        if let Some(Component::CurDir | Component::ParentDir) = components.next() {
            let path = base_dir.join(module_specifier).clean();

            // Try to resolve by treating `path` as a file first, otherwise try by
            // looking for an `index` file under `path` as a directory
            let resolved_path = self
                .resolve_as_file(&path)
                .or_else(|_| self.resolve_as_file(&path.join("index")))?;

            // Reject if the resolved path goes beyond the root
            if !resolved_path.starts_with(&self.root) {
                bail!(
                    "Relative imports should not go beyond the root '{}'",
                    self.root.display(),
                );
            }
            return Ok(FileName::Real(resolved_path));
        }

        bail!(
            "node_modules imports should be explicitly included in package.json to \
            avoid being bundled at runtime; URL imports are not supported, one should \
            vendor its source to local and use a relative import instead"
        )
    }
}

impl Resolve for PathResolver {
    fn resolve(
        &self,
        base: &FileName,
        module_specifier: &str,
    ) -> Result<Resolution, Error> {
        self.resolve_filename(base, module_specifier)
            .map(|filename| Resolution { filename, slug: None })
    }
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

/// Trait for converting an object into a [`Program`].
///
/// Some transforms requires a [`Program`] as input, so this trait is for providing a
/// better syntax for chaining the transforms, for instance:
///
/// ```ignore
/// module
///     .fold_with(&mut transform1)
///     .into_program()
///     .fold_with(&mut transform2) // `transform2` requires a `Program`
///     .expect_module()
///     .fold_with(&mut transform3)
/// ```
trait IntoProgram {
    /// Return a [`Program`] wrapping the object itself.
    fn into_program(self) -> Program;
}

impl IntoProgram for Module {
    fn into_program(self) -> Program {
        Program::Module(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{assert_err_eq, ChainReason};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use std::fs::{create_dir, read_to_string};
    use tempfile::{tempdir, TempDir};

    /// Get the absolute path to the fixture directory.
    ///
    /// The paths used within the SWC bundler are all canonicalized (and thus verbatim
    /// with the `\\?\` prefix on Windows), so canonicalize here to match them. Note
    /// that this is not the case elsewhere in the codebase.
    fn fixture_dir() -> PathBuf {
        Path::new("tests/fixtures/bundler").canonicalize().unwrap()
    }

    /// Setup a temporary directory for testing.
    ///
    /// This would create a temporary directory and an `input` directory inside it.
    fn setup_temp_dir() -> TempDir {
        let temp_dir = tempdir().unwrap();
        create_dir(temp_dir.path().join("input")).unwrap();
        temp_dir
    }

    #[rstest]
    // Entry does not use `React`, but we should not remove it
    #[case::no_react("no_react")]
    // Entry uses a `React` hook
    #[case::with_react_hook("with_react_hook")]
    // Entry imports a JS file with the extension
    #[case::import("import")]
    // Entry imports a JS file without specifying the extension
    #[case::import_no_ext("import_no_ext")]
    // Entry imports a JSX file without specifying the extension
    #[case::import_no_ext_jsx("import_no_ext_jsx")]
    // Entry imports a directory with `index.js`
    #[case::import_directory("import_directory")]
    // Entry defines `React` but the imported JSX file does not
    #[case::import_no_inner_react_def("import_no_inner_react_def")]
    // Entry does not define `React` but the imported JSX file does
    #[case::import_no_outer_react_def("import_no_outer_react_def")]
    fn test_bundle_ok(#[case] case: &str) {
        let case_dir = fixture_dir().join("_javascript").join(case);
        let bundle_root = case_dir.join("input");
        let result = bundle(
            &bundle_root,
            &bundle_root.join("index.jsx"),
            Default::default(),
            &Default::default(),
        )
        .expect("Expected bundling to succeed");

        let expected = read_to_string(case_dir.join("output.js")).unwrap();
        self::assert_eq!(result, expected);
    }

    #[rstest]
    // Basic TypeScript syntax
    #[case::types("types")]
    // Entry does not use `React`, but we should not remove it
    #[case::no_react("no_react")]
    // Importing types
    #[case::import_types("import_types")]
    fn test_bundle_ok_typescript(#[case] case: &str) {
        let case_dir = fixture_dir().join("_typescript").join(case);
        let bundle_root = case_dir.join("input");
        let result = bundle(
            &bundle_root,
            &bundle_root.join("index.tsx"),
            Default::default(),
            &Default::default(),
        )
        .expect("Expected bundling to succeed");

        let expected = read_to_string(case_dir.join("output.js")).unwrap();
        self::assert_eq!(result, expected);
    }

    #[rstest]
    fn test_bundle_ignore_external_dependencies() {
        // Test that specified external dependencies are left as is in the bundled code
        let case_dir = fixture_dir().join("external_deps");
        let bundle_root = case_dir.join("input");
        let external_deps = HashMap::from([
            ("os-name".to_string(), "^6.0.0".to_string()),
            ("matcher".to_string(), "^5.0.0".to_string()),
        ]);
        let result = bundle(
            &bundle_root,
            &bundle_root.join("index.jsx"),
            Default::default(),
            &external_deps,
        )
        .expect("Expected bundling to succeed");

        let expected = read_to_string(case_dir.join("output.js")).unwrap();
        self::assert_eq!(result, expected);
    }

    #[rstest]
    fn test_bundle_replace_apis_url() {
        // Test that the renaming of `@deskulpt-test/apis` to the blob URL is done
        let case_dir = fixture_dir().join("replace_apis");
        let bundle_root = case_dir.join("input");
        let result = bundle(
            &bundle_root,
            &bundle_root.join("index.jsx"),
            "blob://dummy-url".to_string(),
            &Default::default(),
        )
        .expect("Expected bundling to succeed");

        let expected = read_to_string(case_dir.join("output.js")).unwrap();
        self::assert_eq!(result, expected);
    }

    #[rstest]
    // Node modules import that are not specified as external dependencies
    #[case::import_node_modules(
        "import_node_modules",
        vec![
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve os-name from {}",
                fixture_dir().join("import_node_modules/input/index.jsx").display(),
            )),
            ChainReason::Exact(
                "node_modules imports should be explicitly included in package.json to \
                avoid being bundled at runtime; URL imports are not supported, one \
                should vendor its source to local and use a relative import instead"
                .to_string()
            ),
        ]
    )]
    // URL import
    #[case::import_url(
        "import_url",
        vec![
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve https://cdn.jsdelivr.net/npm/os-name@6.0.0/+esm from {}",
                fixture_dir().join("import_url/input/index.jsx").display(),
            )),
            ChainReason::Exact(
                "node_modules imports should be explicitly included in package.json to \
                avoid being bundled at runtime; URL imports are not supported, one \
                should vendor its source to local and use a relative import instead"
                .to_string()
            ),
        ]
    )]
    // Relative import that goes beyond the root
    #[case::import_beyond_root(
        "import_beyond_root",
        vec![
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve ../../foo from {}",
                fixture_dir().join("import_beyond_root/input/index.jsx").display(),
            )),
            ChainReason::Exact(format!(
                "Relative imports should not go beyond the root '{}'",
                fixture_dir().join("import_beyond_root/input").display(),
            )),
        ]
    )]
    // Entry file does not exist
    #[case::entry_not_exist(
        "entry_not_exist",
        vec![
            ChainReason::Exact(format!(
                "Entry point does not exist: '{}'",
                fixture_dir().join("entry_not_exist/input/index.jsx").display()
            ))
        ]
    )]
    // Bad syntax that cannot be parsed
    #[case::bad_syntax(
        "bad_syntax",
        vec![
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("Bundler.load() failed".to_string()),
            ChainReason::Exact(format!(
                "Bundler.loader.load({}) failed",
                fixture_dir().join("bad_syntax/input/index.jsx")
                    .canonicalize()
                    .unwrap()
                    .display()
                )),
            ChainReason::Regex("error: Expected ';', '}' or <eof>".to_string()),
        ]
    )]
    fn test_bundle_error(#[case] case: &str, #[case] expected_error: Vec<ChainReason>) {
        let case_dir = fixture_dir().join(case);
        let bundle_root = case_dir.join("input");
        let error = bundle(
            &bundle_root,
            &bundle_root.join("index.jsx"),
            Default::default(),
            &Default::default(),
        )
        .expect_err("Expected bundling error");
        assert_err_eq(error, expected_error);
    }

    #[rstest]
    #[should_panic]
    fn test_bundle_import_meta_panic() {
        // Test that accessing `import.meta` is not supported
        let bundle_root = fixture_dir().join("import_meta/input");
        let _ = bundle(
            &bundle_root,
            &bundle_root.join("index.jsx"),
            Default::default(),
            &Default::default(),
        );
    }

    #[rstest]
    fn test_bundle_absolute_import_error() {
        // Test that an absolute import raises a proper error
        let temp_dir = setup_temp_dir();
        let bundle_root = temp_dir.path().canonicalize().unwrap().join("input");
        let utils_path = bundle_root.join("utils.js");
        std::fs::write(&utils_path, "export const foo = 42;").unwrap();

        // Create an entry file that imports the absolute path of `utils.js`; note that
        // we use debugging format for the path because otherwise path separator "\" on
        // Windows will not be escaped
        let entry_path = bundle_root.join("index.jsx");
        println!("import {{ foo }} from {utils_path:?};");
        std::fs::write(&entry_path, format!("import {{ foo }} from {utils_path:?};"))
            .unwrap();

        let error =
            bundle(&bundle_root, &entry_path, Default::default(), &Default::default())
                .expect_err("Expected bundling error");
        let expected_error = vec![
            ChainReason::Exact("load_transformed failed".to_string()),
            ChainReason::Exact("failed to analyze module".to_string()),
            ChainReason::Exact(format!(
                "failed to resolve {} from {}",
                utils_path.display(),
                entry_path.display()
            )),
            ChainReason::Exact(
                "Absolute imports are not supported; use relative imports instead"
                    .to_string(),
            ),
        ];
        assert_err_eq(error, expected_error);
    }
}
