// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Output helpers.

/// Gets a colorized message for outputting to a particular stream. Do not use colors if the stream
/// is not a tty.
#[macro_export]
macro_rules! colorize {
    ($color:ident, $stream:ident, $arg:tt) => {{
        let style = if atty::is(atty::Stream::$stream) {
            ansi_term::Color::$color.normal()
        } else {
            ansi_term::Style::default()
        };
        style.paint($arg)
    }};
    ($color:ident, $stream:ident, $($arg:tt)*) => {{
        let style = if atty::is(atty::Stream::$stream) {
            ansi_term::Color::$color.normal()
        } else {
            ansi_term::Style::default()
        };
        style.paint(format!($($arg)*))
    }};
}

/// Wrapper around `eprint!` that accepts a color as its first argument.
#[macro_export]
macro_rules! eprint_colored {
    ($color:ident, $($arg:tt)*) => {
        eprint!("{}", $crate::colorize!($color, Stderr, $($arg)*))
    };
}

/// Wrapper around `eprintln!` that accepts a color as its first argument.
#[macro_export]
macro_rules! eprintln_colored {
    ($color:ident, $($arg:tt)*) => {
        eprintln!("{}", $crate::colorize!($color, Stderr, $($arg)*))
    };
}

// COLORS:
// The output colour of a particular object of text should convey some information about what
// exactly it is. These are not hard and fast rules, but generally speaking:
// - Blue for general information.
// - Green for success.
// - Red for errors and warnings.
// - Cyan should be used for section titles.
// - Yellow should represent field names -- i.e. UUIDs, versions, etc.
//
// By sticking to this guideline we ensure that the CLI stays consistent.

/// Write an info message to stdout.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::eprint_colored!(Blue, "[INFO] ");
        eprintln!($($arg)*);
    };
}

/// Write an error message to stdout.
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        $crate::eprint_colored!(Red, "[ERR] ");
        eprintln!($($arg)*);
    };
}

/// Write a success message to stdout.
#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        $crate::eprint_colored!(Green, "[SUCCESS] ");
        eprintln!($($arg)*);
    };
}

/// Write a title to stdout.
#[macro_export]
macro_rules! title {
    ($($arg:tt)*) => {
        $crate::eprintln_colored!(Cyan, "[{}]", format!($($arg)*));
    };
}

/// Write a field to stdout.
#[macro_export]
macro_rules! field {
    ($fieldname:tt, $($arg:tt)*) => {
        $crate::eprint_colored!(Yellow, "{}: ", $fieldname);
        eprintln!("{}", format!($($arg)*));
    };
}
