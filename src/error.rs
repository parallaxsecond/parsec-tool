// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Error definitions/handling.

use parsec_interface::operations::NativeResult;
use thiserror::Error;

/// Errors in parsec-tool.
#[derive(Error, Debug)]
pub enum ParsecToolError {
    /// Error emanating from the parsec_client crate.
    #[error(transparent)]
    ParsecClientError(#[from] parsec_client::error::Error),

    /// Error emanating from the parsec_interface crate.
    #[error(transparent)]
    ParsecInterfaceError(#[from] parsec_interface::requests::ResponseStatus),

    /// Unexpected native result error, for when we expected a particular type of result but get
    /// something else.
    #[error("Got an unexpected native result: {0:?}")]
    UnexpectedNativeResult(NativeResult),
}
