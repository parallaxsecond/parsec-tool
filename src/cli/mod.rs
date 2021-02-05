// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Base CLI implementation.

use crate::common::{PROJECT_AUTHOR, PROJECT_DESC, PROJECT_NAME, PROJECT_VERSION};
use crate::subcommands::Subcommand;
use structopt::StructOpt;

/// Struct representing the command-line interface of parsec-tool.
#[derive(Debug, StructOpt)]
#[structopt(name=PROJECT_NAME, about=PROJECT_DESC, author=PROJECT_AUTHOR, version=PROJECT_VERSION)]
pub struct ParsecToolApp {
    /// How verbose should we be?
    #[structopt(short = "v", multiple = true)]
    pub verbosity: Option<u8>,

    /// Sets the application name -- will default to "parsec-tool" if unspecified.
    /// The app name is used when the service uses direct authentication.
    #[structopt(short = "a", long = "app_name")]
    pub app_name: Option<String>,

    /// The subcommand -- e.g., ping.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}
