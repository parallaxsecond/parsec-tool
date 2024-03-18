// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Source code for the `parsec-tool` project. This is a command-line interface for interacting
//! with the Parsec service.

#![deny(
    dead_code,
    improper_ctypes,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    nonstandard_style,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_bounds,
    private_interfaces,
    renamed_and_removed_lints,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]
// This one is hard to avoid.
#![allow(clippy::multiple_crate_versions)]

pub mod cli;
pub mod common;
pub mod error;
pub mod subcommands;
pub mod util;
