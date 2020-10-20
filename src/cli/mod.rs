// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Base CLI implementation.

use crate::common::{PROJECT_AUTHOR, PROJECT_DESC, PROJECT_NAME, PROJECT_VERSION};
use crate::error::ParsecToolError;
use crate::subcommands::Subcommand;
use parsec_client::auth::Authentication;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Struct representing the command-line interface of parsec-tool.
#[derive(Debug, StructOpt)]
#[structopt(name=PROJECT_NAME, about=PROJECT_DESC, author=PROJECT_AUTHOR, version=PROJECT_VERSION)]
pub struct ParsecToolApp {
    /// How verbose should we be?
    #[structopt(short = "v", multiple = true, default_value = "0")]
    pub verbosity: u8,

    /// Sets the application name -- will default to "parsec-tool" if unspecified.
    /// The app name is used when the service uses direct authentication.
    #[structopt(short = "a", long = "app_name", default_value = "parsec-tool")]
    pub app_name: String,

    /// The subcommand -- e.g., ping.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

impl ParsecToolApp {
    /// Given an optional app name, generate the corresponding Authentication instance. This method
    /// makes use of the authentication bootstrapping mechanism in `BasicClient` to obtain the
    /// appropriate data for the tool.
    pub fn authentication_data(&self) -> Result<Authentication, ParsecToolError> {
        let mut client = BasicClient::new_naked();
        client.set_default_auth(Some(self.app_name.clone()))?;
        Ok(client.auth_data())
    }
}
