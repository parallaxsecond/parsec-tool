#!/usr/bin/env bash

# Copyright 2020 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0

set -xeuf -o pipefail

cleanup() {
	# Remove created keys in case we failed somewhere
	./target/debug/parsec-tool delete-key -k ecc-key 2>/dev/null || true
	./target/debug/parsec-tool delete-key -k rsa-key 2>/dev/null || true
}

trap cleanup EXIT

# Points to Parsec's Unix Domain Socket on the CI
export PARSEC_SERVICE_ENDPOINT="unix:/tmp/parsec.sock"
export RUST_LOG=error

#########
# Build #
#########
RUST_BACKTRACE=1 cargo build
RUST_BACKTRACE=1 cargo build --features spiffe-auth

#################
# Static checks #
#################
# On native target clippy or fmt might not be available.
if cargo fmt -h; then
	cargo fmt --all -- --check
fi
if cargo clippy -h; then
	cargo clippy --all-targets -- -D clippy::all -D clippy::cargo
fi

#############
# CLI tests #
#############
./target/debug/parsec-tool --help

./target/debug/parsec-tool ping > /tmp/ping
git diff --exit-code /tmp/ping tests/expected_output/ping

./target/debug/parsec-tool list-authenticators > /tmp/list-authenticators
git diff --exit-code /tmp/list-authenticators tests/expected_output/list-authenticators

./target/debug/parsec-tool list-providers > /tmp/list-providers
git diff --exit-code /tmp/list-providers tests/expected_output/list-providers

# Just checking if the command works as the output list can change
./target/debug/parsec-tool list-opcodes -p 1
./target/debug/parsec-tool list-opcodes

./target/debug/parsec-tool create-ecc-key -k ecc-key
./target/debug/parsec-tool create-rsa-key -k rsa-key

./target/debug/parsec-tool list-keys

./target/debug/parsec-tool export-public-key -k ecc-key
./target/debug/parsec-tool export-public-key -k rsa-key
./target/debug/parsec-tool generate-random -n 50
./target/debug/parsec-tool sign -k ecc-key "Les carottes sont cuites!"

./target/debug/parsec-tool delete-key -k ecc-key
./target/debug/parsec-tool delete-key -k rsa-key
