<!--
  -- Copyright 2020 Contributors to the Parsec project. 
  -- SPDX-License-Identifier: Apache-2.0
--->
# Parsec Tool

This repository contains a tool to communicate with the [Parsec
service](https://github.com/parallaxsecond/parsec) on the command-line.

For demos and to test the Parsec service, you might want to change the Parsec endpoint location.
For that, set the `PARSEC_SERVICE_ENDPOINT` environment variable to correction endpoint.

To set a Unix Domain Socket Listener endpoint at `/tmp/parsec.sock`:

```
$ export PARSEC_SERVICE_ENDPOINT=unix:/tmp/parsec.sock
```

# Demo

[![asciicast](https://asciinema.org/a/bGRK4lFZnCq3UZQSWa7vQfuT5.svg)](https://asciinema.org/a/bGRK4lFZnCq3UZQSWa7vQfuT5)

## License

The software is provided under Apache-2.0. Contributions to this project are accepted under the same license.

## Contributing

Please check the [**Contribution Guidelines**](https://parallaxsecond.github.io/parsec-book/contributing.html)
to know more about the contribution process.
