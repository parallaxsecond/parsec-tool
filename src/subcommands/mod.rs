// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Subcommand implementations. Interacts with parsec-client-rust.

pub mod common;
pub mod list_opcodes;
pub mod list_providers;
pub mod ping;

use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::{
    list_opcodes::ListOpcodesSubcommand, list_providers::ListProvidersSubcommand,
    ping::PingSubcommand,
};
use anyhow::Result;
use parsec_interface::operations::NativeOperation;
use std::convert::TryInto;
use structopt::StructOpt;

/// A trait to represent a `parsec-tool` subcommand. Subcommands have three important properties:
/// - They have their own command-line interface, hence the dependency on `StructOpt`.
/// - They are convertible to a `NativeOperation` -- i.e. they can all be converted to messages to
///   the Parsec service. The conversion is fallible.
/// - They implement `run`, which executes the subcommand.
pub trait ParsecToolSubcommand: StructOpt + TryInto<NativeOperation> {
    /// Run the subcommand.
    fn run(&self, matches: &ParsecToolApp) -> Result<(), ParsecToolError>;
}

/// Command-line interface to Parsec operations.
#[derive(Copy, Clone, Debug, StructOpt)]
pub enum Subcommand {
    /// Pings the Parsec service and prints the wire protocol version.
    Ping(PingSubcommand),

    /// Lists the available providers supported by the Parsec service.
    ListProviders(ListProvidersSubcommand),

    /// Lists the supported opcodes for a given provider.
    ListOpcodes(ListOpcodesSubcommand),
}
