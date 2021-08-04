// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Exports a public key.

use crate::error::Result;
use parsec_client::core::interface::operations::psa_key_attributes::Type;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Exports a PEM-encoded public key.
#[derive(Debug, StructOpt)]
pub struct ExportPublicKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,
}

impl ExportPublicKey {
    /// Exports a public key.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let result = basic_client.psa_export_public_key(&self.key_name)?;

        let tag = match basic_client.key_attributes(&self.key_name)?.key_type {
            Type::RsaKeyPair | Type::RsaPublicKey => String::from("RSA PUBLIC KEY"),
            _ => String::from("PUBLIC KEY"),
        };

        let pem_encoded = pem::encode_config(
            &pem::Pem {
                tag,
                contents: result,
            },
            pem::EncodeConfig {
                line_ending: pem::LineEnding::LF,
            },
        );

        print!("{}", pem_encoded);
        Ok(())
    }
}
