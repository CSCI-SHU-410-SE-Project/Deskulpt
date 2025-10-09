use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::time::{Duration, SystemTime};

use tokio::fs;
use tokio::io::{self, AsyncRead, AsyncReadExt};
use tokio::process::Command as TokioCommand;
use tokio::task::JoinHandle;
use tokio::time::timeout;

use crate::states::{PackageManagerInfo, PackageManagerKind};

const INSTALL_TIMEOUT: Duration = Duration::from_secs(10 * 60);

/// Status of a dependency installation attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyInstallStatus {
    /// Dependencies were already up-to-date; no install was run.
    AlreadyUpToDate,
    /// Dependencies were successfully (re)installed.
    Installed,
}

/// Successful dependency installation result.
#[derive(Debug, Clone)]
pub struct DependencyInstallSuccess {
    pub status: DependencyInstallStatus,
    pub stdout: String,
    pub stderr: String,
}

/// Failed dependency installation result.
#[derive(Debug)]
pub struct DependencyInstallFailure {
    pub error: DependencyInstallError,
    pub stdout: String,
    pub stderr: String,
}

impl DependencyInstallFailure {
    pub fn new(error: DependencyInstallError) -> Self {
        Self {
            error,
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    fn with_output(error: DependencyInstallError, stdout: String, stderr: String) -> Self {
        Self {
            error,
            stdout,
            stderr,
        }
    }
}

/// Errors produced while attempting to install dependencies.
#[derive(Debug, thiserror::Error)]
pub enum DependencyInstallError {
    #[error("widget directory does not exist: {0}")]
    WidgetDirMissing(PathBuf),
    #[error("package.json not found at {0}")]
    PackageJsonMissing(PathBuf),
    #[error("failed to access {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("package.json at {path} is not valid JSON: {source}")]
    PackageJsonMalformed {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("package manager executable does not exist at {0}")]
    PackageManagerExecutableMissing(PathBuf),
    #[error("failed to spawn install command: {0}")]
    SpawnFailed(#[source] std::io::Error),
    #[error("failed to wait for install command: {0}")]
    WaitFailed(#[source] std::io::Error),
    #[error("failed to collect install command output: {0}")]
    OutputJoinFailed(#[source] tokio::task::JoinError),
    #[error("failed to read install command output: {0}")]
    OutputReadFailed(#[source] std::io::Error),
    #[error("installation timed out after {0:?}")]
    TimedOut(Duration),
    #[error("install command exited with status {status}")]
    CommandFailed { status: ExitStatus },
}

/// Install all dependencies for a widget directory using the selected package
/// manager.
///
/// The caller must provide the [`PackageManagerInfo`] returned by the detection
/// state. This function determines whether installation is necessary before
/// spawning the package manager process. Outputs from the install command are
/// captured and included in the returned status.
pub async fn install_dependencies(
    widget_dir: impl AsRef<Path>,
    manager: &PackageManagerInfo,
) -> Result<DependencyInstallSuccess, DependencyInstallFailure> {
    let widget_dir = widget_dir.as_ref();

    let widget_exists = fs::try_exists(widget_dir)
        .await
        .map_err(|error| DependencyInstallFailure::new(io_error(widget_dir, error)))?;
    if !widget_exists {
        return Err(DependencyInstallFailure::new(
            DependencyInstallError::WidgetDirMissing(widget_dir.to_path_buf()),
        ));
    }
    let widget_dir_meta = fs::metadata(widget_dir)
        .await
        .map_err(|error| DependencyInstallFailure::new(io_error(widget_dir, error)))?;
    if !widget_dir_meta.is_dir() {
        return Err(DependencyInstallFailure::new(
            DependencyInstallError::WidgetDirMissing(widget_dir.to_path_buf()),
        ));
    }

    let manager_exists = fs::try_exists(&manager.executable)
        .await
        .map_err(|error| DependencyInstallFailure::new(io_error(&manager.executable, error)))?;
    if !manager_exists {
        return Err(DependencyInstallFailure::new(
            DependencyInstallError::PackageManagerExecutableMissing(manager.executable.clone()),
        ));
    }
    let manager_meta = fs::metadata(&manager.executable)
        .await
        .map_err(|error| DependencyInstallFailure::new(io_error(&manager.executable, error)))?;
    if !manager_meta.is_file() {
        return Err(DependencyInstallFailure::new(
            DependencyInstallError::PackageManagerExecutableMissing(manager.executable.clone()),
        ));
    }

    let package_json = widget_dir.join("package.json");
    let package_exists = fs::try_exists(&package_json)
        .await
        .map_err(|error| DependencyInstallFailure::new(io_error(&package_json, error)))?;
    if !package_exists {
        return Err(DependencyInstallFailure::new(
            DependencyInstallError::PackageJsonMissing(package_json.clone()),
        ));
    }

    let package_bytes = fs::read(&package_json)
        .await
        .map_err(|error| DependencyInstallFailure::new(io_error(&package_json, error)))?;

    if let Err(source) = serde_json::from_slice::<serde_json::Value>(&package_bytes) {
        return Err(DependencyInstallFailure::new(
            DependencyInstallError::PackageJsonMalformed {
                path: package_json.clone(),
                source,
            },
        ));
    }

    let package_meta = fs::metadata(&package_json)
        .await
        .map_err(|error| DependencyInstallFailure::new(io_error(&package_json, error)))?;
    let package_modified = package_meta
        .modified()
        .map_err(|error| DependencyInstallFailure::new(io_error(&package_json, error)))?;

    let node_modules = widget_dir.join("node_modules");
    let requirement =
        determine_install_requirement(widget_dir, &node_modules, manager.kind, package_modified)
            .await
            .map_err(DependencyInstallFailure::new)?;

    if !requirement.needs_install {
        return Ok(DependencyInstallSuccess {
            status: DependencyInstallStatus::AlreadyUpToDate,
            stdout: String::new(),
            stderr: String::new(),
        });
    }

    let command_output = match run_install_command(widget_dir, &manager.executable).await {
        Ok(output) => output,
        Err(failure) => {
            let mut failure = failure;
            if !requirement.node_modules_preexisting {
                if let Err(error) = cleanup_node_modules(&node_modules).await {
                    append_cleanup_error(&mut failure.stderr, &node_modules, error);
                }
            }
            return Err(failure);
        },
    };

    if !command_output.status.success() {
        let mut failure = DependencyInstallFailure::with_output(
            DependencyInstallError::CommandFailed {
                status: command_output.status,
            },
            command_output.stdout,
            command_output.stderr,
        );

        if !requirement.node_modules_preexisting {
            if let Err(error) = cleanup_node_modules(&node_modules).await {
                append_cleanup_error(&mut failure.stderr, &node_modules, error);
            }
        }

        return Err(failure);
    }

    Ok(DependencyInstallSuccess {
        status: DependencyInstallStatus::Installed,
        stdout: command_output.stdout,
        stderr: command_output.stderr,
    })
}

struct InstallRequirement {
    needs_install: bool,
    node_modules_preexisting: bool,
}

struct CommandExecution {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

async fn run_install_command(
    widget_dir: &Path,
    executable: &Path,
) -> Result<CommandExecution, DependencyInstallFailure> {
    let mut command = TokioCommand::new(executable);
    command.arg("install");
    command.current_dir(widget_dir);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command.kill_on_drop(true);

    let mut child = command.spawn().map_err(|error| {
        DependencyInstallFailure::with_output(
            DependencyInstallError::SpawnFailed(error),
            String::new(),
            String::new(),
        )
    })?;

    let stdout_handle = spawn_reader(child.stdout.take());
    let stderr_handle = spawn_reader(child.stderr.take());

    let wait_result = timeout(INSTALL_TIMEOUT, child.wait()).await;
    let mut status: Option<ExitStatus> = None;
    let mut error: Option<DependencyInstallError> = None;

    match wait_result {
        Ok(Ok(exit_status)) => status = Some(exit_status),
        Ok(Err(wait_error)) => error = Some(DependencyInstallError::WaitFailed(wait_error)),
        Err(_) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            error = Some(DependencyInstallError::TimedOut(INSTALL_TIMEOUT));
        },
    };

    let stdout_bytes = collect_output(stdout_handle).await;
    let stderr_bytes = collect_output(stderr_handle).await;

    let mut stdout = String::new();
    let mut stderr = String::new();

    match stdout_bytes {
        Ok(bytes) => stdout = String::from_utf8_lossy(&bytes).to_string(),
        Err(e) => {
            error.get_or_insert(e);
        },
    };
    match stderr_bytes {
        Ok(bytes) => stderr = String::from_utf8_lossy(&bytes).to_string(),
        Err(e) => {
            error.get_or_insert(e);
        },
    };

    if let Some(error) = error {
        return Err(DependencyInstallFailure::with_output(error, stdout, stderr));
    }

    let status = status.expect("exit status available when no error");

    Ok(CommandExecution {
        status,
        stdout,
        stderr,
    })
}

async fn collect_output(
    handle: JoinHandle<io::Result<Vec<u8>>>,
) -> Result<Vec<u8>, DependencyInstallError> {
    let bytes = handle
        .await
        .map_err(DependencyInstallError::OutputJoinFailed)?
        .map_err(DependencyInstallError::OutputReadFailed)?;
    Ok(bytes)
}

async fn determine_install_requirement(
    widget_dir: &Path,
    node_modules: &Path,
    kind: PackageManagerKind,
    package_modified: SystemTime,
) -> Result<InstallRequirement, DependencyInstallError> {
    let exists = fs::try_exists(node_modules)
        .await
        .map_err(|error| io_error(node_modules, error))?;
    if !exists {
        return Ok(InstallRequirement {
            needs_install: true,
            node_modules_preexisting: false,
        });
    }

    let node_meta = fs::metadata(node_modules)
        .await
        .map_err(|error| io_error(node_modules, error))?;
    let node_modified = node_meta
        .modified()
        .map_err(|error| io_error(node_modules, error))?;

    if node_modified < package_modified {
        return Ok(InstallRequirement {
            needs_install: true,
            node_modules_preexisting: true,
        });
    }

    if let Some(lock_modified) = newest_lock_mtime(widget_dir, kind).await? {
        if node_modified < lock_modified {
            return Ok(InstallRequirement {
                needs_install: true,
                node_modules_preexisting: true,
            });
        }
    }

    Ok(InstallRequirement {
        needs_install: false,
        node_modules_preexisting: true,
    })
}

async fn newest_lock_mtime(
    widget_dir: &Path,
    kind: PackageManagerKind,
) -> Result<Option<SystemTime>, DependencyInstallError> {
    let mut newest: Option<SystemTime> = None;

    for candidate in lock_file_candidates(kind) {
        let path = widget_dir.join(candidate);
        match fs::metadata(&path).await {
            Ok(metadata) => {
                if let Ok(modified) = metadata.modified() {
                    newest = Some(match newest {
                        Some(existing) => existing.max(modified),
                        None => modified,
                    });
                }
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(io_error(&path, error)),
        }
    }

    Ok(newest)
}

fn lock_file_candidates(kind: PackageManagerKind) -> &'static [&'static str] {
    match kind {
        PackageManagerKind::Pnpm => &["pnpm-lock.yaml"],
        PackageManagerKind::Npm => &["package-lock.json", "npm-shrinkwrap.json"],
        PackageManagerKind::Yarn => &["yarn.lock"],
        PackageManagerKind::Bun => &["bun.lockb"],
    }
}

fn spawn_reader<R>(pipe: Option<R>) -> JoinHandle<io::Result<Vec<u8>>>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        if let Some(mut pipe) = pipe {
            let mut buffer = Vec::new();
            pipe.read_to_end(&mut buffer).await?;
            Ok(buffer)
        } else {
            Ok(Vec::new())
        }
    })
}

async fn cleanup_node_modules(path: &Path) -> io::Result<()> {
    if fs::try_exists(path).await? {
        fs::remove_dir_all(path).await
    } else {
        Ok(())
    }
}

fn io_error(path: &Path, source: std::io::Error) -> DependencyInstallError {
    DependencyInstallError::Io {
        path: path.to_path_buf(),
        source,
    }
}

fn append_cleanup_error(stderr: &mut String, path: &Path, error: std::io::Error) {
    let message = format!(
        "Failed to clean up {} after installation failure: {error}",
        path.display()
    );
    if stderr.is_empty() {
        *stderr = message;
    } else {
        stderr.push('\n');
        stderr.push_str(&message);
    }
}
