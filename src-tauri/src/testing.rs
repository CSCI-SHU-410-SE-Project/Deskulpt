//! This module is provides testing utilities. It should not be included except for in
//! test builds.

use anyhow::Error;
use pretty_assertions::assert_eq;
use regex::Regex;

pub(crate) enum ChainReason {
    /// The error reason should be exactly equal to the given string.
    Exact(String),
    /// The error reason should match the given regular expression.
    _Match(Regex),
    /// Skip this reason.
    _Skip,
    /// The error reason should be an IO error.
    ///
    /// It can wrap the expected kind of IO error, or specify `None` to skip the step of
    /// checking the error kind.
    IOErrorKind(Option<std::io::ErrorKind>),
    /// The error reason should be a `serde_json` error.
    ///
    /// It wraps the expected category of the error and a regular expression that the
    /// error message should match.
    SerdeErrorCategory(serde_json::error::Category, Regex),
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
            ChainReason::_Match(expr) => assert!(
                expr.is_match(&reason.to_string()),
                "Expected reason: {reason:?}",
            ),
            ChainReason::_Skip => continue,
            ChainReason::IOErrorKind(kind) => {
                let io_error = reason.downcast_ref::<std::io::Error>();
                assert!(io_error.is_some(), "Expected an IO error in the error chain");
                if let Some(kind) = kind {
                    assert_eq!(io_error.unwrap().kind(), kind);
                }
            },
            ChainReason::SerdeErrorCategory(cat, expr) => {
                let serde_error = reason.downcast_ref::<serde_json::Error>();
                assert!(
                    serde_error.is_some(),
                    "Expected a serde_json error in the error chain",
                );
                let serde_error = serde_error.unwrap();
                assert_eq!(serde_error.classify(), cat);
                assert!(
                    expr.is_match(&serde_error.to_string()),
                    "Expected reason: {reason:?}"
                );
            },
        }
    }
    // Assert that the chain of reasons ends here
    assert!(error_chain.next().is_none(), "Expected no more reason in the error chain");
}
