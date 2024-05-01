// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Create a RSA key pair
//!
//! The key will be 2048 bits long. Used by default for asymmetric encryption with RSA PKCS#1 v1.5.

use crate::error::Result;
use clap::Parser;
use log::info;
use parsec_client::core::interface::operations::psa_algorithm::{
    AsymmetricEncryption, AsymmetricSignature, Hash, SignHash,
};
use parsec_client::core::interface::operations::psa_key_attributes::{
    Attributes, Lifetime, Policy, Type, UsageFlags,
};
use parsec_client::BasicClient;

/// Create a RSA key pair.
#[derive(Debug, Parser)]
pub struct CreateRsaKey {
    #[structopt(short = 'k', long = "key-name")]
    key_name: String,

    /// This command creates RSA encryption keys by default. Supply this flag to create a signing key instead.
    /// Signing keys, by default, will specify the SHA-256 hash algorithm and use PKCS#1 v1.5.
    /// This has priority over ("r", "for-signing-pss") option.
    #[structopt(short = 's', long = "for-signing")]
    is_for_signing: bool,

    /// Supply this flag to create a signing key with PSS scheme and SHA-256 hash algorithm.
    #[structopt(short = 'r', long = "for-signing-pss")]
    is_for_signing_pss: bool,

    /// Specifies the size (strength) of the key in bits. The default size for RSA keys is 2048 bits.
    #[structopt(short = 'b', long = "bits")]
    bits: Option<usize>,

    /// Specifies if the RSA key should be created with permitted RSA OAEP (SHA256) encryption algorithm
    /// instead of the default RSA PKCS#1 v1.5 one.
    #[structopt(short = 'o', long = "oaep")]
    oaep: bool,
}

impl CreateRsaKey {
    /// Exports a key.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let policy = if self.is_for_signing {
            // If both "-s" and "-r" flags are set, then "-s" takes precedence
            info!("Creating RSA signing key with PKCS1 v1.5 scheme...");
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
        } else if self.is_for_signing_pss {
            info!("Creating RSA signing key with PSS scheme...");
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
                permitted_algorithms: AsymmetricSignature::RsaPss {
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
                permitted_algorithms: if self.oaep {
                    AsymmetricEncryption::RsaOaep {
                        hash_alg: Hash::Sha256,
                    }
                    .into()
                } else {
                    AsymmetricEncryption::RsaPkcs1v15Crypt.into()
                },
            }
        };

        let attributes = Attributes {
            lifetime: Lifetime::Persistent,
            key_type: Type::RsaKeyPair,
            // No prior validation of 'bits' argument. We have to let the service (and back-end hardware)
            // decide what is valid. The PSA specification does not enforce any minimum/maximum/supported
            // sizes for RSA keys.
            bits: self.bits.unwrap_or(2048),
            policy,
        };

        basic_client.psa_generate_key(&self.key_name, attributes)?;

        info!("Key \"{}\" created.", self.key_name);
        Ok(())
    }
}
