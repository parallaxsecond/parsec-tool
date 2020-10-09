// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Error definitions/handling.

use parsec_client::core::interface::operations::NativeResult;
use thiserror::Error;

/// Errors in parsec-tool.
#[derive(Error, Debug)]
pub enum ParsecToolError {
    /// Error emanating from the parsec_client crate.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Error emanating from the parsec_client crate.
    #[error(transparent)]
    ParsecClientError(#[from] parsec_client::error::Error),

    /// Error emanating from the parsec_client::core::interface crate.
    #[error(transparent)]
    ParsecInterfaceError(#[from] parsec_client::core::interface::requests::ResponseStatus),

    /// Unexpected native result error, for when we expected a particular type of result but get
    /// something else.
    #[error("Got an unexpected native result: {0:?}")]
    UnexpectedNativeResult(NativeResult),
}
