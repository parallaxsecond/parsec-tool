// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the supported opcodes for a given provider.

use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::ProviderOpts;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::list_opcodes;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Lists the supported opcodes for a given provider.
#[derive(Debug, StructOpt)]
#[structopt(name = "list_opcodes")]
pub struct ListOpcodesSubcommand {
    #[structopt(flatten)]
    provider_opts: ProviderOpts,
}

impl TryFrom<&ListOpcodesSubcommand> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(list_opcodes_subcommand: &ListOpcodesSubcommand) -> Result<Self, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::ListOpcodes(list_opcodes::Operation {
            provider_id: ProviderID::try_from(list_opcodes_subcommand.provider_opts.provider)?,
        }))
    }
}

impl ParsecToolSubcommand<'_> for ListOpcodesSubcommand {
    /// Lists the supported opcodes for a given provider.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            // We still use the core provider beacuse listing opcodes is a core operation. Note the
            // distinction between the provider we're _using_ and the provider we're querying.
            ProviderID::Core,
            &matches.authentication_data()?,
        )?;

        if let NativeResult::ListOpcodes(result) = native_result {
            info!(
                "Available opcodes for provider {:?}:",
                self.provider_opts.provider()?
            );
            for provider_opcode in result.opcodes {
                eprint_colored!(Blue, "*");
                eprintln!(" {:?}", provider_opcode);
            }
            Ok(())
        } else {
            Err(ParsecToolError::UnexpectedNativeResult(native_result))
        }
    }
}
