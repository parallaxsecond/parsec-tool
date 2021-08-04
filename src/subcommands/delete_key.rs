// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Delete a key.

use crate::error::Result;
use log::info;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Delete a key.
#[derive(Debug, StructOpt)]
pub struct DeleteKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,
}

impl DeleteKey {
    /// Destroys a key.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        info!("Deleting a key...");

        basic_client.psa_destroy_key(&self.key_name)?;

        info!("Key \"{}\" deleted.", self.key_name);
        Ok(())
    }
}
