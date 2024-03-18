// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Base CLI implementation.

use crate::common::{PROJECT_AUTHOR, PROJECT_DESC, PROJECT_NAME, PROJECT_VERSION};
use crate::subcommands::Subcommand;
use clap::StructOpt;

/// Struct representing the command-line interface of parsec-tool.
#[derive(Debug, StructOpt)]
#[structopt(name=PROJECT_NAME, about=PROJECT_DESC, author=PROJECT_AUTHOR, version=PROJECT_VERSION)]
pub struct ParsecToolApp {
    /// The ID of the provider to target for the command. Will use the default provider if not specified.
    #[structopt(short = 'p', long = "provider")]
    pub provider: Option<u8>,

    /// The timeout time used for all commands in seconds. Will use the client's default if not specified. If
    /// set to 0, will not use any timeout and will block indefinitely.
    #[structopt(short = 't', long = "timeout")]
    pub timeout: Option<u32>,

    /// The subcommand -- e.g., ping.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}
