use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::{anyhow, Context, Result};
use tracing::{info, warn};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::{SubscriberInitExt, SubscriberInitExtError};
use tracing_subscriber::{fmt, EnvFilter};

static FILE_GUARD: OnceLock<non_blocking::WorkerGuard> = OnceLock::new();
static INITIALIZED: OnceLock<()> = OnceLock::new();

/// Initialize global logging for Deskulpt.
///
/// Logs are written to both stdout and a rolling file located under the given
/// `log_dir`. The log level can be overridden by the `RUST_LOG` environment
/// variable; otherwise we default to info level while enabling debug logs for
/// the widget bundler pipeline.
pub fn init(log_dir: &Path) -> Result<()> {
    INITIALIZED
        .get_or_try_init(|| {
            fs::create_dir_all(log_dir).context("Failed to create log directory")?;

            let file_appender = rolling::daily(log_dir, "widget-bundler.log");
            let (file_writer, guard) = non_blocking::non_blocking(file_appender);

            // Keep the guard alive so that logs are flushed properly. Ignore the
            // result if we've already set it, which can happen in unit tests.
            let _ = FILE_GUARD.set(guard);

            let env_filter = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info,deskulpt_core::bundler=debug"))
                .context("Failed to configure log level filter")?;

            let console_layer = fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_file(true);

            let file_layer = fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_writer(file_writer);

            if let Err(err) = tracing_subscriber::registry()
                .with(env_filter)
                .with(console_layer)
                .with(file_layer)
                .try_init()
            {
                // If a global subscriber has already been set, fall back to emitting a
                // warning and continue so we don't fail the app startup.
                if err.already_set() {
                    warn!(
                        "Global tracing subscriber already initialized; reusing existing \
                         subscriber"
                    );
                } else {
                    return Err(anyhow!(err));
                }
            } else {
                info!(path = %log_dir.display(), "Deskulpt logging initialized");
            }

            Ok(())
        })
        .map(|_| ())
}

/// Extension trait to interrogate `SetGlobalDefaultError` without importing the
/// concrete type into callers.
trait InitErrorExt {
    fn already_set(&self) -> bool;
}

impl InitErrorExt for SubscriberInitExtError {
    fn already_set(&self) -> bool {
        matches!(self, SubscriberInitExtError::SetGlobalDefaultError(_))
    }
}
