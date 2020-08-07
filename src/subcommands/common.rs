// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Common facilities and options for subcommands.

use structopt::StructOpt;

/// Options for specifying a provider. Most, but not all subcommands require the user to do this,
/// so it's useful to have these options shared.
#[derive(Debug, StructOpt)]
pub struct ProviderOpts {
    /// The provider to list opcodes for.
    #[structopt(short = "p", long = "provider", default_value = "0")]
    pub provider: u8,
}
