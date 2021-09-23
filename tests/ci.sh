#!/usr/bin/env bash

# Copyright 2020 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0

set -xeuf -o pipefail

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

PARSEC_TOOL="./target/debug/parsec-tool" tests/parsec-cli-tests.sh -d
