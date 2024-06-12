//! This module implements the path resolver for the bundler.

use super::EXTENSIONS;
use anyhow::{bail, Error};
use path_clean::PathClean;
use std::path::{Component, Path, PathBuf};
use swc_bundler::Resolve;
use swc_common::FileName;
use swc_ecma_loader::resolve::Resolution;

/// The Deskulpt-customized path resolver for SWC bundler.
///
/// It is in charge of resolving the module specifiers in the import statements. Note
/// that module specifiers that are ignored in the first place will not go through this
/// resolver at all.
///
/// This path resolver manages to solve only relative/absolute path imports (with or
/// without extension). See [`EXTENSIONS`] for the list of extensions supported. It is,
/// however, designed to rehext the following types of imports:
///
/// - URL imports, because they require network connection. Vendor the source code to
///   local instead.
/// - Node module resolution, because it requires a node environment. Deskulpt supports
///   bundling external dependencies separately where one can resolve node modules.
/// - Imports that go beyond the root, because of security concerns.
pub(super) struct PathResolver {
    /// The root directory of the widget to bundle.
    pub(super) root: PathBuf,
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

    /// Helper function to validate a module specifier.
    ///
    /// The base directory is the directory from which to look for the specifier.
    /// Absolute/relative path specifiers are accepted, while node modules and URL
    /// specifiers are rejected.
    fn validate_specifer(
        &self,
        base_dir: &Path,
        module_specifier: &str,
    ) -> Result<PathBuf, Error> {
        let spec_path = Path::new(module_specifier);

        if spec_path.is_absolute() {
            return Ok(spec_path.clean());
        }

        // If not absolute, then it should be either relative, a node module, or a URL;
        // we support only relative import among these types
        let mut components = spec_path.components();
        if let Some(Component::CurDir | Component::ParentDir) = components.next() {
            return Ok(base_dir.join(module_specifier).clean());
        }

        bail!(
            "node_modules imports should be explicitly included in package.json to \
            avoid being bundled at runtime; URL imports are not supported, one should \
            vendor its source to local and use a relative import instead"
        )
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

        let path = self.validate_specifer(base_dir, module_specifier)?;

        // Try to resolve by treating `path` as a file first, otherwise try by
        // looking for an `index` file under `path` as a directory
        let resolved_path = self
            .resolve_as_file(&path)
            .or_else(|_| self.resolve_as_file(&path.join("index")))?;

        // Reject if the resolved path goes beyond the root
        let resolved_path = resolved_path.canonicalize()?;
        if !resolved_path.starts_with(&self.root) {
            bail!(
                "Relative imports should not go beyond the root '{}'",
                self.root.display(),
            );
        }
        Ok(FileName::Real(resolved_path))
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
