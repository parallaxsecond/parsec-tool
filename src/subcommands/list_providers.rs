// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the available providers supported by the Parsec service.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::list_providers;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Lists the available providers supported by the Parsec service.
#[derive(Debug, StructOpt)]
#[structopt(name = "list_providers")]
pub struct ListProvidersSubcommand {}

impl TryFrom<&ListProvidersSubcommand> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(_list_providers_subcommand: &ListProvidersSubcommand) -> Result<Self, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::ListProviders(list_providers::Operation {}))
    }
}

impl ParsecToolSubcommand<'_> for ListProvidersSubcommand {
    /// Lists the available providers supported by the Parsec service.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            ProviderID::Core,
            &matches.authentication_data(),
        )?;

        if let NativeResult::ListProviders(result) = native_result {
            info!("Available providers:");
            for provider in result.providers {
                title!("0x{:02x} ({})", provider.id as u32, provider.id);
                field!("Description", "{}", provider.description);
                field!(
                    "Version",
                    "{}.{}.{}",
                    provider.version_maj,
                    provider.version_min,
                    provider.version_rev
                );
                field!(
                    "Vendor",
                    "{}",
                    if !provider.vendor.is_empty() {
                        provider.vendor
                    } else {
                        "Unspecified".to_string()
                    },
                );
                field!("UUID", "{}", provider.uuid);
                println!();
            }
            Ok(())
        } else {
            Err(ParsecToolError::UnexpectedNativeResult(native_result))
        }
    }
}
