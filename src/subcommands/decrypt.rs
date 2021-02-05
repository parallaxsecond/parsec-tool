// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Generates a key.
//!
//! Currently a lot of the parameters of the key generation are hardcoded because it is not clear
//! on how it will be possible in the future to generate a key from the command line. This is
//! currently useful for playing with the tool, and demonstrating the use of Parsec.
//!
//! This will generate a 2048 bits RSA key pair for signing.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::ProviderOpts;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::psa_algorithm::{AsymmetricSignature, Hash};
use parsec_client::core::interface::operations::psa_generate_key;
use parsec_client::core::interface::operations::psa_key_attributes::{
    Attributes, Lifetime, Policy, Type, UsageFlags,
};
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::operation_client::OperationClient;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Generates a key.
#[derive(Debug, StructOpt)]
pub struct Decrypt {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    #[structopt(flatten)]
    provider_opts: ProviderOpts,
}

impl TryFrom<&Decrypt> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(psa_generate_key_subcommand: &Decrypt) -> Result<NativeOperation, Self::Error> {
        //TODO: All of the parameters are currently hardcoded to make it easier to use on the
        //command line for testing/demos. In the future, we want to have more options and keep a
        //relative simplicity.
        Ok(NativeOperation::PsaGenerateKey(
            psa_generate_key::Operation {
                key_name: psa_generate_key_subcommand.key_name.clone(),
                attributes: Attributes {
                    lifetime: Lifetime::Persistent,
                    key_type: Type::RsaKeyPair,
                    bits: 2048,
                    policy: Policy {
                        usage_flags: UsageFlags {
                            sign_hash: true,
                            ..Default::default()
                        },
                        permitted_algorithms: AsymmetricSignature::RsaPkcs1v15Sign {
                            hash_alg: Hash::Sha256.into(),
                        }
                        .into(),
                    },
                },
            },
        ))
    }
}

impl ParsecToolSubcommand<'_> for Decrypt {
    /// Exports a key.
    fn run(
        &self,
        _matches: &ParsecToolApp,
        basic_client: BasicClient,
    ) -> Result<(), ParsecToolError> {
        info!("Generating key...");

        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            self.provider_opts.provider()?,
            &basic_client.auth_data(),
        )?;

        match native_result {
            NativeResult::PsaGenerateKey(_) => (),
            _ => {
                return Err(ParsecToolError::UnexpectedNativeResult(native_result));
            }
        };

        success!("Key \"{}\" generated.", self.key_name);
        Ok(())
    }
}
