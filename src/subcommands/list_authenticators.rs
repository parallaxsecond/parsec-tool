// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! List the authenticators supported by the Parsec service.

use crate::error::Result;
use clap::StructOpt;
use log::info;
use parsec_client::BasicClient;

/// List the authenticators supported by the Parsec service.
#[derive(Debug, StructOpt)]
pub struct ListAuthenticators {}

impl ListAuthenticators {
    /// Lists the available authenticators supported by the Parsec service.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let authenticators = basic_client.list_authenticators()?;

        info!("Available authenticators:");
        for authenticator in authenticators {
            println!(
                "ID: 0x{:02x} ({})",
                authenticator.id as u32, authenticator.id
            );
            println!("Description: {}", authenticator.description);
            println!(
                "Version: {}.{}.{}",
                authenticator.version_maj, authenticator.version_min, authenticator.version_rev
            );
            println!();
        }
        Ok(())
    }
}
