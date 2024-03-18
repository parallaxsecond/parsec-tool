// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! parsec-tool: a tool for interfacing with the Parsec service from the command-line.

use clap::StructOpt;
use log::error;
use parsec_tool::cli;
use parsec_tool::common::PROJECT_NAME;
use std::convert::TryInto;

fn main() {
    let mut env_log_builder = env_logger::Builder::new();
    // By default, only show the logs from this crate.
    env_log_builder.filter_level(log::LevelFilter::Info);
    env_log_builder.format_timestamp(None);
    env_log_builder.format_module_path(false);

    // Allows to still set configuration via the default environment variable
    env_log_builder.parse_default_env();
    env_log_builder.init();

    let matches = cli::ParsecToolApp::from_args();

    let mut client = match matches
        .subcommand
        .create_client(Some(PROJECT_NAME.to_string()))
    {
        Err(e) => {
            error!("Error spinning up the BasicClient: {}", e);
            std::process::exit(1);
        }
        Ok(client) => client,
    };

    if let Some(provider) = matches.provider {
        let provider = match provider.try_into() {
            Err(_) => {
                error!("The provider ID entered does not map with an existing provider");
                std::process::exit(1);
            }
            Ok(provider) => provider,
        };
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

    if let Err(e) = matches.subcommand.run(client) {
        error!("Subcommand failed: {} ({:?})", e, e);
        std::process::exit(1);
    }

    std::process::exit(0);
}
