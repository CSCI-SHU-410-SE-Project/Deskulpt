#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::path::{Path, PathBuf};

use path_clean::PathClean;

pub mod assert;
pub mod mock;

/// Absolutize a relative path within the fixtures directory.
pub fn fixture_path<P: AsRef<Path>>(path: P) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(path)
        .clean()
}
