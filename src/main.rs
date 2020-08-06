// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! parsec-tool: a tool for interfacing with the Parsec service from the command-line.

use parsec_tool::err;

use anyhow::Context;
use anyhow::Result;
use parsec_tool::cli;
use structopt::StructOpt;

fn run() -> Result<()> {
    let matches = cli::ParsecToolApp::from_args();
    matches
        .subcommand
        .run(&matches)
        .context("Executing subcommand failed.")?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        err!("{:?}", err);
        std::process::exit(1);
    }
}
