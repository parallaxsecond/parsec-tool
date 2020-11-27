// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! parsec-tool: a tool for interfacing with the Parsec service from the command-line.

use parsec_tool::err;

use parsec_tool::cli;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = cli::ParsecToolApp::from_args();
    matches.subcommand.run(&matches).map_err(|e| {
        err!("{:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Executing subcommand failed.")
    })
}
