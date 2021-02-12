// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the available providers supported by the Parsec service.

use crate::error::Result;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Lists the available providers supported by the Parsec service.
#[derive(Debug, StructOpt)]
pub struct ListProviders {}

impl ListProviders {
    /// Lists the available providers supported by the Parsec service.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let providers = basic_client.list_providers()?;

        info!("Available providers:");
        for provider in providers {
            title!("0x{:02x} ({})", provider.id as u32, provider.id);
            field!("Description", "{}", provider.description);
            field!(
                "Version",
                "{}.{}.{}",
                provider.version_maj,
                provider.version_min,
                provider.version_rev
            );
            field!(
                "Vendor",
                "{}",
                if !provider.vendor.is_empty() {
                    provider.vendor
                } else {
                    "Unspecified".to_string()
                },
            );
            field!("UUID", "{}", provider.uuid);
            println!();
        }
        Ok(())
    }
}
