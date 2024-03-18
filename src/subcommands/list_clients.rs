// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists all clients currently having data in the service (admin operation).

use crate::error::Result;
use clap::StructOpt;
use log::info;
use parsec_client::BasicClient;

/// Lists all clients currently having data in the service (admin operation).
#[derive(Debug, StructOpt)]
pub struct ListClients {}

impl ListClients {
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let clients = basic_client.list_clients()?;

        if clients.is_empty() {
            info!("No clients in the service.");
            return Ok(());
        }
        info!("Parsec clients:");
        for client in clients {
            println!("{}", client);
        }
        Ok(())
    }
}
