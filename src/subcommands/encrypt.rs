// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Encrypts data using a public key or the public part of a key pair.
//!
//! Will use the algorithm set to the key's policy during creation. Currently only
//! supports asymmetric encryption such as RSA.
//!
//! No salt is used.
//!
//! The input is a plain text message string, which is treated as raw bytes.
//!
//! The output is base64-encoded ciphertext.

use crate::error::{Result, ToolErrorKind};
use log::{error, info};
use parsec_client::core::interface::operations::psa_algorithm::Algorithm;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Encrypts data.
#[derive(Debug, StructOpt)]
pub struct Encrypt {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    /// Plaintext input string.
    input_data: String,
}

impl Encrypt {
    /// Encrypts data.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let input = self.input_data.as_bytes();

        let alg = basic_client
            .key_attributes(&self.key_name)?
            .policy
            .permitted_algorithms;

        let ciphertext = match alg {
            Algorithm::AsymmetricEncryption(alg) => {
                info!("Encrypting data with {:?}...", alg);
                basic_client.psa_asymmetric_encrypt(&self.key_name, alg, input, None)?
            }
            Algorithm::Cipher(_) | Algorithm::Aead(_) => {
                error!(
                    "Key's algorithm is {:?} which is not currently supported for encryption.",
                    alg
                );
                return Err(ToolErrorKind::NotSupported.into());
            }
            other => {
                error!(
                    "Key's algorithm is {:?} which cannot be used for encryption.",
                    other
                );
                return Err(ToolErrorKind::WrongKeyAlgorithm.into());
            }
        };

        let ciphertext = base64::encode(&ciphertext);

        println!("{}", ciphertext);

        Ok(())
    }
}
