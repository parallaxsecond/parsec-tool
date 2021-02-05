// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists all clients currently having data in the service (admin operation).

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::{list_clients, NativeOperation, NativeResult};

use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Lists all clients currently having data in the service (admin operation).
#[derive(Debug, StructOpt)]
pub struct ListClients {}

impl TryFrom<&ListClients> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(_list_clients_subcommand: &ListClients) -> Result<Self, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::ListClients(list_clients::Operation {}))
    }
}

impl ParsecToolSubcommand<'_> for ListClients {
    fn run(
        &self,
        _matches: &ParsecToolApp,
        basic_client: BasicClient,
    ) -> Result<(), ParsecToolError> {
        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            ProviderID::Core,
            &basic_client.auth_data(),
        )?;

        if let NativeResult::ListClients(result) = native_result {
            if result.clients.is_empty() {
                info!("No clients in the service.");
                return Ok(());
            }
            info!("Parsec clients:");
            for client in result.clients {
                eprint_colored!(Blue, "*");
                eprint_colored!(Yellow, " '{}'", client);
                eprintln!("");
            }
            Ok(())
        } else {
            Err(ParsecToolError::UnexpectedNativeResult(native_result))
        }
    }
}
