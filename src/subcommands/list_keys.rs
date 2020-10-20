// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists all keys belonging to the application.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::{list_keys, NativeOperation, NativeResult};
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Lists all keys belonging to the application.
#[derive(Debug, StructOpt)]
#[structopt(name = "list_keys")]
pub struct ListKeysSubcommand {}

impl TryFrom<&ListKeysSubcommand> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(_list_keys_subcommand: &ListKeysSubcommand) -> Result<Self, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::ListKeys(list_keys::Operation {}))
    }
}

impl ParsecToolSubcommand<'_> for ListKeysSubcommand {
    /// Lists the available providers supported by the Parsec service.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            ProviderID::Core,
            &matches.authentication_data()?,
        )?;

        if let NativeResult::ListKeys(result) = native_result {
            if result.keys.is_empty() {
                info!("No keys currently available.");
                return Ok(());
            }
            info!("Available keys:");
            for key in result.keys {
                eprint_colored!(Blue, "*");
                eprint_colored!(Yellow, " '{}'", key.name);
                eprintln!(
                    " ({}, {} bit {:?})",
                    key.provider_id, key.attributes.bits, key.attributes.key_type
                );
            }
            Ok(())
        } else {
            Err(ParsecToolError::UnexpectedNativeResult(native_result))
        }
    }
}
