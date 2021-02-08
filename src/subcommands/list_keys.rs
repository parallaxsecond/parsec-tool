// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists all keys belonging to the application.

use crate::error::Result;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Lists all keys belonging to the application.
#[derive(Debug, StructOpt)]
pub struct ListKeys {}

impl ListKeys {
    /// Lists the available providers supported by the Parsec service.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let keys = basic_client.list_keys()?;

        if keys.is_empty() {
            info!("No keys currently available.");
            return Ok(());
        }
        info!("Available keys:");
        for key in keys {
            eprint_colored!(Blue, "*");
            eprint_colored!(Yellow, " '{}'", key.name);
            eprintln!(
                " ({}, {} bit {:?})",
                key.provider_id, key.attributes.bits, key.attributes.key_type
            );
        }
        Ok(())
    }
}
