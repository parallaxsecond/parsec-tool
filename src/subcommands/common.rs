// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Common facilities and options for subcommands.

use crate::error::ParsecToolError;
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use std::path::PathBuf;
use structopt::StructOpt;

/// Options for specifying a provider. Most, but not all subcommands require the user to do this,
/// so it's useful to have these options shared.
#[derive(Debug, StructOpt)]
pub struct ProviderOpts {
    /// The provider to target for the operation.
    #[structopt(short = "p", long = "provider")]
    pub provider: Option<u8>,
}

impl ProviderOpts {
    /// Get the ProviderID selected by the user or the service default if
    /// no provider was selected.
    ///
    /// The Core Provider cannot be used and will be overriden by the
    /// service default.
    pub fn provider(&self) -> Result<ProviderID, ParsecToolError> {
        match self.provider {
            None => {
                let mut client = BasicClient::new_naked();
                client.set_default_provider()?;
                Ok(client.implicit_provider())
            }
            Some(id) => Ok(ProviderID::try_from(id)?),
        }
    }
}

/// Options for specifying an output file.
#[derive(Debug, StructOpt)]
pub struct OutputFileOpts {
    /// The output file to write to.
    #[structopt(parse(from_os_str), short = "o", long = "output")]
    pub output_file_path: Option<PathBuf>,
}
