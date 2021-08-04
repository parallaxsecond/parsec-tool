# Parsec Tool

<p align="center">
  <a href="https://crates.io/crates/parsec-tool"><img alt="Crates.io" src="https://img.shields.io/crates/v/parsec-tool"></a>
  <a href="https://docs.rs/parsec-tool"><img src="https://docs.rs/parsec-tool/badge.svg" alt="Code documentation"/></a>
</p>

This repository contains a tool to communicate with the [Parsec
service](https://github.com/parallaxsecond/parsec) on the command-line.

## Getting started

To compile and list the available commands:

```
$ cargo build
$ cargo run
```

Ping the service:

```
$ cargo run -- ping
```

## Modifying Parsec service endpoint

For demos and to test the Parsec service, you might want to change the Parsec endpoint location. For
that, set the `PARSEC_SERVICE_ENDPOINT` environment variable to correction endpoint.

To set a Unix Domain Socket Listener endpoint at `/tmp/parsec.sock`:

```
$ export PARSEC_SERVICE_ENDPOINT=unix:/tmp/parsec.sock
```

## Modifying logging output

You can set the `RUST_LOG` environment variable to modify the logging outpout. See [the
documentation](https://docs.rs/env_logger/0.8.3/env_logger/index.html) for more information.

## Data format

Unless specified otherwise below, the data format expected by the commands is the same as describe
in the [Parsec
Book](https://parallaxsecond.github.io/parsec-book/parsec_client/operations/index.html). The
`--help` option of commands might give more information about the expected format.

- ECDSA signatures are formatted using the ASN.1 representation `Ecdsa-Sig-Value` described in [RFC
   3279](https://tools.ietf.org/html/rfc3279#section-2.2.3).
- Plaintext data is expected/shown as a UTF-8 string (input data of `sign`, output data of
   `decrypt`).
- Ciphertext data is expected/shown as base 64 (output data of `sign`, input data of `decrypt`).
- Exported public keys are formatted in PEM. RSA keys are encoded in [`RSAPublicKey`](https://tools.ietf.org/html/rfc3279.html#section-2.3.1)
   ASN.1 format. EC keys are encoded in the uncompressed format described in 
   [_SEC 1: Elliptic Curve Cryptography_ §2.3.3](https://www.secg.org/sec1-v2.pdf).

## SPIFFE based authenticator

To be able to authenticate with the [JWT-SVID
authenticator](https://parallaxsecond.github.io/parsec-book/parsec_service/authenticators.html#jwt-spiffe-verifiable-identity-document-authenticator),
compile this crate with the `spiffe-auth` feature.

# Demo

[![asciicast](https://asciinema.org/a/RNPjvbgKDlQ0FRFUUKjjNUom6.svg)](https://asciinema.org/a/RNPjvbgKDlQ0FRFUUKjjNUom6)

# License

The software is provided under Apache-2.0. Contributions to this project are accepted under the same
license.

# Contributing

Please check the [**Contribution
Guidelines**](https://parallaxsecond.github.io/parsec-book/contributing/index.html) to know more
about the contribution process.

*Copyright 2020 Contributors to the Parsec project.*
