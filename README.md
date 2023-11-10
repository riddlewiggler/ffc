# Fun with Forty Client

[![CI](https://github.com/riddlewiggler/ffc/actions/workflows/ci.yml/badge.svg)](https://github.com/riddlewiggler/ffc/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/riddlewiggler/ffc/graph/badge.svg?token=5R70R73WK8)](https://codecov.io/gh/riddlewiggler/ffc)

## TL;DR

Use this [CLI][CLI] tool to clean the certificate authority files
([pkcs11.txt][PKCS11]) used by your browsers and _polluted_ by Forty Client.

## Description

Forty Client happens to pollute periodically the authority certificate files
used by some browsers by appending an additional authority. This turns out to
be a major drag for the browsing performances. In particular during a cold
start the first page fetch takes ages, ending up in a failure.

Fun with Forty Client (`ffc` for brevity) cleans the certificate authority
files ([pkcs11.txt][PKCS11]) used by your browsers and _polluted_ by Forty Client.

## Usage

This will clean both Firefox and Chrome:

```bash
ffc
```

For verbose output, you need to prefix it with `RUST_LOG=trace`:

```bash
RUST_LOG=trace ffc
```

This will clean authority certificates of Chrome browser only:

```bash
ffc -b chrome
```

Print help and exit:

```bash
ffc -h
```

Print version and exit:

```bash
ffc -V
```

TODO
## Limitations

Current logged user home folder

## Supported platforms

Currently only Linux distributions are supported:

- Linux [glibc][glibc-wiki] 2.31 onwards

## Supported browsers

Currently `ffc` supports:

- Google Chrome;
- Mozilla Firefox.

Google Chrome certificate filepath is `~/.pki/nssdb/pkcs11.txt`  
Firefox certificate file is located in `.mozilla/firefox/**/pkcs11.txt`

## Show your support

Give a ⭐️ if this project helped you

## Licence

This project is [MIT][MIT] licensed

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[PKCS11]: https://en.wikipedia.org/wiki/PKCS_11
[glibc-wiki]: https://en.wikipedia.org/wiki/Glibc
[MIT]: ./LICENCE
