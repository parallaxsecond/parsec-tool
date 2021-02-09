// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Exports a public key.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::{OutputFileOpts, ProviderOpts};
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::psa_export_public_key;
use parsec_client::core::interface::operations::psa_key_attributes::Type;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::operation_client::OperationClient;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

/// Exports a PEM-encoded public key.
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
    fn run(
        &self,
        _matches: &ParsecToolApp,
        basic_client: BasicClient,
    ) -> Result<(), ParsecToolError> {
        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            self.provider_opts.provider()?,
            &basic_client.auth_data(),
        )?;

        let result = match native_result {
            NativeResult::PsaExportPublicKey(result) => result,
            _ => {
                return Err(ParsecToolError::UnexpectedNativeResult(native_result));
            }
        };

        let key_list = basic_client.list_keys()?;
        let mut tag = String::from("PUBLIC KEY");
        for key in key_list {
            if key.name != self.key_name {
                continue;
            }

            if key.attributes.key_type == Type::RsaKeyPair
                || key.attributes.key_type == Type::RsaPublicKey
            {
                tag = String::from("RSA PUBLIC KEY");
            }
        }

        let pem_encoded = pem::encode_config(
            &pem::Pem {
                tag,
                contents: result.data.to_vec(),
            },
            pem::EncodeConfig {
                line_ending: pem::LineEnding::LF,
            },
        );

        if let Some(output_file_path) = &self.output_file_opts.output_file_path {
            success!("Exported the key to {:?}.", output_file_path);
            let mut file = File::create(output_file_path)?;
            file.write_all(&Vec::from(pem_encoded))?;
        } else {
            print!("{}", pem_encoded);
        }
        Ok(())
    }
}
