//! State management for Node.js package manager detection.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use anyhow::{bail, Context, Error, Result};
use dunce::simplified;
use serde::ser::SerializeStruct;
use serde::Serialize;
use tauri::{App, AppHandle, Manager, Runtime};
use which::which;

/// Supported Node.js package managers in decreasing priority order.
const PACKAGE_MANAGER_CANDIDATES: &[Candidate] = &[
    Candidate::new(PackageManagerKind::Pnpm),
    Candidate::new(PackageManagerKind::Npm),
    Candidate::new(PackageManagerKind::Yarn),
    Candidate::new(PackageManagerKind::Bun),
];

/// Managed state for package manager detection.
struct PackageManagerState(Arc<PackageManagerDetection>);

/// Extension trait for operations related to package manager detection.
pub trait PackageManagerStateExt<R: Runtime>: Manager<R> {
    /// Detect available package managers and cache the result.
    fn manage_package_manager(&self) -> Result<()> {
        let detection = detect_package_managers()?;
        self.manage(PackageManagerState(Arc::new(detection)));
        Ok(())
    }

    /// Get the cached detection result.
    fn package_manager_detection(&self) -> Arc<PackageManagerDetection> {
        self.state::<PackageManagerState>().inner().0.clone()
    }

    /// Get the preferred package manager.
    fn selected_package_manager(&self) -> PackageManagerInfo {
        self.package_manager_detection().selected.clone()
    }
}

impl<R: Runtime> PackageManagerStateExt<R> for App<R> {}
impl<R: Runtime> PackageManagerStateExt<R> for AppHandle<R> {}

/// Supported package managers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageManagerKind {
    Pnpm,
    Npm,
    Yarn,
    Bun,
}

impl PackageManagerKind {
    /// Get the executable name for the package manager.
    fn command(self) -> &'static str {
        match self {
            Self::Pnpm => "pnpm",
            Self::Npm => "npm",
            Self::Yarn => "yarn",
            Self::Bun => "bun",
        }
    }
}

/// Details of a detected package manager executable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageManagerInfo {
    pub kind: PackageManagerKind,
    pub executable: PathBuf,
    pub version: String,
}

impl Serialize for PackageManagerInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PackageManagerInfo", 3)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("executable", &self.executable.to_string_lossy())?;
        state.serialize_field("version", &self.version)?;
        state.end()
    }
}

/// Summary of package manager detection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageManagerDetection {
    pub selected: PackageManagerInfo,
    pub available: Vec<PackageManagerInfo>,
}

/// Detection outcome for a candidate package manager.
enum CandidateDetection {
    Found(PackageManagerInfo),
    NotFound,
    Error { path: PathBuf, source: Error },
}

/// Metadata describing a package manager candidate.
struct Candidate {
    kind: PackageManagerKind,
}

impl Candidate {
    const fn new(kind: PackageManagerKind) -> Self {
        Self { kind }
    }

    fn kind(&self) -> PackageManagerKind {
        self.kind
    }

    fn command(&self) -> &'static str {
        self.kind.command()
    }
}

/// Detect and validate supported package managers.
fn detect_package_managers() -> Result<PackageManagerDetection> {
    let mut available = Vec::new();
    let mut failures = Vec::new();

    for candidate in PACKAGE_MANAGER_CANDIDATES {
        match detect_candidate(candidate) {
            CandidateDetection::Found(info) => available.push(info),
            CandidateDetection::NotFound => failures.push(format!(
                "{}: executable not found on PATH",
                candidate.command()
            )),
            CandidateDetection::Error { path, source } => failures.push(format!(
                "{}: found at {} but failed to run `--version`: {source}",
                candidate.command(),
                path.display()
            )),
        }
    }

    if let Some(selected) = available.first().cloned() {
        return Ok(PackageManagerDetection {
            selected,
            available,
        });
    }

    let mut message = String::from(
        "No supported Node.js package manager found. Install pnpm, npm, yarn, or bun and ensure \
         the executable is available on PATH.",
    );
    if !failures.is_empty() {
        message.push_str(" Details:\n");
        for failure in failures {
            message.push_str(" - ");
            message.push_str(&failure);
            message.push('\n');
        }
        message.pop(); // Remove trailing newline added by the loop
    }
    bail!(message);
}

/// Detect a specific package manager candidate.
fn detect_candidate(candidate: &Candidate) -> CandidateDetection {
    let path = match which(candidate.command()) {
        Ok(path) => path,
        Err(_) => return CandidateDetection::NotFound,
    };
    let path = simplified(path.as_path()).to_path_buf();

    match read_version(&path) {
        Ok(version) => CandidateDetection::Found(PackageManagerInfo {
            kind: candidate.kind(),
            executable: path,
            version,
        }),
        Err(source) => CandidateDetection::Error { path, source },
    }
}

/// Run the version command for a package manager executable.
fn read_version(executable: &Path) -> Result<String> {
    let output = Command::new(executable)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("failed to execute `{}`", executable.display()))?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let exit = output.status.code().map_or_else(
            || "terminated by signal".to_string(),
            |code| code.to_string(),
        );
        bail!(
            "`--version` exited with code {}. stdout: {}; stderr: {}",
            exit,
            stdout.trim(),
            stderr.trim()
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = stdout.trim();
    if version.is_empty() {
        bail!("`--version` produced empty output");
    }
    Ok(version.to_string())
}
