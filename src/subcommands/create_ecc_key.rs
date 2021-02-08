// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Create an ECC key pair.
//!
use crate::error::Result;
/// The curve will be secp256r1. Used by default for asymmetric signing with ECDSA (SHA-256).
use parsec_client::core::interface::operations::psa_algorithm::{AsymmetricSignature, Hash};
use parsec_client::core::interface::operations::psa_key_attributes::{
    Attributes, EccFamily, Lifetime, Policy, Type, UsageFlags,
};
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Create an ECC key pair.
#[derive(Debug, StructOpt)]
pub struct CreateEccKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,
}

impl CreateEccKey {
    /// Exports a key.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        info!("Creating ECC key...");

        let attributes = Attributes {
            lifetime: Lifetime::Persistent,
            key_type: Type::EccKeyPair {
                curve_family: EccFamily::SecpR1,
            },
            bits: 256,
            policy: Policy {
                usage_flags: UsageFlags {
                    sign_hash: true,
                    sign_message: true,
                    ..Default::default()
                },
                permitted_algorithms: AsymmetricSignature::Ecdsa {
                    hash_alg: Hash::Sha256.into(),
                }
                .into(),
            },
        };

        basic_client.psa_generate_key(self.key_name.clone(), attributes)?;

        success!("Key \"{}\" created.", self.key_name);
        Ok(())
    }
}
