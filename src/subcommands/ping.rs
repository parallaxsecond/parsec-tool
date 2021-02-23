// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Pings the Parsec service.

use crate::error::Result;
use log::info;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Pings the Parsec service.
#[derive(Debug, StructOpt)]
pub struct Ping {}

impl Ping {
    /// Pings the Parsec service and prints the wire protocol version.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let result = basic_client.ping()?;

        info!("Service wire protocol version",);
        println!("{}.{}", result.0, result.1);
        Ok(())
    }
}
