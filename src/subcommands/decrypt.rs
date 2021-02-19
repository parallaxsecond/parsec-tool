// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Decrypts data.
//!
//! Will use the algorithm set to the key's policy during creation.

use crate::error::{Result, ToolErrorKind};
use log::{error, info};
use parsec_client::core::interface::operations::psa_algorithm::Algorithm;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Decrypts data.
#[derive(Debug, StructOpt)]
pub struct Decrypt {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    /// Ciphertext base64 encoded
    input_data: String,
}

impl Decrypt {
    /// Decrypts data.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let input = base64::decode(self.input_data.as_bytes().to_vec())?;

        let alg = basic_client
            .key_attributes(&self.key_name)?
            .policy
            .permitted_algorithms;

        let plaintext = match alg {
            Algorithm::AsymmetricEncryption(alg) => {
                info!("Decrypting data with {:?}...", alg);
                basic_client.psa_asymmetric_decrypt(self.key_name.clone(), alg, &input, None)?
            }
            Algorithm::Cipher(_) | Algorithm::Aead(_) => {
                error!(
                    "Key's algorithm is {:?} which is not currently supported for decryption.",
                    alg
                );
                return Err(ToolErrorKind::NotSupported.into());
            }
            other => {
                error!(
                    "Key's algorithm is {:?} which can not be used for decryption.",
                    other
                );
                return Err(ToolErrorKind::WrongKeyAlgorithm.into());
            }
        };

        let plaintext = String::from_utf8_lossy(&plaintext).to_string();

        println!("{}", plaintext);

        Ok(())
    }
}
