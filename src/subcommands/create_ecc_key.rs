// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Create an ECC key pair.
//!
/// The curve will be secp256r1. Used by default for asymmetric signing with ECDSA (SHA-256).
pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::ProviderOpts;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::psa_algorithm::{AsymmetricSignature, Hash};
use parsec_client::core::interface::operations::psa_generate_key;
use parsec_client::core::interface::operations::psa_key_attributes::{
    Attributes, EccFamily, Lifetime, Policy, Type, UsageFlags,
};
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::operation_client::OperationClient;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Create an ECC key pair.
#[derive(Debug, StructOpt)]
pub struct CreateEccKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    #[structopt(flatten)]
    provider_opts: ProviderOpts,
}

impl TryFrom<&CreateEccKey> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(
        psa_generate_key_subcommand: &CreateEccKey,
    ) -> Result<NativeOperation, Self::Error> {
        Ok(NativeOperation::PsaGenerateKey(
            psa_generate_key::Operation {
                key_name: psa_generate_key_subcommand.key_name.clone(),
                attributes: Attributes {
                    lifetime: Lifetime::Persistent,
                    key_type: Type::EccKeyPair {
                        curve_family: EccFamily::SecpR1,
                    },
                    bits: 256,
                    policy: Policy {
                        usage_flags: UsageFlags {
                            sign_hash: true,
                            ..Default::default()
                        },
                        permitted_algorithms: AsymmetricSignature::Ecdsa {
                            hash_alg: Hash::Sha256.into(),
                        }
                        .into(),
                    },
                },
            },
        ))
    }
}

impl ParsecToolSubcommand<'_> for CreateEccKey {
    /// Exports a key.
    fn run(
        &self,
        _matches: &ParsecToolApp,
        basic_client: BasicClient,
    ) -> Result<(), ParsecToolError> {
        info!("Creating ECC key...");

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

        success!("Key \"{}\" created.", self.key_name);
        Ok(())
    }
}
