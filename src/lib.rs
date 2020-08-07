// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Source code for the `parsec-tool` project. This is a command-line interface for interacting
//! with the Parsec service.

#![deny(
    nonstandard_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]
// This one is hard to avoid.
#![allow(clippy::multiple_crate_versions)]

#[macro_use]
pub mod util;

pub mod cli;
pub mod common;
pub mod error;
pub mod subcommands;
