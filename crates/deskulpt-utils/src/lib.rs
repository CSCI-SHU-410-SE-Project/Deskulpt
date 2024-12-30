//! The module implements the Deskulpt bundler based on SWC.
//!
//! Note that this is not a general-purpose bundler; it is specifically designed
//! for the use case of bundling Deskulpt widgets and their external
//! dependencies.

#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/crates/deskulpt/icons/icon.png"
)]

use std::collections::HashMap;

use anyhow::Error;

pub type CommandOut<T> = Result<T, String>;

pub type IdMap<T> = HashMap<String, T>;

/// Stringify an [`Error`].
///
/// This is a similar representation to that one gets by default if returning an
/// error from `fn main`, except that it never includes the backtrace to not be
/// too verbose.
#[doc(hidden)]
pub fn stringify_anyhow(err: Error) -> String {
    err.chain()
        .enumerate()
        .map(|(index, reason)| match index {
            0 => reason.to_string(),
            1 => format!("\nCaused by:\n  1: {reason}"),
            _ => format!("  {index}: {reason}"),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Get a formatted error string.
///
/// It accepts any arguments that can be passed to [`anyhow::anyhow`].
#[macro_export]
macro_rules! cmderr {
    ($msg:literal $(,)?) => {
        $crate::stringify_anyhow(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        $crate::stringify_anyhow(anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::stringify_anyhow(anyhow::anyhow!($fmt, $($arg)*))
    };
}

/// Return early a formatted error string.
///
/// This is equivalent to `return Err(cmderr!($args...))`.
#[macro_export]
macro_rules! cmdbail {
    ($msg:literal $(,)?) => {
        return Err($crate::cmderr!($msg))
    };
    ($err:expr $(,)?) => {{
        return Err($crate::cmderr!($err))
    }};
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::cmderr!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_stringify_anyhow() {
        // Test that stringification of an anyhow error works correctly
        let reason1 = anyhow::anyhow!("reason 1");
        let reason2 = anyhow::anyhow!("reason 2");

        let error = anyhow::anyhow!("reason 3")
            .context(reason2)
            .context(reason1);
        let expected = "reason 1\n\nCaused by:\n  1: reason 2\n  2: reason 3";
        assert_eq!(stringify_anyhow(error), expected);
    }
}
