//! Command errors.

use serde::Serialize;

/// A serializable error type for the commands.
#[derive(thiserror::Error, Debug)]
pub enum CmdError {
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("MessagePack decode error: {0}")]
    MessagePackDecode(#[from] rmp_serde::decode::Error),
    #[error("Global shortcut error: {0}")]
    TauriPluginGlobalShortcut(#[from] tauri_plugin_global_shortcut::Error),
}

impl Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// The return type of all Deskulpt core commands.
pub type CmdResult<T> = Result<T, CmdError>;

/// [anyhow::anyhow!] that returns a [CmdError].
macro_rules! cmderr {
    ($msg:literal $(,)?) => {
        $crate::commands::error::CmdError::from(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        $crate::commands::error::CmdError::from(anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::commands::error::CmdError::from(anyhow::anyhow!($fmt, $($arg)*))
    };
}

/// [anyhow::bail!] that early returns a [CmdError].
macro_rules! cmdbail {
    ($msg:literal $(,)?) => {
        return Err($crate::commands::error::cmderr!($msg))
    };
    ($err:expr $(,)?) => {{
        return Err($crate::commands::error::cmderr!($err))
    }};
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::commands::error::cmderr!($fmt, $($arg)*))
    };
}

pub(super) use {cmdbail, cmderr};
