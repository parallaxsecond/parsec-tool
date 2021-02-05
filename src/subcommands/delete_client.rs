// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Delete all data a client has in the service (admin operation).

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::{delete_client, NativeOperation, NativeResult};

use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Delete all data a client has in the service (admin operation).
#[derive(Debug, StructOpt)]
pub struct DeleteClient {
    #[structopt(short = "c", long = "client")]
    client: String,
}

impl TryFrom<&DeleteClient> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(delete_client: &DeleteClient) -> Result<Self, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::DeleteClient(delete_client::Operation {
            client: delete_client.client.clone(),
        }))
    }
}

impl ParsecToolSubcommand<'_> for DeleteClient {
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

        match native_result {
            NativeResult::DeleteClient(_) => (),
            _ => {
                return Err(ParsecToolError::UnexpectedNativeResult(native_result));
            }
        };

        success!("Client \"{}\" deleted.", self.client);
        Ok(())
    }
}
