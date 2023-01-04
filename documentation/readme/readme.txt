
    Table of contents:

      + about; status;
      + documentation; examples;
      + install;
      + licensing; SBOM;
      + chat on Discord, discuss on GitHub, or email author

-------------------------------------------------------------------------------

About

TBD

-------------------------------------------------------------------------------

Status

TBD

-------------------------------------------------------------------------------

Documentation

    WIP (work in progress)

Besides what is available by running z-tokens help and the examples there is no
other documentation at the moment.

That being said, just run the following and start experimenting with the
commands. (If there is need for documentation, besides the frugally -h for each
command, I have failed in one of the mandatory requirements, that of being
"simple to use".)

For how to download and install it see the dedicated section.

Get some help:

z-tokens -h
z-tokens generate -h
z-tokens patterns -h

-------------------------------------------------------------------------------

Examples

TBD

-------------------------------------------------------------------------------

Installation

At the moment, due to the Rust cross-compilation hurdles, one can install
z-tokens by building it with cargo, or downloading the already built Linux or
OSX binaries.

TBD

Building from sources

At the moment z-tokens requires at least Rust 1.63 or later.

Use the latest development branch:

cargo install --bins --git https://github.com/volution/z-tokens --force

Use a particular tag:

cargo install --bins --tag v0.3.1 --git https://github.com/volution/z-tokens --force

It should build at least on the following platforms:

  * Linux: x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl;
  * OSX: x86_64-apple-darwin, aarch64-apple-darwin;
  * Android: x86_64-linux-android, aarch64-linux-android;
  * FreeBSD: only x86_64-unknown-freebsd;
  * OpenBSD: not tested, but I don't see why it shouldn't build;
  * Windows: x86_64-pc-windows-msvc, aarch64-pc-windows-msvc;

The build status was assessed by running cargo check --target ....

Downloading pre-built binaries

  * download the executable and (optiotal) signature (replace linux with darwin
    (for OSX), freebsd or openbsd, and use x86_64 or aarch64 matching your
    processor):

curl -s -L -f -S \
    -o /tmp/z-tokens \
    https://github.com/volution/z-tokens/releases/download/v0.3.1/z-tokens--linux--x86_64--v0.3.1

curl -s -L -f -S \
    -o /tmp/z-tokens.asc \
    https://github.com/volution/z-tokens/releases/download/v0.3.1/z-tokens--linux--x86_64--v0.3.1.asc

  * (optional) import my PGP key:

curl -s -f -S https://github.com/cipriancraciun.gpg | gpg2 --import

  * (optional) verify the executable:

gpg --verify /tmp/z-tokens.asc /tmp/z-tokens

  * check that the key is 58FC 2194 FCC2 4783 99CB 220C 5A97 4037 A6FD 8839;

  * change the executable permissions:

chmod a=rx /tmp/z-tokens

  * copy the executable on the $PATH:

sudo cp /tmp/z-tokens /usr/local/bin/z-tokens

  * check that it works:

z-tokens --version

0.3.1

-------------------------------------------------------------------------------

Notice (copyright and licensing)

Notice -- short version

The code is licensed under GPL 3 or later.

Notice -- long version

For details about the copyright and licensing, please consult the notice.txt
file in the documentation/licensing folder.

If someone requires the sources and/or documentation to be released under a
different license, please send an email to the authors, stating the licensing
requirements, accompanied by the reasons and other details; then, depending on
the situation, the authors might release the sources and/or documentation under
a different license.

SBOM (Software Bill of Materials)

This project, like many other open-source projects, incorporates code from
other open-source projects (besides other tools used to develop, build and
test).

Strictly related to the project's dependencies (direct and transitive), please
see the SBOM (Software Bill of Materials) for links to these dependencies and
their licenses.

