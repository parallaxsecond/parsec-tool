// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Subcommand implementations. Interacts with parsec-client-rust.

mod create_csr;
mod create_ecc_key;
mod create_rsa_key;
mod decrypt;
mod delete_client;
mod delete_key;
mod export_public_key;
mod generate_random;
mod list_authenticators;
mod list_clients;
mod list_keys;
mod list_opcodes;
mod list_providers;
mod ping;
mod sign;

use crate::error::{Error::ParsecClientError, Result};
use crate::subcommands::{
    create_csr::CreateCsr, create_ecc_key::CreateEccKey, create_rsa_key::CreateRsaKey,
    decrypt::Decrypt, delete_client::DeleteClient, delete_key::DeleteKey,
    export_public_key::ExportPublicKey, generate_random::GenerateRandom,
    list_authenticators::ListAuthenticators, list_clients::ListClients, list_keys::ListKeys,
    list_opcodes::ListOpcodes, list_providers::ListProviders, ping::Ping, sign::Sign,
};
use parsec_client::BasicClient;
use structopt::StructOpt;

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

    /// Export the public part of the key pair in PEM format
    ExportPublicKey(ExportPublicKey),

    /// Create a RSA key pair (2048 bits). Used by default for asymmetric encryption with RSA PKCS#1 v1.5.
    CreateRsaKey(CreateRsaKey),

    /// Create a ECC key pair (curve secp256r1). Used by default for asymmetric signing with ECDSA (SHA-256).
    CreateEccKey(CreateEccKey),

    /// Decrypt data using the algorithm of the key
    Decrypt(Decrypt),

    /// Sign data using the algorithm of the key (base64 signature)
    Sign(Sign),

    /// Delete a key.
    DeleteKey(DeleteKey),

    /// Lists all clients currently having data in the service (admin operation).
    ListClients(ListClients),

    /// Delete all data a client has in the service (admin operation).
    DeleteClient(DeleteClient),

    /// Create a Certificate Signing Request (CSR) from a keypair.
    CreateCsr(CreateCsr),
}

impl Subcommand {
    /// Runs the subcommand.
    pub fn run(&self, client: BasicClient) -> Result<()> {
        match &self {
            Subcommand::Ping(cmd) => cmd.run(client),
            Subcommand::ListProviders(cmd) => cmd.run(client),
            Subcommand::ListAuthenticators(cmd) => cmd.run(client),
            Subcommand::ListKeys(cmd) => cmd.run(client),
            Subcommand::ListClients(cmd) => cmd.run(client),
            Subcommand::DeleteClient(cmd) => cmd.run(client),
            Subcommand::ListOpcodes(cmd) => cmd.run(client),
            Subcommand::GenerateRandom(cmd) => cmd.run(client),
            Subcommand::ExportPublicKey(cmd) => cmd.run(client),
            Subcommand::CreateRsaKey(cmd) => cmd.run(client),
            Subcommand::CreateEccKey(cmd) => cmd.run(client),
            Subcommand::Sign(cmd) => cmd.run(client),
            Subcommand::Decrypt(cmd) => cmd.run(client),
            Subcommand::DeleteKey(cmd) => cmd.run(client),
            Subcommand::CreateCsr(cmd) => cmd.run(client),
        }
    }
    /// Indicates if subcommand requires authentication
    fn authentication_required(&self) -> bool {
        // Subcommands below don't need authentication - all others do.
        !matches!(
            &self,
            Subcommand::Ping(_)
                | Subcommand::ListProviders(_)
                | Subcommand::ListAuthenticators(_)
                | Subcommand::ListOpcodes(_)
        )
    }

    /// Get BasicClient for operation
    pub fn create_client(&self, app_name: Option<String>) -> Result<BasicClient> {
        let client_result = if self.authentication_required() {
            // BasicClient::new will do default config including setting up authenticator
            BasicClient::new(app_name)
        } else {
            // Create a naked client which should be set up for core operations with no authenticator
            BasicClient::new_naked()
        };
        match client_result {
            Ok(client) => Ok(client),
            Err(err) => Err(ParsecClientError(err)),
        }
    }
}
