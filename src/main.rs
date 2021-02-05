// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! parsec-tool: a tool for interfacing with the Parsec service from the command-line.

use parsec_client::BasicClient;
use parsec_tool::cli;
use parsec_tool::err;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = cli::ParsecToolApp::from_args();
    let client = BasicClient::new(matches.app_name.clone()).map_err(|e| {
        err!("{:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to spin up basic client.")
    })?;
    matches.subcommand.run(&matches, client).map_err(|e| {
        err!("{:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Executing subcommand failed.")
    })
}
