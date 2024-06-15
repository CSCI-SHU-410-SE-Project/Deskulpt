//! This module contains utilities that does not fit into any other module.

use crate::states::CanvasClickThroughState;
use anyhow::{bail, Error};
use serde::Serialize;
use std::{collections::HashMap, path::Path, time::Instant};
use tauri::{async_runtime, AppHandle, Manager, Runtime};
use tauri_plugin_shell::ShellExt;

/// Mapping from widget IDs to corresponding data.
pub(crate) type IdMap<T> = HashMap<String, T>;

/// Toast kind of the "show-toast" event.
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum ToastKind {
    Success,
}

/// Payload of the "show-toast" event.
#[derive(Serialize, Clone)]
struct ShowToastPayload {
    kind: ToastKind,
    message: String,
}

/// Result of executing a shell command.
#[derive(Serialize)]
pub(crate) struct ShellCommandResult {
    /// The elapsed time of executing the command in seconds.
    pub(crate) time: f64,
    /// Whether the command is successful.
    pub(crate) success: bool,
    /// The exit code of the command.
    pub(crate) code: Option<i32>,
    /// The standard output of the command.
    pub(crate) stdout: String,
    /// The standard error of the command.
    pub(crate) stderr: String,
}

/// Toggle the click-through state of the canvas window.
///
/// This will toggle whether the canvas window ignores cursor events and update the
/// state accordingly. If the canvas is toggled to not click-through, it will try to
/// regain focus automatically. The function will fail if:
///
/// - The canvas window is not found.
/// - Fails to set the canvas to ignore/unignore cursor events.
pub(crate) fn toggle_click_through_state<R: Runtime>(
    app_handle: &AppHandle<R>,
) -> Result<(), Error> {
    let canvas = match app_handle.get_webview_window("canvas") {
        Some(canvas) => canvas,
        None => bail!("Canvas window not found"),
    };

    let click_through_state = &app_handle.state::<CanvasClickThroughState<R>>();
    let mut click_through = click_through_state.0.lock().unwrap();
    let prev_can_click_through = click_through.yes();

    // Try to toggle the click through state
    canvas.set_ignore_cursor_events(!prev_can_click_through)?;
    click_through.toggle();

    // If the canvas is previously click-through, meaning that it is now set to not
    // click-through, try to regain focus to avoid flickering on the first click
    if prev_can_click_through {
        let _ = canvas.set_focus(); // Consume any error because this is not critical
    }

    // Try to let canvas show the toast message
    let _ = app_handle.emit_to(
        "canvas",
        "show-toast",
        ShowToastPayload {
            kind: ToastKind::Success,
            message: format!(
                "Canvas {}.",
                if prev_can_click_through { "floated" } else { "sunk" }
            ),
        },
    );
    Ok(())
}

/// Execute a shell command.
///
/// This will launch a command prompt on Windows or a bash shell on Unix in the
/// specified working directory and execute the given command via Tauri async runtime.
/// This function will not raise an error regardless of whether the command succeeds or
/// not, unless the internal execution failed that cannot be recovered or recorded.
pub(crate) fn run_shell_command<R: Runtime>(
    app_handle: &AppHandle<R>,
    cwd: &Path,
    cmd: &str,
) -> ShellCommandResult {
    let now = Instant::now();
    let shell = app_handle.shell();

    // Use command prompt on Windows or bash otherwise
    let (alias, flag) =
        if cfg!(target_os = "windows") { ("cmd", "/C") } else { ("sh", "-c") };

    let output = async_runtime::block_on(async move {
        shell.command(alias).current_dir(cwd).arg(flag).arg(cmd).output().await.unwrap()
    });

    ShellCommandResult {
        time: now.elapsed().as_secs_f64(),
        success: output.status.success(),
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
}
