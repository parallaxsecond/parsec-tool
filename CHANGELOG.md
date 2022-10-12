# Changelog

## [0.5.4](https://github.com/parallaxsecond/parsec-tool/tree/HEAD)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.5.3...HEAD)

**Closed issues:**

- parsec-cli-tests.sh incorrectly fails with openssl 3.0.5 [\#90](https://github.com/parallaxsecond/parsec-tool/issues/90)

**Merged pull requests:**

- Make RSA key string generic [\#92](https://github.com/parallaxsecond/parsec-tool/pull/92) ([gowthamsk-arm](https://github.com/gowthamsk-arm))

## [0.5.3](https://github.com/parallaxsecond/parsec-tool/tree/0.5.3) (2022-09-12)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.5.2...0.5.3)

**Implemented enhancements:**

- Support serialNumber attribute as part of DN for CSR [\#84](https://github.com/parallaxsecond/parsec-tool/issues/84)
- Allow strength of RSA keys to be specified on the command-line [\#83](https://github.com/parallaxsecond/parsec-tool/issues/83)

**Merged pull requests:**

- Update Chanege log for the release 0.5.3 [\#89](https://github.com/parallaxsecond/parsec-tool/pull/89) ([mohamedasaker-arm](https://github.com/mohamedasaker-arm))
- Release prep 0.5.3 [\#88](https://github.com/parallaxsecond/parsec-tool/pull/88) ([mohamedasaker-arm](https://github.com/mohamedasaker-arm))
- Support a 'bits' argument for specifying the size/strength of RSA keys. [\#86](https://github.com/parallaxsecond/parsec-tool/pull/86) ([paulhowardarm](https://github.com/paulhowardarm))
- Support serialNumber field of Distinguished Name for CSRs. [\#85](https://github.com/parallaxsecond/parsec-tool/pull/85) ([paulhowardarm](https://github.com/paulhowardarm))
- Update cargo audit configuration [\#81](https://github.com/parallaxsecond/parsec-tool/pull/81) ([hug-dev](https://github.com/hug-dev))
- Add an encrypt command plus tests. [\#80](https://github.com/parallaxsecond/parsec-tool/pull/80) ([paulhowardarm](https://github.com/paulhowardarm))

## [0.5.2](https://github.com/parallaxsecond/parsec-tool/tree/0.5.2) (2022-03-21)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.5.1...0.5.2)

**Merged pull requests:**

- Prepare 0.5.2 release [\#78](https://github.com/parallaxsecond/parsec-tool/pull/78) ([ionut-arm](https://github.com/ionut-arm))
- Update `regex` to 1.5.5 [\#77](https://github.com/parallaxsecond/parsec-tool/pull/77) ([ionut-arm](https://github.com/ionut-arm))
- Update version of `rcgen` we use [\#76](https://github.com/parallaxsecond/parsec-tool/pull/76) ([ionut-arm](https://github.com/ionut-arm))

## [0.5.1](https://github.com/parallaxsecond/parsec-tool/tree/0.5.1) (2022-02-22)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.5.0...0.5.1)

**Merged pull requests:**

- Update changelog for 0.5.x [\#74](https://github.com/parallaxsecond/parsec-tool/pull/74) ([ionut-arm](https://github.com/ionut-arm))
- Update dependencies that have been yanked [\#73](https://github.com/parallaxsecond/parsec-tool/pull/73) ([ionut-arm](https://github.com/ionut-arm))
- Bump version of PSA Crypto crates [\#72](https://github.com/parallaxsecond/parsec-tool/pull/72) ([ionut-arm](https://github.com/ionut-arm))

## [0.5.0](https://github.com/parallaxsecond/parsec-tool/tree/0.5.0) (2022-02-15)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.4.0...0.5.0)

**Fixed bugs:**

- Some commands should not need authentication [\#49](https://github.com/parallaxsecond/parsec-tool/issues/49)

**Closed issues:**

- Error detection broken in parsec-cli-tests.sh  [\#66](https://github.com/parallaxsecond/parsec-tool/issues/66)

**Merged pull requests:**

- Prepare to release new version [\#71](https://github.com/parallaxsecond/parsec-tool/pull/71) ([ionut-arm](https://github.com/ionut-arm))
- Small output fixes [\#70](https://github.com/parallaxsecond/parsec-tool/pull/70) ([anta5010](https://github.com/anta5010))
- Support the creation of RSA signing keys as well as encryption keys [\#69](https://github.com/parallaxsecond/parsec-tool/pull/69) ([paulhowardarm](https://github.com/paulhowardarm))
- Early support for certificate request generation in parsec-tool [\#68](https://github.com/parallaxsecond/parsec-tool/pull/68) ([paulhowardarm](https://github.com/paulhowardarm))
- Avoid using pipes to run all test commands in the same sub-shell [\#67](https://github.com/parallaxsecond/parsec-tool/pull/67) ([anta5010](https://github.com/anta5010))
- fix \#49 [\#65](https://github.com/parallaxsecond/parsec-tool/pull/65) ([jn9e9](https://github.com/jn9e9))
- Upgrade client's version [\#64](https://github.com/parallaxsecond/parsec-tool/pull/64) ([hug-dev](https://github.com/hug-dev))
- Update CHANGELOG.md with 0.4.0 details [\#63](https://github.com/parallaxsecond/parsec-tool/pull/63) ([anta5010](https://github.com/anta5010))

## [0.4.0](https://github.com/parallaxsecond/parsec-tool/tree/0.4.0) (2021-09-24)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.3.1...0.4.0)

**Implemented enhancements:**

- Add round-trip testing [\#47](https://github.com/parallaxsecond/parsec-tool/issues/47)

**Merged pull requests:**

- Bump version to 0.4.0 [\#62](https://github.com/parallaxsecond/parsec-tool/pull/62) ([anta5010](https://github.com/anta5010))
- Bash wrapper for parsec-tool to run basic e2e Parsec tests [\#61](https://github.com/parallaxsecond/parsec-tool/pull/61) ([anta5010](https://github.com/anta5010))
- Update CHaNGELOG [\#59](https://github.com/parallaxsecond/parsec-tool/pull/59) ([hug-dev](https://github.com/hug-dev))

## [0.3.1](https://github.com/parallaxsecond/parsec-tool/tree/0.3.1) (2021-08-04)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.3.0...0.3.1)

**Implemented enhancements:**

- --provider option for list-opcodes is not consistent [\#53](https://github.com/parallaxsecond/parsec-tool/issues/53)
- Use the implicit provider for list\_opcodes [\#54](https://github.com/parallaxsecond/parsec-tool/pull/54) ([hug-dev](https://github.com/hug-dev))

**Fixed bugs:**

- Update sha2 version [\#57](https://github.com/parallaxsecond/parsec-tool/pull/57) ([hug-dev](https://github.com/hug-dev))

**Closed issues:**

- Update the demo with most recent contents [\#44](https://github.com/parallaxsecond/parsec-tool/issues/44)

**Merged pull requests:**

- Prepare for the next release [\#58](https://github.com/parallaxsecond/parsec-tool/pull/58) ([hug-dev](https://github.com/hug-dev))
- Add cargo-audit config file [\#56](https://github.com/parallaxsecond/parsec-tool/pull/56) ([ionut-arm](https://github.com/ionut-arm))
- Update the CHANGELOG file [\#51](https://github.com/parallaxsecond/parsec-tool/pull/51) ([hug-dev](https://github.com/hug-dev))

## [0.3.0](https://github.com/parallaxsecond/parsec-tool/tree/0.3.0) (2021-03-18)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.2.0...0.3.0)

**Closed issues:**

- Tag 0.2.0 and upload on crates.io [\#45](https://github.com/parallaxsecond/parsec-tool/issues/45)

**Merged pull requests:**

- Prepare for 0.3.0 release [\#50](https://github.com/parallaxsecond/parsec-tool/pull/50) ([hug-dev](https://github.com/hug-dev))
- Specify format of public keys in README [\#48](https://github.com/parallaxsecond/parsec-tool/pull/48) ([ionut-arm](https://github.com/ionut-arm))

## [0.2.0](https://github.com/parallaxsecond/parsec-tool/tree/0.2.0) (2021-02-23)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/0.1.0...0.2.0)

**Implemented enhancements:**

- Make the output of export-public-key more useful. [\#28](https://github.com/parallaxsecond/parsec-tool/issues/28)
- Add encrypt/decrypt support [\#27](https://github.com/parallaxsecond/parsec-tool/issues/27)
- Format public key to PEM [\#38](https://github.com/parallaxsecond/parsec-tool/pull/38) ([ionut-arm](https://github.com/ionut-arm))
- Add BasicClient to ParsecToolApp [\#35](https://github.com/parallaxsecond/parsec-tool/pull/35) ([ionut-arm](https://github.com/ionut-arm))

**Fixed bugs:**

- Review the default options [\#30](https://github.com/parallaxsecond/parsec-tool/issues/30)

**Closed issues:**

- Add basic CLI tests on the CI [\#42](https://github.com/parallaxsecond/parsec-tool/issues/42)
- Check if it is possible to use the BasicClient for operations [\#36](https://github.com/parallaxsecond/parsec-tool/issues/36)
- Add support for sign/verify [\#31](https://github.com/parallaxsecond/parsec-tool/issues/31)
- Automatic key creation for some operations [\#29](https://github.com/parallaxsecond/parsec-tool/issues/29)
- Rename commands to remove PSA prefix and make them more user-friendly [\#26](https://github.com/parallaxsecond/parsec-tool/issues/26)
- Add support for ListClients and DeleteClient [\#22](https://github.com/parallaxsecond/parsec-tool/issues/22)

**Merged pull requests:**

- Add some CLI tests on the CI [\#46](https://github.com/parallaxsecond/parsec-tool/pull/46) ([hug-dev](https://github.com/hug-dev))
- Use log crate instead of custom logging logic [\#43](https://github.com/parallaxsecond/parsec-tool/pull/43) ([hug-dev](https://github.com/hug-dev))
- Add a timeout CLI option [\#41](https://github.com/parallaxsecond/parsec-tool/pull/41) ([hug-dev](https://github.com/hug-dev))
- Add decrypt/sign and simplify things [\#39](https://github.com/parallaxsecond/parsec-tool/pull/39) ([hug-dev](https://github.com/hug-dev))
- Rectify the key creation operations [\#34](https://github.com/parallaxsecond/parsec-tool/pull/34) ([hug-dev](https://github.com/hug-dev))
- Rename things with more friendly names [\#33](https://github.com/parallaxsecond/parsec-tool/pull/33) ([hug-dev](https://github.com/hug-dev))
- Replace default\_value with Option [\#32](https://github.com/parallaxsecond/parsec-tool/pull/32) ([hug-dev](https://github.com/hug-dev))
- Update the Rust client to the spiffe-less version [\#25](https://github.com/parallaxsecond/parsec-tool/pull/25) ([hug-dev](https://github.com/hug-dev))
- Add ListClients and DeleteClient operations [\#24](https://github.com/parallaxsecond/parsec-tool/pull/24) ([hug-dev](https://github.com/hug-dev))
- Update dependencies [\#23](https://github.com/parallaxsecond/parsec-tool/pull/23) ([ionut-arm](https://github.com/ionut-arm))
- Disable Travis CI builds and update Cargo.lock [\#21](https://github.com/parallaxsecond/parsec-tool/pull/21) ([ionut-arm](https://github.com/ionut-arm))
- Add project changelog [\#20](https://github.com/parallaxsecond/parsec-tool/pull/20) ([ionut-arm](https://github.com/ionut-arm))
- Remove unused anyhow [\#19](https://github.com/parallaxsecond/parsec-tool/pull/19) ([hug-dev](https://github.com/hug-dev))
- Upgrade the client's version to add SPIFFE support [\#18](https://github.com/parallaxsecond/parsec-tool/pull/18) ([hug-dev](https://github.com/hug-dev))
- Add list-authenticators subcommand [\#17](https://github.com/parallaxsecond/parsec-tool/pull/17) ([hug-dev](https://github.com/hug-dev))

## [0.1.0](https://github.com/parallaxsecond/parsec-tool/tree/0.1.0) (2020-10-20)

[Full Changelog](https://github.com/parallaxsecond/parsec-tool/compare/d36eb9f5d2e57fc29924c7e32c11da0c66b4ba4e...0.1.0)

**Implemented enhancements:**

- Make use of client bootstrapping functionality [\#16](https://github.com/parallaxsecond/parsec-tool/pull/16) ([ionut-arm](https://github.com/ionut-arm))
- Add the generate and destroy key operations [\#13](https://github.com/parallaxsecond/parsec-tool/pull/13) ([hug-dev](https://github.com/hug-dev))
- Upgrade the client version [\#12](https://github.com/parallaxsecond/parsec-tool/pull/12) ([hug-dev](https://github.com/hug-dev))
- Upgrade dependencies [\#10](https://github.com/parallaxsecond/parsec-tool/pull/10) ([hug-dev](https://github.com/hug-dev))
- Add asciinema demo [\#5](https://github.com/parallaxsecond/parsec-tool/pull/5) ([joechrisellis](https://github.com/joechrisellis))
- Add initial parsec-tool implementation [\#1](https://github.com/parallaxsecond/parsec-tool/pull/1) ([joechrisellis](https://github.com/joechrisellis))

**Closed issues:**

- Use the bootstrapping client [\#15](https://github.com/parallaxsecond/parsec-tool/issues/15)
- asciinema demo [\#2](https://github.com/parallaxsecond/parsec-tool/issues/2)

**Merged pull requests:**

- Add list-keys subcommand [\#14](https://github.com/parallaxsecond/parsec-tool/pull/14) ([joechrisellis](https://github.com/joechrisellis))
- Add psa-export-key subcommand [\#9](https://github.com/parallaxsecond/parsec-tool/pull/9) ([joechrisellis](https://github.com/joechrisellis))
- Add psa-export-public-key subcommand [\#8](https://github.com/parallaxsecond/parsec-tool/pull/8) ([joechrisellis](https://github.com/joechrisellis))
- List providers UUID fix [\#6](https://github.com/parallaxsecond/parsec-tool/pull/6) ([joechrisellis](https://github.com/joechrisellis))
- Move subcommand dispatching to `Subcommand` enum [\#4](https://github.com/parallaxsecond/parsec-tool/pull/4) ([joechrisellis](https://github.com/joechrisellis))
- Add psa-generate-random subcommand [\#3](https://github.com/parallaxsecond/parsec-tool/pull/3) ([joechrisellis](https://github.com/joechrisellis))



\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
