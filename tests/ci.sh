#!/usr/bin/env bash

# Copyright 2020 Contributors to the Parsec project.
# SPDX-License-Identifier: Apache-2.0

set -xeuf -o pipefail

error_msg () {
    echo "Error: $1"
    exit 1
}

# Points to Parsec's Unix Domain Socket on the CI
export PARSEC_SERVICE_ENDPOINT="unix:/tmp/parsec.sock"
export RUST_LOG=error

#TODO: This applies the rcgen patch that exposes the PKCS_RSA_PSS_SHA256 and PKCS_RSA_PSS_SHA384 types. Remove this
#      when the corresponding patch gets merged. Also remove rcgen+0.9.3.patch.
rustup install 1.77.1 # We know that this version works for patch-crate
cargo +1.77.1 install patch-crate --version 0.1.9
cargo patch-crate

##################
# Get Parameters #
##################
MISMATCHER=
TEST_NEXT_BRANCH_TRACKING=
while [ "$#" -gt 0 ]; do
    case "$1" in
        mismatcher )
            MISMATCHER="True"
        ;;
        --test-next-branch-tracking )
            TEST_NEXT_BRANCH_TRACKING="True"
        ;;
        *)
            error_msg "Unknown argument: $1"
        ;;
    esac
    shift
done

#########################
# Dependency mismatcher #
#########################
if [ "$MISMATCHER" = "True" ]; then

    python3 $(pwd)/parsec/utils/dependency_cross_matcher.py --deps_dir $(pwd)

    python3 $(pwd)/parsec/utils/dependency_cross_matcher.py -c --deps_dir $(pwd)/parsec $(pwd)

    exit 0
fi

#########################
# Next branch tracking  #
#########################
if [ "$TEST_NEXT_BRANCH_TRACKING" ]; then
    echo "Track next branches for parallaxsecond repositories"
    python3 $(pwd)/parsec/utils/release_tracking.py $(pwd)/Cargo.toml
fi
#########
# Build #
#########
rustup --version
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
PARSEC_TOOL="./target/debug/parsec-tool" tests/parsec-cli-tests.sh -d --rsa-key-size 1024
