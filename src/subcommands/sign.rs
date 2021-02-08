// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Signs data.
//!
//! Will use the algorithm set to the key's policy during creation.

use crate::error::{Error, Result};
use crate::subcommands::common::key_attributes;
use parsec_client::core::interface::operations::psa_algorithm::Algorithm;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Signs data.
#[derive(Debug, StructOpt)]
pub struct Sign {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    /// String of UTF-8 text
    input_data: String,
}

impl Sign {
    /// Signs data.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let alg = key_attributes(&basic_client, &self.key_name)?
            .policy
            .permitted_algorithms;

        //TODO: depending on the hash in the algorithm, we need to hash input data

        let signature = match alg {
            Algorithm::AsymmetricSignature(alg) => {
                info!("Signing data...");
                Ok(basic_client.psa_sign_hash(
                    self.key_name.clone(),
                    self.input_data.as_bytes(),
                    alg,
                )?)
            }
            other => {
                err!(
                    "Key's algorithm is {:?} which can not be used for signing.",
                    other
                );
                Err(Error::WrongKeyAlgorithm)
            }
        }?;

        let signature = base64::encode(&signature);

        println!("{}", signature);

        Ok(())
    }
}
