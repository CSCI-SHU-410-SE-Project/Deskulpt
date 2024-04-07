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
    /// The error reason should be an IO error with the given kind.
    IOErrorKind(std::io::ErrorKind),
}

/// Assert that an [`Error`] object has the expected chain of reasons.
pub(crate) fn assert_err_eq(error: Error, chain: Vec<ChainReason>) {
    let mut error_chain = error.chain();
    for expected_reason in chain {
        let reason = error_chain.next();
        assert!(reason.is_some());
        let reason = reason.unwrap();

        match expected_reason {
            ChainReason::Exact(msg) => assert_eq!(reason.to_string(), msg),
            ChainReason::_Match(expr) => assert!(expr.is_match(&reason.to_string())),
            ChainReason::_Skip => continue,
            ChainReason::IOErrorKind(kind) => {
                let io_error = reason.downcast_ref::<std::io::Error>();
                assert!(io_error.is_some());
                assert_eq!(io_error.unwrap().kind(), kind);
            },
        }
    }
    // Assert that the chain of reasons ends here
    assert!(error_chain.next().is_none());
}
