// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Signs data.
//!
//! Will use the algorithm set to the key's policy during creation.

use crate::error::Result;
use crate::util::sign_message_with_policy;
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
        let signature = sign_message_with_policy(
            &basic_client,
            &self.key_name,
            self.input_data.as_bytes(),
            None,
        )?;

        let signature = base64::encode(signature);

        println!("{}", signature);

        Ok(())
    }
}
