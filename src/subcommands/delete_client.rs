// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Delete all data a client has in the service (admin operation).

use crate::error::Result;

use clap::Parser;
use log::info;
use parsec_client::BasicClient;

/// Delete all data a client has in the service (admin operation).
#[derive(Debug, Parser)]
pub struct DeleteClient {
    #[structopt(short = 'c', long = "client")]
    client: String,
}

impl DeleteClient {
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        basic_client.delete_client(&self.client)?;

        info!("Client \"{}\" deleted.", self.client);
        Ok(())
    }
}
