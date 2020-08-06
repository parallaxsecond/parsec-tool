// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Base CLI implementation.

use crate::common::{PROJECT_AUTHOR, PROJECT_DESC, PROJECT_NAME, PROJECT_VERSION};
use crate::subcommands::Subcommand;
use parsec_client::auth::AuthenticationData;
use structopt::StructOpt;

/// Struct representing the command-line interface of parsec-tool.
#[derive(Debug, StructOpt)]
#[structopt(name=PROJECT_NAME, about=PROJECT_DESC, author=PROJECT_AUTHOR, version=PROJECT_VERSION)]
pub struct ParsecToolApp {
    /// How verbose should we be?
    #[structopt(short = "v", multiple = true, default_value = "0")]
    pub verbosity: u8,

    /// Sets the authentication secret -- will default to no authentication if unspecified.
    #[structopt(short = "a", long = "auth-secret")]
    pub auth_secret: Option<String>,

    /// The subcommand -- e.g., ping.
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

impl ParsecToolApp {
    /// Given an optional string, generate the corresponding AuthenticationData instance. This is
    /// effectively a `FromStr` implementation for AuthenticationData. Passing in `None` will
    /// return AuthenticationData::None. Passing in `Some(s)` will give you an app identity whose
    /// secret is built from the string `s`.
    pub fn authentication_data(&self) -> AuthenticationData {
        match &self.auth_secret {
            None => AuthenticationData::None,
            Some(s) => AuthenticationData::AppIdentity(secrecy::Secret::new(s.into())),
        }
    }
}
