// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Exports a public key.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::{OutputFileOpts, ProviderOpts};
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::psa_export_public_key;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::operation_client::OperationClient;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

/// Exports a public key.
#[derive(Debug, StructOpt)]
pub struct ExportPublicKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    #[structopt(flatten)]
    provider_opts: ProviderOpts,

    #[structopt(flatten)]
    output_file_opts: OutputFileOpts,
}

impl TryFrom<&ExportPublicKey> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(
        psa_export_public_key_subcommand: &ExportPublicKey,
    ) -> Result<NativeOperation, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::PsaExportPublicKey(
            psa_export_public_key::Operation {
                key_name: psa_export_public_key_subcommand.key_name.clone(),
            },
        ))
    }
}

impl ParsecToolSubcommand<'_> for ExportPublicKey {
    /// Exports a public key.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        info!("Exporting public key...");

        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            self.provider_opts.provider()?,
            &matches.authentication_data()?,
        )?;

        let result = match native_result {
            NativeResult::PsaExportPublicKey(result) => result,
            _ => {
                return Err(ParsecToolError::UnexpectedNativeResult(native_result));
            }
        };

        if let Some(output_file_path) = &self.output_file_opts.output_file_path {
            success!("Exported the key to {:?}.", output_file_path);
            let mut file = File::create(output_file_path)?;
            file.write_all(&result.data)?;
        } else {
            success!("Key:");
            for byte in &*result.data {
                print!("{:02X} ", byte);
            }
            println!();
        }
        Ok(())
    }
}
