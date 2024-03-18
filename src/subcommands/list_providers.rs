// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the available providers supported by the Parsec service.

use crate::error::Result;
use clap::StructOpt;
use log::info;
use parsec_client::BasicClient;

/// Lists the available providers supported by the Parsec service.
#[derive(Debug, StructOpt)]
pub struct ListProviders {}

impl ListProviders {
    /// Lists the available providers supported by the Parsec service.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let providers = basic_client.list_providers()?;

        info!("Available providers:");
        for provider in providers {
            println!("ID: 0x{:02x} ({})", provider.id as u32, provider.id);
            println!("Description: {}", provider.description);
            println!(
                "Version: {}.{}.{}",
                provider.version_maj, provider.version_min, provider.version_rev
            );
            println!(
                "Vendor: {}",
                if !provider.vendor.is_empty() {
                    provider.vendor
                } else {
                    "Unspecified".to_string()
                },
            );
            println!("UUID: {}", provider.uuid);
            println!();
        }
        Ok(())
    }
}
