//! Command errors.

use serde::Serialize;

/// A serializable error type for the commands.
#[derive(thiserror::Error, Debug)]
pub enum CmdError {
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
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

impl specta::Type for CmdError {
    fn inline(
        type_map: &mut specta::TypeCollection,
        generics: specta::Generics,
    ) -> specta::datatype::DataType {
        <String as specta::Type>::inline(type_map, generics)
    }

    fn reference(
        type_map: &mut specta::TypeCollection,
        generics: &[specta::datatype::DataType],
    ) -> specta::datatype::reference::Reference {
        <String as specta::Type>::reference(type_map, generics)
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
