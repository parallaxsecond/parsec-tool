// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Pings the Parsec service.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use parsec_client::core::interface::operations::ping;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use std::convert::TryFrom;
use structopt::StructOpt;

/// Pings the Parsec service.
#[derive(Debug, StructOpt)]
#[structopt(name = "ping")]
pub struct PingSubcommand {}

impl TryFrom<&PingSubcommand> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(_ping_subcommand: &PingSubcommand) -> Result<NativeOperation, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::Ping(ping::Operation {}))
    }
}

impl ParsecToolSubcommand<'_> for PingSubcommand {
    /// Pings the Parsec service and prints the wire protocol version.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        info!("Pinging Parsec service...");

        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            ProviderID::Core,
            &matches.authentication_data(),
        )?;

        if let NativeResult::Ping(result) = native_result {
            success!(
                "Service wire protocol version is {}.{}.",
                result.wire_protocol_version_maj,
                result.wire_protocol_version_min,
            );
            Ok(())
        } else {
            Err(ParsecToolError::UnexpectedNativeResult(native_result))
        }
    }
}
