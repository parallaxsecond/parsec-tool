// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! parsec-tool: a tool for interfacing with the Parsec service from the command-line.

use parsec_client::core::ipc_handler::unix_socket::Handler;
use parsec_client::BasicClient;
use parsec_tool::common::PROJECT_NAME;
use parsec_tool::{cli, err, error};
use std::convert::TryInto;
use std::env;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

const TIMEOUT: Option<Duration> = Some(Duration::from_secs(30));

fn main() -> Result<(), error::Error> {
    env_logger::init();

    let matches = cli::ParsecToolApp::from_args();
    let mut client = BasicClient::new_naked();
    client.set_ipc_handler(Box::new(find_socket(matches.service_socket_uri.as_ref())?));
    client
        .set_default_auth(Some(PROJECT_NAME.to_owned()))
        .map_err(|e| {
            err!("{:?}", e);
            error::Error::IoError(Error::new(
                ErrorKind::Other,
                "Failed to set up default authentication.",
            ))
        })?;
    client.set_default_provider().map_err(|e| {
        err!("{:?}", e);
        error::Error::IoError(Error::new(
            ErrorKind::Other,
            "Failed to set up default provider.",
        ))
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

    matches.subcommand.run(client).map_err(|e| {
        err!("{:?}", e);
        error::Error::IoError(Error::new(ErrorKind::Other, "Executing subcommand failed."))
    })
}

/// Try to find an appropriate socket location
fn find_socket(socker_uri_arg: Option<&String>) -> Result<Handler, error::Error> {
    let endpoint_env_var = env::var("PARSEC_SERVICE_ENDPOINT");

    if let Some(socket_uri) = socker_uri_arg {
        // URI given as a command line argument
        Ok(parse_socket_location(&socket_uri)?)
    } else if endpoint_env_var.is_ok() {
        // URI given as an environment variable
        Ok(parse_socket_location(&endpoint_env_var.unwrap())?)
    } else if PathBuf::from("/run/parsec/parsec.sock").exists() {
        // Default Unix socket location
        Ok(Handler::new(
            PathBuf::from("/run/parsec/parsec.sock"),
            TIMEOUT,
        ))
    } else if PathBuf::from("/tmp/parsec.sock").exists() {
        // Old socket location (still used for testing)
        Ok(Handler::new(PathBuf::from("/tmp/parsec.sock"), TIMEOUT))
    } else {
        Err(error::Error::IoError(Error::new(
            ErrorKind::NotFound,
            "Failed to find an appropriate communication path to the Parsec service",
        )))
    }
}

/// Parse the socket location from a URI and construct a Handler for it
fn parse_socket_location(socket_uri: &str) -> Result<Handler, error::Error> {
    if let Some(path) = socket_uri.strip_prefix("unix://") {
        let path = PathBuf::from(path);
        if path.exists() {
            Ok(Handler::new(path, TIMEOUT))
        } else {
            Err(error::Error::IoError(Error::new(
                ErrorKind::NotFound,
                format!("Path from socket URI does not exist: {}", socket_uri),
            )))
        }
    } else {
        Err(error::Error::IoError(Error::new(
            ErrorKind::NotFound,
            format!("Failed to parse the given socket URI: {}", socket_uri),
        )))
    }
}
