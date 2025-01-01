//! Re-export of `pretty_assertions` with extra assertion utilities.

use anyhow::Error;
pub use pretty_assertions::*;
use regex::Regex;

pub enum ChainReason {
    /// The error reason should be exactly equal to the given string.
    Exact(String),
    /// The error reason should match the given regex.
    Regex(String),
    /// The error reason should be an IO error.
    IOError,
    /// The error reason should be a `serde_json` error.
    SerdeError,
    /// Skip validating the reason.
    Skip,
}

/// Assert that an [`Error`] object has the expected chain of reasons.
pub fn assert_err_eq(error: Error, chain: Vec<ChainReason>) {
    let mut error_chain = error.chain();
    for expected_reason in chain {
        let reason = error_chain.next();
        assert!(reason.is_some(), "Expected more reasons in the error chain");
        let reason = reason.unwrap();

        match expected_reason {
            ChainReason::Exact(msg) => {
                self::assert_eq!(
                    reason.to_string(),
                    msg,
                    "Expected reason to be: {msg:?}; got: {reason:?}"
                );
            },
            ChainReason::Regex(pattern) => {
                let re = Regex::new(&pattern).unwrap();
                assert!(
                    re.is_match(&reason.to_string()),
                    "Expected reason to match pattern: {pattern:?}; got: {reason:?}"
                );
            },
            ChainReason::IOError => {
                let io_error = reason.downcast_ref::<std::io::Error>();
                assert!(
                    io_error.is_some(),
                    "Expected an IO error in the error chain; got: {reason:?}",
                );
            },
            ChainReason::SerdeError => {
                let serde_error = reason.downcast_ref::<serde_json::Error>();
                assert!(
                    serde_error.is_some(),
                    "Expected a serde_json error in the error chain; got: {reason:?}",
                );
            },
            ChainReason::Skip => continue,
        }
    }
    // Assert that the chain of reasons ends here
    assert!(
        error_chain.next().is_none(),
        "Expected no more reason in the error chain"
    );
}
