// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! parsec-tool: a tool for interfacing with the Parsec service from the command-line.

use parsec_client::BasicClient;
use parsec_tool::cli;
use parsec_tool::common::PROJECT_NAME;
use parsec_tool::err;
use std::convert::TryInto;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = cli::ParsecToolApp::from_args();

    let mut client = BasicClient::new(Some(PROJECT_NAME.to_string())).map_err(|e| {
        err!("{:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to spin up basic client.")
    })?;

    if let Some(provider) = matches.provider {
        let provider = provider.try_into().map_err(|e| {
            err!("{:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to find provider with ID entered.",
            )
        })?;
        client.set_implicit_provider(provider);
    }

    if let Some(timeout) = matches.timeout {
        let timeout = if timeout == 0 {
            None
        } else {
            Some(std::time::Duration::from_secs(timeout.into()))
        };
        client.set_timeout(timeout);
    }

    matches.subcommand.run(client).map_err(|e| {
        err!("{:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Executing subcommand failed.")
    })
}
