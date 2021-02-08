// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Common facilities and options for subcommands.

use crate::error::{Error, Result};
use parsec_client::core::interface::operations::psa_key_attributes::Attributes;
use parsec_client::core::interface::requests::ResponseStatus;
use parsec_client::BasicClient;

/// Get the key attributes.
pub fn key_attributes(basic_client: &BasicClient, key_name: &str) -> Result<Attributes> {
    // First let's find the key to find its algorithm
    let keys = basic_client.list_keys()?;
    let key_info = keys
        .into_iter()
        .find(|key_info| key_info.name == key_name)
        .ok_or(Error::ParsecClientError(
            parsec_client::error::Error::Service(ResponseStatus::PsaErrorDoesNotExist),
        ))?;
    Ok(key_info.attributes)
}
