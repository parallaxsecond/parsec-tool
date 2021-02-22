// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the supported opcodes for a given provider.

use crate::error::Result;
use log::info;
use parsec_client::BasicClient;
use std::convert::TryInto;
use structopt::StructOpt;

/// Lists the supported opcodes for a given provider.
#[derive(Debug, StructOpt)]
pub struct ListOpcodes {
    /// ID of the provider.
    #[structopt(short = "p", long = "provider")]
    pub provider: u8,
}

impl ListOpcodes {
    /// Lists the supported opcodes for a given provider.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        let provider = self.provider.try_into()?;
        let opcodes = basic_client.list_opcodes(provider)?;

        info!("Available opcodes for {}:", provider);
        for provider_opcode in opcodes {
            println!("0x{:02x} ({:?})", provider_opcode as u32, provider_opcode);
        }
        Ok(())
    }
}
