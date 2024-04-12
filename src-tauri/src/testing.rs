//! This module is provides testing utilities. It should not be included except for in
//! test builds.

use anyhow::Error;
use pretty_assertions::assert_eq;

pub(crate) enum ChainReason {
    /// The error reason should be exactly equal to the given string.
    Exact(String),
    /// The error reason should be an IO error.
    IOError,
    /// The error reason should be a `serde_json` error.
    SerdeError,
    /// Skip validating the reason.
    _Skip,
}

/// Assert that an [`Error`] object has the expected chain of reasons.
pub(crate) fn assert_err_eq(error: Error, chain: Vec<ChainReason>) {
    let mut error_chain = error.chain();
    for expected_reason in chain {
        let reason = error_chain.next();
        assert!(reason.is_some(), "Expected more reasons in the error chain");
        let reason = reason.unwrap();

        match expected_reason {
            ChainReason::Exact(msg) => {
                assert_eq!(reason.to_string(), msg, "Expected reason: {reason:?}")
            },
            ChainReason::IOError => {
                let io_error = reason.downcast_ref::<std::io::Error>();
                assert!(io_error.is_some(), "Expected an IO error in the error chain");
            },
            ChainReason::SerdeError => {
                let serde_error = reason.downcast_ref::<serde_json::Error>();
                assert!(
                    serde_error.is_some(),
                    "Expected a serde_json error in the error chain",
                );
            },
            ChainReason::_Skip => continue,
        }
    }
    // Assert that the chain of reasons ends here
    assert!(error_chain.next().is_none(), "Expected no more reason in the error chain");
}
