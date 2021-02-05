// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Delete a key.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::ProviderOpts;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::psa_destroy_key;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::operation_client::OperationClient;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Delete a key.
#[derive(Debug, StructOpt)]
pub struct DeleteKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    #[structopt(flatten)]
    provider_opts: ProviderOpts,
}

impl TryFrom<&DeleteKey> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(psa_destroy_key_subcommand: &DeleteKey) -> Result<NativeOperation, Self::Error> {
        Ok(NativeOperation::PsaDestroyKey(psa_destroy_key::Operation {
            key_name: psa_destroy_key_subcommand.key_name.clone(),
        }))
    }
}

impl ParsecToolSubcommand<'_> for DeleteKey {
    /// Destroys a key.
    fn run(
        &self,
        _matches: &ParsecToolApp,
        basic_client: BasicClient,
    ) -> Result<(), ParsecToolError> {
        info!("Deleting a key...");

        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            self.provider_opts.provider()?,
            &basic_client.auth_data(),
        )?;

        match native_result {
            NativeResult::PsaDestroyKey(_) => (),
            _ => {
                return Err(ParsecToolError::UnexpectedNativeResult(native_result));
            }
        };

        success!("Key \"{}\" deleted.", self.key_name);
        Ok(())
    }
}
