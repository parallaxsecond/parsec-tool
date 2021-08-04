// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Create a RSA key pair
//!
//! The key will be 2048 bits long. Used by default for asymmetric encryption with RSA PKCS#1 v1.5.

use crate::error::Result;
use log::info;
use parsec_client::core::interface::operations::psa_algorithm::AsymmetricEncryption;
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
}

impl CreateRsaKey {
    /// Exports a key.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        info!("Creating RSA key...");

        let attributes = Attributes {
            lifetime: Lifetime::Persistent,
            key_type: Type::RsaKeyPair,
            bits: 2048,
            policy: Policy {
                usage_flags: {
                    let mut usage_flags = UsageFlags::default();
                    let _ = usage_flags.set_encrypt().set_decrypt();
                    usage_flags
                },
                permitted_algorithms: AsymmetricEncryption::RsaPkcs1v15Crypt.into(),
            },
        };

        basic_client.psa_generate_key(&self.key_name, attributes)?;

        info!("Key \"{}\" created.", self.key_name);
        Ok(())
    }
}
