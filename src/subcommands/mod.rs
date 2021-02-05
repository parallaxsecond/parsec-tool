// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Subcommand implementations. Interacts with parsec-client-rust.

pub mod common;
pub mod create_ecc_key;
pub mod create_rsa_key;
pub mod decrypt;
pub mod delete_client;
pub mod delete_key;
pub mod encrypt;
pub mod export_key;
pub mod export_public_key;
pub mod generate_random;
pub mod list_authenticators;
pub mod list_clients;
pub mod list_keys;
pub mod list_opcodes;
pub mod list_providers;
pub mod ping;
pub mod sign;
pub mod verify;

use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::{
    create_ecc_key::CreateEccKey, create_rsa_key::CreateRsaKey, decrypt::Decrypt,
    delete_client::DeleteClient, delete_key::DeleteKey, encrypt::Encrypt, export_key::ExportKey,
    export_public_key::ExportPublicKey, generate_random::GenerateRandom,
    list_authenticators::ListAuthenticators, list_clients::ListClients, list_keys::ListKeys,
    list_opcodes::ListOpcodes, list_providers::ListProviders, ping::Ping, sign::Sign,
    verify::Verify,
};
use parsec_client::core::interface::operations::NativeOperation;
use parsec_client::BasicClient;
use std::convert::TryInto;
use structopt::StructOpt;

/// A trait to represent a `parsec-tool` subcommand. Subcommands have three important properties:
/// - They have their own command-line interface, hence the dependency on `StructOpt`.
/// - They are convertible to a `NativeOperation` -- i.e. they can all be converted to messages to
///   the Parsec service. The conversion is fallible.
/// - They implement `run`, which executes the subcommand.
pub trait ParsecToolSubcommand<'a>
where
    Self: 'a,
    Self: StructOpt,
    &'a Self: TryInto<NativeOperation>,
{
    /// Run the subcommand.
    fn run(&self, matches: &ParsecToolApp, client: BasicClient) -> Result<(), ParsecToolError>;
}

/// Command-line interface to Parsec operations.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Ping the Parsec service and prints the wire protocol version.
    Ping(Ping),

    /// List the available providers supported by the Parsec service.
    ListProviders(ListProviders),

    /// List the available authenticators supported by the Parsec service.
    ListAuthenticators(ListAuthenticators),

    /// List the supported opcodes for a given provider.
    ListOpcodes(ListOpcodes),

    /// List all keys belonging to the application.
    ListKeys(ListKeys),

    /// Generate a sequence of random bytes.
    GenerateRandom(GenerateRandom),

    /// Export the public part of the key pair
    ExportPublicKey(ExportPublicKey),

    /// Export the key
    ExportKey(ExportKey),

    /// Create a RSA key pair (2048 bits). Used by default for asymmetric encryption with RSA PKCS#1 v1.5.
    CreateRsaKey(CreateRsaKey),

    /// Create a ECC key pair (curve secp256r1). Used by default for asymmetric signing with ECDSA (SHA-256).
    CreateEccKey(CreateEccKey),

    /// Encrypt data using the algorithm of the key
    Encrypt(Encrypt),

    /// Decrypt data using the algorithm of the key
    Decrypt(Decrypt),

    /// Sign data using the algorithm of the key
    Sign(Sign),

    /// Verify data using the algorithm of the key
    Verify(Verify),

    /// Delete a key.
    DeleteKey(DeleteKey),

    /// Lists all clients currently having data in the service (admin operation).
    ListClients(ListClients),

    /// Delete all data a client has in the service (admin operation).
    DeleteClient(DeleteClient),
}

impl Subcommand {
    /// Runs the subcommand.
    pub fn run(&self, matches: &ParsecToolApp, client: BasicClient) -> Result<(), ParsecToolError> {
        match &self {
            Subcommand::Ping(cmd) => cmd.run(matches, client),
            Subcommand::ListProviders(cmd) => cmd.run(matches, client),
            Subcommand::ListAuthenticators(cmd) => cmd.run(matches, client),
            Subcommand::ListKeys(cmd) => cmd.run(matches, client),
            Subcommand::ListClients(cmd) => cmd.run(matches, client),
            Subcommand::DeleteClient(cmd) => cmd.run(matches, client),
            Subcommand::ListOpcodes(cmd) => cmd.run(matches, client),
            Subcommand::GenerateRandom(cmd) => cmd.run(matches, client),
            Subcommand::ExportPublicKey(cmd) => cmd.run(matches, client),
            Subcommand::ExportKey(cmd) => cmd.run(matches, client),
            Subcommand::CreateRsaKey(cmd) => cmd.run(matches, client),
            Subcommand::CreateEccKey(cmd) => cmd.run(matches, client),
            Subcommand::Sign(cmd) => cmd.run(matches, client),
            Subcommand::Verify(cmd) => cmd.run(matches, client),
            Subcommand::Encrypt(cmd) => cmd.run(matches, client),
            Subcommand::Decrypt(cmd) => cmd.run(matches, client),
            Subcommand::DeleteKey(cmd) => cmd.run(matches, client),
        }
    }
}
