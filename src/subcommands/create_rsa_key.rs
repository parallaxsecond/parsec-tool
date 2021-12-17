// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Create a RSA key pair
//!
//! The key will be 2048 bits long. Used by default for asymmetric encryption with RSA PKCS#1 v1.5.

use crate::error::Result;
use log::info;
use parsec_client::core::interface::operations::psa_algorithm::{
    AsymmetricEncryption, AsymmetricSignature, Hash, SignHash,
};
use parsec_client::core::interface::operations::psa_key_attributes::{
    Attributes, Lifetime, Policy, Type, UsageFlags,
};
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Create a RSA key pair.
#[derive(Debug, StructOpt)]
pub struct CreateRsaKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    /// This command creates RSA encryption keys by default. Supply this flag to create a signing key instead.
    /// Signing keys, by default, will specify the SHA-256 hash algorithm and use PKCS#1 v1.5.
    #[structopt(short = "s", long = "for-signing")]
    is_for_signing: bool,
}

impl CreateRsaKey {
    /// Exports a key.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let policy = if self.is_for_signing {
            info!("Creating RSA signing key...");
            Policy {
                usage_flags: {
                    let mut usage_flags = UsageFlags::default();
                    let _ = usage_flags
                        .set_sign_hash()
                        .set_verify_hash()
                        .set_sign_message()
                        .set_verify_message();
                    usage_flags
                },
                permitted_algorithms: AsymmetricSignature::RsaPkcs1v15Sign {
                    hash_alg: SignHash::Specific(Hash::Sha256),
                }
                .into(),
            }
        } else {
            info!("Creating RSA encryption key...");
            Policy {
                usage_flags: {
                    let mut usage_flags = UsageFlags::default();
                    let _ = usage_flags.set_encrypt().set_decrypt();
                    usage_flags
                },
                permitted_algorithms: AsymmetricEncryption::RsaPkcs1v15Crypt.into(),
            }
        };

        let attributes = Attributes {
            lifetime: Lifetime::Persistent,
            key_type: Type::RsaKeyPair,
            bits: 2048,
            policy,
        };

        basic_client.psa_generate_key(&self.key_name, attributes)?;

        info!("Key \"{}\" created.", self.key_name);
        Ok(())
    }
}
