



![banner](./documentation/assets/github-banner.png)




-------------------------------------------------------------------------------




# `z-tokens` -- Random tokens generation and related tools

> * [about](#about);
>   [status](#status);
> * [documentation](#documentation);
>   [examples](#examples);
> * [install](#install);
> * [licensing](#license);
>   [SBOM](#sbom);
> * [chat on Discord](https://discord.gg/fwQmHGzs7E);
>   [discuss on GitHub](https://github.com/volution/z-tokens/discussions/categories/discussions);
>   [email author](mailto:ciprian.craciun@gmail.com);
> * source code at <https://github.com/volution/z-tokens>




-------------------------------------------------------------------------------




## <span id="about">About</span>


> Welcome to the `z-tokens` project, available at <https://github.com/volution/z-tokens>.

As the title says, `z-tokens` is a Rust-based tool for generating tokens
(especially passwords and other entropy-sensitive tokens)
and other related utilities.
(Although, at the moment it can only generate tokens.)

This project is at its third iteration,
and has its roots in a 2008 CommonLisp script that had the same purpose,
and follows the second Python-based iteration.
(Although the Rust-based implementation is quite more complex than the previous iterations.)

Needless to say, I've used this tool for the last 10+ years for all my password generation,
thus I find it critical for my own use-cases.
I just hope others also find some use in it.

In terms of types of generated tokens, here are a few of them:
* passwords -- from easily pronounceable (see the `cv-lower:*` and `proquint-lower:*` ones)
  complete random ASCII dumps;
* pins -- numeric tokens with various levels of entropy (see the `digits-base10:*` ones);
* raw tokens -- in various encodings (such as `base64`, `base32`, `z85`, `bech32`, etc.);
  (please note that such tokens just use the encoding character set,
  and don't necessarily decode correctly in one of those encodings;)
* IP and MAC addresses;
* UUIDv4;
* timestamps in various representations;
  (not for security purposes, as they have almost 0 entropy;)
* time sorted tokens with various levels of entropy (see the `flake:*` ones);
* seed phrases (see the `mnemonic:*` or `bip0039:*` ones);




-------------------------------------------------------------------------------




## <span id="status">Status</span>


**The Rust code is quite fresh and thus not thoroughly tested and reviewed.**

However, it relies on solid Rust libraries (like `rand`)
and it doesn't do much (like touching the file-system or talking to the network),
thus it shouldn't break anything.

The only critical code at the moment,
--- **which could have major security implications** by weakening the generated tokens ---
is the random token generation, especially the selection from the pattern character sets.
Although I haven't thoroughly reviewed this, it's simple and just delegates
to the `rand` library for the actual random number handling.

The generated patterns should provide the stated entropy bits,
as per `z-tokens patterns`, because these values are computed based on the actual patterns.
Thus even if I missed a character in a given pattern character set,
if your token matches your entropy requirements, then it should be safe enough
(provided the random selection doesn't have issues).




-------------------------------------------------------------------------------




## <span id="documentation">Documentation</span>


Besides what is available by running `z-tokens help`
and this document (especially the [examples section](#examples))
there is no other documentation at the moment.

That being said, just run the following and start experimenting with the commands.
(If there is need for documentation, besides the frugally `-h` for each command, I have failed in one of the mandatory requirements, that of being "simple to use".)

Get some help:
~~~~
z-tokens -h
z-tokens generate -h
z-tokens patterns -h
~~~~

For how to install see the [dedicated section](#install).




-------------------------------------------------------------------------------




## <span id="examples">Examples</span>




### How to generate passwords and other tokens

Generate a few consonant-vowel passwords,
and display for transcribing (do not copy-paste the spaces)
(all the commands are equivalent, as these are the defaults):
~~~~
z-tokens g
z-tokens generate
z-tokens generate -p cv-lower:4
z-tokens generate -p cv-lower:4 -c 10
~~~~
~~~~
cama nera zoju liye
yuxe nefi qahi lasa
nemi koho cuho ciwo
neqi vejo zuso kuvu
cixi muve nefo tipi
mose fafu gudu wizo
sale xoyo wuro quye
kome qide yuyo gumi
bepe domo sota xuci
yiqo kewo himu tebe
~~~~

Generate a few consonant-vowel passwords,
and output for copy-pasting
(again all the commands are equivalent):
~~~~
z-tokens g -C
z-tokens generate -C
z-tokens generate -p cv-lower:4 -C
~~~~

Generate one UUIDv4 token,
and pipe it to another process:
~~~~
z-tokens generate -p uuid-v4 -C -c 1 | clipboard copy
~~~~




### How to list available patters

The `z-tokens patterns` command usually outputs a table with the following format:
* the first column is the pattern identifier (to be used with `z-tokens generate -p {pattern}`);
* the `b` column shows the bits of entropy a given token pattern has;
  i.e. the amount of work for brute-force attack;
  if the number doesn't have a `.` (not even `.0`) it implies that exact number of bits;
* the `c` column shows the token length with spaces or separators
  (or without mandatory separators if `-C` is used);
* the last column is an example of such a token;
  **you can safely use it**, with or without spaces or separators as required
  (although without is suggested, unless these separators are mandatory);

Show a selection of supported patterns (mainly those up to 40 characters in length):
~~~~
z-tokens patterns
~~~~
~~~~
| digits-base2:32        | b   32   | c   35 ||  11001111 11000110 00011001 11110101
| digits-base8:32        | b   96   | c   39 ||  2062 1245 7100 3077 7220 0374 2700 4047
| digits-base10:32       | b  106.3 | c   39 ||  1100 4027 9449 5896 6024 1449 6501 9290
| digits-base16:32       | b  128   | c   39 ||  8bae 1f6f 8019 9ba4 8fd9 edcb 7641 c123
| digits-base32-hex:32   | b  160   | c   39 ||  u5go jm97 n1tl c2gk pnja p3f7 f218 dib2
| digits-base32-rfc:32   | b  160   | c   39 ||  2fui k5nk hvjh ztey fljp jip7 zrem 7iaz
| digits-base64-url:32   | b  192   | c   39 ||  j75P GS9I duPu LJF6 LTi8 -vTZ hJYE WyjX
| digits-base64-rfc:32   | b  192   | c   39 ||  ZUiB zV2q WJ7o c+qL TwwK Cdcf PT/4 cl/G
| digits-base58:32       | b  187.4 | c   39 ||  qG75 Nqyt ZLy6 NBUp BBSJ QuZr Jmxg REVr
| digits-base62:32       | b  190.5 | c   39 ||  kUAa cFug EoFD GMRH InRu wBVt iL5J GClu
| digits-bech32:32       | b  160   | c   39 ||  s9vu 8sne xyuv jv5z azjr ms40 jfv0 zvck
| digits-z85:30          | b  192.2 | c   35 ||  q7i[@ fqq?b vX=9j 3.Kgq Y3r6& J1uaN
| ascii-lower:32         | b  150.4 | c   39 ||  kxuv lcbc acjs dcpx uzvx wtyy rnqf kcph
| ascii-upper:32         | b  150.4 | c   39 ||  HXOK XCOI PBBJ SBMY YTBF RUUG CZGV FUFX
| ascii-mixed:32         | b  182.4 | c   39 ||  eIQy OLRN Pfkl Oeeu huAI fvAe WfFZ XTJl
| ascii-symbols:32       | b  160   | c   39 ||  ~,?! &\(@ /.)! +^%< #'.* "\;] {(;_ ~?"&
| ascii-any:32           | b  209.7 | c   39 ||  4?T. WGBr `|CB NH)z U8j_ [X8W )P@m 5x<_
| cv-lower:8             | b  107.4 | c   39 ||  fojo sopo zuwu hehi roqo deja hawa hoxe
| cv-upper:8             | b  107.4 | c   39 ||  YAJO GIWI TIJO CIMB MURU GOLO TBKU NUTI
| cv-mixed:8             | b  139.4 | c   39 ||  PBxa jAwa memU ceBI PuWu MIbe jaJe tOni
| cv-plus-a:8            | b  107.3 | c   39 ||  zake giji meko xixi lidu fonu mozo SB19
| cv-plus-b:8            | b  109.0 | c   39 ||  xaxe vitu jubo saci yire keyo guvo SO8&
| proquint-lower:6       | b   96   | c   35 ||  pilod kipun zavat nurij hanab jamaz
| proquint-upper:6       | b   96   | c   35 ||  FIZUK BIKUD GALUT KODID GURID HUSIG
| mnemonic:2             | b   64.0 | c   40 ||  mayday manager lobby - glass mambo labor
| bip0039:2              | b   66   | c   34 ||  fatal type latin - meadow base bag
| uuid-v4                | b  122   | c   36 ||  ea532774-49fa-40fd-b2b4-dddd3868d652
| ip-127                 | b   23.9 | c   13 ||  127.51.166.43
| ip-10                  | b   23.9 | c   13 ||  10.103.84.167
| ip-172                 | b   20.8 | c   13 ||  172.14.184.28
| ip-192                 | b   15.9 | c   15 ||  192.168.209.137
| ip-mac                 | b   40   | c   17 ||  02:21:a3:78:e2:d5
| bytes-hex:16           | b  128   | c   32 ||  e3e1fdd733b2ab69b40574c6ac7d0545
| timestamp-iso          | b    0   | c   19 ||  2023-01-04-10-19-29
| timestamp-sec          | b    0   | c   10 ||  1672827569
| timestamp-sec-hex      | b    0   | c   10 ||  0063b552b1
| timestamp-nano         | b    0   | c   19 ||  1672827569961180889
| timestamp-nano-hex     | b    0   | c   18 ||  00173713cfd6042526
| timestamp-flake        | b    0   | c    9 ||  726142769
| timestamp-flake-hex    | b    0   | c    8 ||  2b480f31
| flake:14               | b  112   | c   37 ||  2b480f31-6d13295ff502c28ce8eaf7e4cafb
~~~~

Show all supported patterns identifiers:
~~~~
z-tokens patterns -i
~~~~

Show only patterns that have between 64 and 90 bits of entropy:
~~~~
z-tokens patterns -b 64 -B 90
~~~~

Show only patterns that have all types of characters
(lower and upper letters, digits, and symbols):
~~~~
z-tokens patterns -D
~~~~




-------------------------------------------------------------------------------




## <span id="install">Installation</span>


At the moment, due to the Rust cross-compilation hurdles,
one can install `z-tokens` by [building from sources](#build) with `cargo`,
or [downloading the already built](#download) Linux or OSX binaries.




### <span id="build">Building from sources</span>

At the moment `z-tokens` requires at least Rust 1.63 or later.

Use the latest development branch:
~~~~
cargo install --bins --git https://github.com/volution/z-tokens --force
~~~~

Use a particular tag:
~~~~
cargo install --bins --tag v0.3.1 --git https://github.com/volution/z-tokens --force
~~~~

It should build at least on the following platforms:
* Linux:  `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`;
* OSX:  `x86_64-apple-darwin`, `aarch64-apple-darwin`;
* Android: `x86_64-linux-android`, `aarch64-linux-android`;
* FreeBSD:  only `x86_64-unknown-freebsd`;
* OpenBSD:  not tested, but I don't see why it shouldn't build;
* Windows:  `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`;

The build status was assessed by running `cargo check --target ...`.




### <span id="download">Downloading pre-built binaries</span>

* download the executable and (optional) signature
(replace `linux` with `darwin` (for OSX), `freebsd` or `openbsd`,
and use `x86_64` or `aarch64` matching your processor):
~~~~
curl -s -L -f -S \
    -o /tmp/z-tokens \
    https://github.com/volution/z-tokens/releases/download/v0.3.1/z-tokens--linux--x86_64--v0.3.1
~~~~
~~~~
curl -s -L -f -S \
    -o /tmp/z-tokens.asc \
    https://github.com/volution/z-tokens/releases/download/v0.3.1/z-tokens--linux--x86_64--v0.3.1.asc
~~~~

* (optional) import my PGP key:
~~~~
curl -s -f -S https://github.com/cipriancraciun.gpg | gpg2 --import
~~~~

* (optional) verify the executable:
~~~~
gpg --verify /tmp/z-tokens.asc /tmp/z-tokens
~~~~

* **check that the key is `58FC 2194 FCC2 4783 99CB  220C 5A97 4037 A6FD 8839`**;

* change the executable permissions:
~~~~
chmod a=rx /tmp/z-tokens
~~~~

* copy the executable on the `$PATH`:
~~~~
sudo cp /tmp/z-tokens /usr/local/bin/z-tokens
~~~~

* check that it works:
~~~~
z-tokens --version
~~~~
~~~~
0.3.1
~~~~




-------------------------------------------------------------------------------




## <span id="license">Notice (copyright and licensing)</span>


### Notice -- short version

The code is licensed under GPL 3 or later.


### Notice -- long version

For details about the copyright and licensing, please consult the [`notice.txt`](./documentation/licensing/notice.txt) file in the [`documentation/licensing`](./documentation/licensing) folder.

If someone requires the sources and/or documentation to be released
under a different license, please email the authors,
stating the licensing requirements, accompanied by the reasons
and other details; then, depending on the situation, the authors might
release the sources and/or documentation under a different license.


### <span id="sbom">SBOM (Software Bill of Materials)</span>

This project, like many other open-source projects,
incorporates code from other open-source projects
(besides other tools used to develop, build and test).

Strictly related to the project's dependencies (direct and transitive),
please see the [SBOM (Software Bill of Materials)](./documentation/sbom/sbom.md)
for links to these dependencies and their licenses.

