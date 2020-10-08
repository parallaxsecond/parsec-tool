// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Generates a sequence of random bytes.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::{OutputFileOpts, ProviderOpts};
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use parsec_client::core::interface::operations::psa_generate_random;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use std::convert::TryFrom;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

/// Generates a sequence of random bytes.
#[derive(Debug, StructOpt)]
#[structopt(name = "psa_generate_random")]
pub struct PsaGenerateRandomSubcommand {
    #[structopt(short = "n", long = "nbytes")]
    nbytes: usize,

    #[structopt(flatten)]
    provider_opts: ProviderOpts,

    #[structopt(flatten)]
    output_file_opts: OutputFileOpts,
}

impl TryFrom<&PsaGenerateRandomSubcommand> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(
        psa_generate_random_subcommand: &PsaGenerateRandomSubcommand,
    ) -> Result<NativeOperation, Self::Error> {
        Ok(NativeOperation::PsaGenerateRandom(
            psa_generate_random::Operation {
                size: psa_generate_random_subcommand.nbytes,
            },
        ))
    }
}

impl ParsecToolSubcommand<'_> for PsaGenerateRandomSubcommand {
    /// Generates a sequence of random bytes.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        info!("Generating {} random bytes...", self.nbytes);

        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            ProviderID::try_from(self.provider_opts.provider)?,
            &matches.authentication_data(),
        )?;

        let result = match native_result {
            NativeResult::PsaGenerateRandom(result) => result,
            _ => {
                return Err(ParsecToolError::UnexpectedNativeResult(native_result));
            }
        };

        if let Some(output_file_path) = &self.output_file_opts.output_file_path {
            let mut file = File::create(output_file_path)?;
            file.write_all(&result.random_bytes)?;
            success!("Written random bytes to file {:?}.", output_file_path);
        } else {
            success!("Random bytes:");
            for byte in &*result.random_bytes {
                print!("{:02X} ", byte);
            }
            println!();
        }
        Ok(())
    }
}
