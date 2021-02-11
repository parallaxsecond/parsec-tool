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
    /// The ID of the provider to target for the command. Will use the default provider if not specified.
    #[structopt(short = "p", long = "provider")]
    pub provider: Option<u8>,

    /// Sets the socket URI for communicating with the service.
    ///
    /// The value should be of the form "scheme://path". Currently the only supported
    /// scheme is `unix`, signifying a Unix Domain Socket found at the given `path`
    /// in the file system (e.g. `/some/path/parsec.sock`)
    ///
    /// If no value is provided, the following options are tried, in order:
    ///
    /// a) a socket URI stored in the `PARSEC_SERVICE_ENDPOINT` environment variable
    ///
    /// b) `unix:///run/parsec/parsec.sock` - the default socket location for the Parsec
    /// Unix Domain Socket
    ///
    /// c) `unix:///tmp/parsec.sock` - a usual location used for test deployments
    #[structopt(long, long = "socket_uri")]
    pub service_socket_uri: Option<String>,

    /// The subcommand -- e.g., ping.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}
