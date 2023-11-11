# Fun with Forty Client

[![CI](https://github.com/riddlewiggler/ffc/actions/workflows/ci-main.yml/badge.svg)](https://github.com/riddlewiggler/ffc/actions/workflows/ci-main.yml)
[![codecov](https://codecov.io/gh/riddlewiggler/ffc/graph/badge.svg?token=5R70R73WK8)](https://codecov.io/gh/riddlewiggler/ffc)

## TL;DR

Use this [CLI][CLI] tool to clean the certificate authority files
([pkcs11.txt][PKCS11]) used by your browsers and _polluted_ by Forty Client.

## Description

Forty Client happens to pollute periodically the authority certificate files
used by some browsers by appending an additional authority. This turns out to be
a major drag for the browsing performances. In particular during a cold start,
the first page fetch takes ages, ending up in a failure.

Fun with Forty Client (`ffc` for brevity) cleans the certificate authority files
([pkcs11.txt][PKCS11]) used by your browsers and _polluted_ by Forty Client.

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

## Software limitations

It is intended to be used by the current logged-in linux user, since the
certificate authority files are located by default in the user's home folder.

## Supported platforms

Currently only Linux distributions are supported:

- Linux [glibc][glibc-wiki] 2.31 onwards

## Supported browsers

Currently `ffc` supports:

- Google [Chrome][chrome];
- Mozilla [Firefox][firefox].

Google Chrome certificate filepath is `~/.pki/nssdb/pkcs11.txt`  
Firefox certificate file is located in `~/.mozilla/firefox/**/pkcs11.txt`

## Show your support

Give a ⭐️ if this project helped you

## Disclaimers and limitations on liability

THE SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, AND NO WARRANTY, EITHER EXPRESS OR
IMPLIED, IS GIVEN. YOUR USE OF THE SOFTWARE IS AT YOUR SOLE RISK.
[riddlewiggler][rw] does not warrant that the Software will meet your specific
requirements; the Software is fully compatible with any particular platform;
your use of the Software will be uninterrupted, timely, secure, or error-free;
the results that may be obtained from the use of the Software will be accurate
or reliable; the quality of any products, services, information, or other
material purchased or obtained by you through the Software will meet your
expectations; or any errors in the Software will be corrected.

YOU EXPRESSLY UNDERSTAND AND AGREE THAT [riddlewiggler][rw] SHALL NOT BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, CONSEQUENTIAL OR EXEMPLARY
DAMAGES, INCLUDING BUT NOT LIMITED TO, DAMAGES FOR LOSS OF PROFITS, GOODWILL,
USE, DATA OR OTHER INTANGIBLE LOSSES (EVEN IF [riddlewiggler][rw] HAS BEEN
ADVISED OF THE POSSIBILITY OF SUCH DAMAGES) RELATED TO THE SOFTWARE, including,
for example: the use or the inability to use the Software; the cost of
procurement of substitute goods and services resulting from any goods, data,
information or services purchased or obtained or messages received or
transactions entered into through or from the Software; unauthorized access to
or alteration of your transmissions or data; statements or conduct of any
third-party on the Software; or any other matter relating to the Software.

[riddlewiggler][rw] reserves the right at any time and from time to time to
modify or discontinue, temporarily or permanently, the Software (or any part
thereof) with or without notice. [riddlewiggler][rw] shall not be liable to you
or to any third-party for any modification, price change, suspension or
discontinuance of the Software.

## Licence

This project is [MIT][MIT] licensed

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[PKCS11]: https://en.wikipedia.org/wiki/PKCS_11
[glibc-wiki]: https://en.wikipedia.org/wiki/Glibc
[MIT]: ./LICENCE
[rw]: https://github.com/riddlewiggler
[chrome]: https://en.wikipedia.org/wiki/Google_Chrome
[firefox]: https://en.wikipedia.org/wiki/Firefox
