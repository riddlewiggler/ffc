# Fun with Forty Client

[![CI](https://github.com/riddlewiggler/ffc/actions/workflows/ci.yml/badge.svg)](https://github.com/riddlewiggler/ffc/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/riddlewiggler/ffc/graph/badge.svg?token=5R70R73WK8)](https://codecov.io/gh/riddlewiggler/ffc)

## TL;DR

Use this [CLI][CLI] tool to clean the certificate authority files
([PKCS11][PKCS11]) used by your browsers and *polluted* by Forty Client.

## Description

Forty Client happens to pollute the authority certificate files used by some
browsers appending an additional authority. This turns out to be a major drag
for the browsing performances. In particular during a cold start the first page
fetch takes ages, ending up in a failure.

Fun with Forty Client (`ffc` for brevity) can clean the certificate authority files
([PKCS11][PKCS11]) used by your browsers and *polluted* by Forty Client.

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[PKCS11]: https://en.wikipedia.org/wiki/PKCS_11
