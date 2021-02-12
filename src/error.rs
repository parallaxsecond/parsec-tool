// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Error definitions/handling.

use thiserror::Error;

/// Errors in parsec-tool.
#[derive(Error, Debug)]
pub enum Error {
    /// Error emanating from the parsec_client crate.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Error emanating from the parsec_client crate.
    #[error(transparent)]
    ParsecClientError(#[from] parsec_client::error::Error),

    /// Error emanating from the parsec_client::core::interface crate.
    #[error(transparent)]
    ParsecInterfaceError(#[from] parsec_client::core::interface::requests::ResponseStatus),

    /// Error emanating from the parsec-tool.
    #[error(transparent)]
    ParsecToolError(#[from] ToolErrorKind),

    /// Error emanating from the base64 crate.
    #[error(transparent)]
    Base64Decode(#[from] base64::DecodeError),
}

/// Errors originating in the parsec-tool.
#[derive(Error, Debug)]
pub enum ToolErrorKind {
    /// Operation not supported by the parsec-tool
    #[error("Operation not supported by the parsec-tool")]
    NotSupported,

    /// They key was not created with the correct algorithm for this operation
    #[error("They key was not created with the correct algorithm for this operation")]
    WrongKeyAlgorithm,

    /// Expected input data was not given
    #[error("A command expected input data that was not given")]
    NoInput,
}

/// A Result type with the Err variant set as a ParsecToolError
pub type Result<T> = std::result::Result<T, Error>;
