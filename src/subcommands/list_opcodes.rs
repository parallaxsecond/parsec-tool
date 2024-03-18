// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the supported opcodes for a given provider.
use crate::error::Result;
use clap::StructOpt;
use log::info;
use parsec_client::BasicClient;
use std::convert::TryInto;

/// Lists the supported opcodes for a given provider.
#[derive(Debug, StructOpt)]
pub struct ListOpcodes {
    /// ID of the provider.
    #[structopt(short = 'p', long = "provider")]
    pub provider: Option<u8>,
}

impl ListOpcodes {
    /// Lists the supported opcodes for a given provider.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let provider = match self.provider {
            Some(provider) => provider.try_into()?,
            None => basic_client.implicit_provider(),
        };
        let opcodes = basic_client.list_opcodes(provider)?;

        info!("Available opcodes for {}:", provider);
        for provider_opcode in opcodes {
            println!("0x{:02x} ({:?})", provider_opcode as u32, provider_opcode);
        }
        Ok(())
    }
}
