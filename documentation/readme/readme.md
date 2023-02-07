



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
* passwords -- from easily pronounceable (see the `cv-*` and `proquint-*` ones)
  complete random ASCII dumps;
* pins -- numeric tokens with various levels of entropy (see the `digits-base10-*` ones);
* raw tokens -- in various encodings (such as `base64`, `base32`, `z85`, `bech32`, etc.);
  (please note that such tokens just use the encoding character set,
  and don't necessarily decode correctly in one of those encodings;)
* bytes -- hex encoded random bytes (see `bytes-*`);
* IP and MAC addresses;
* UUIDv4;
* timestamps in various representations;
  (not for security purposes, as they have almost 0 entropy;)
* time sorted tokens with various levels of entropy (see the `flake-*` ones);
* seed phrases (see the `mnemonic-*` or `bip0039-*` ones);

Other related tools part of this project are:
* `z-tokens hash` -- (EXPERIMENTAL) hash strings, stdin or files with various hashes (SHA1,2,3 families, Blake2 and Blake3, MD5, etc.);
* `z-tokens exchange` -- (EXPERIMENTAL) encrypt (with X25519 PKI) or armor (like Base64) stdin or files;
* `z-tokens encode` -- (TODO) encode / decode strings, stdin or files to / from various encodings (Base*, Bech32, Z85, etc.);
* `z-tokens secret` -- (TODO) storing, sharing and using security sensitive data in operational and development scripts;
  (for the gist of the idea [see this document](<https://scratchpad.volution.ro/ciprian/992c7f2944456f18cdde77f683f49aa7/e45fc3da.html>);)




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
z-tokens --help

z-tokens generate --help
z-tokens patterns --help

z-tokens hash --help

z-tokens exchange keys --help
z-tokens exchange encrypt --help
z-tokens exchange decrypt --help
z-tokens exchange password --help
z-tokens exchange armor --help
z-tokens exchange dearmor --help
z-tokens exchange ssh keys --help
z-tokens exchange ssh wrap --help

z-tokens --readme
~~~~

For how to install see the [dedicated section](#install).




-------------------------------------------------------------------------------




## <span id="examples">Examples</span>




### How to generate passwords and other tokens

Generate a few consonant-vowel passwords,
and display for transcribing (do not copy-paste the spaces)
(all the commands are almost equivalent):
~~~~
z-tokens generate
z-tokens generate -p cv:4
z-tokens generate -p cv-lower:4
z-tokens generate -p cv-lower:4 -c 10
~~~~
~~~~
cama nera zoju liye
yuxe nefi qahi lasa
....
~~~~

Generate a few consonant-vowel passwords,
and output for copy-pasting
(again all the commands are equivalent):
~~~~
z-tokens g
z-tokens g cv:4
z-tokens g cv-lower:4
z-tokens g cv-lower:4 -c 1

z-tokens generate -C -c 1
z-tokens generate -p cv:4 -C -c 1
z-tokens generate -p cv-lower:4 -C -c 1
~~~~
~~~~
nawerukuhefapeqo
~~~~

Generate one UUIDv4 token,
and pipe it to another process:
~~~~
z-tokens g uuid-v4 | clipboard copy
z-tokens generate -p uuid-v4 -C -c 1 | clipboard copy
~~~~




### How to list available patters

The `z-tokens patterns` command usually outputs a table with the following format:
* the first column is the pattern identifier (to be used with `z-tokens g {pattern}` or `z-tokens generate -p {pattern} ...`);
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
::  digits-base10:16        :   53.1 b :   19 c ::    8372 2896 3577 2481
::  digits-base2:32         :   32  =b :   35 c ::    01000111 00011111 00101001 10111010
::  digits-base8:16         :   48  =b :   19 c ::    3526 1166 6124 4160
::  digits-base16:16        :   64  =b :   19 c ::    9c61 b16d 2c15 c6b2
::  digits-base32-hex:16    :   80  =b :   19 c ::    8nar 036j nhmn 9gut
::  digits-base32-rfc:16    :   80  =b :   19 c ::    3e35 nxvg tz43 fcyq
::  digits-base64-url:16    :   96  =b :   19 c ::    JGHe SXmF -633 BstE
::  digits-base64-rfc:16    :   96  =b :   19 c ::    olRq gBKq TYFk WDPR
::  digits-base58:16        :   93.7 b :   19 c ::    3e2c nqK7 8XDu EXKW
::  digits-base62:16        :   95.2 b :   19 c ::    gs7C 0a2p KGjH Zh2G
::  digits-bech32:16        :   80  =b :   19 c ::    37zs yvvp a3zw fh3s
::  digits-z85:20           :  128.1 b :   23 c ::    p^6MY i5%$t 98iaN rbDT>
::  ascii-lower:16          :   75.2 b :   19 c ::    xwsl rvmg couv ksrl
::  ascii-mixed:16          :   91.2 b :   19 c ::    xwra htbp CTmD fnbs
::  ascii-symbols:16        :   80  =b :   19 c ::    $[%~ $?~| $:(> ?>~(
::  ascii-any:16            :  104.8 b :   19 c ::    ts-c ]dwB GCWl VX<~
::  cv-lower:4              :   53.7 b :   19 c ::    yolu furi qizi poho
::  cv-mixed:4              :   69.7 b :   19 c ::    nurO qILO pasA humi
::  cv-plus-a:3             :   53.5 b :   19 c ::    fitu sozu nube 8237
::  cv-plus-b:3             :   53.6 b :   19 c ::    japa caya dotu JA40
::  cv-plus-c:3             :   55.3 b :   19 c ::    mote vaco mehi VB4:
::  proquint-lower:4        :   64  =b :   23 c ::    lahis nipas tikim daboj
::  koremutake-a:4          :   56  =b :   20 c ::    radry bydy hidy ryla
::  koremutake-b:4          :   84  =b :   33 c ::    pristyfra lyprevy migruse vigrulo
::  mnemonic:1              :   32.0 b :   23 c ::    bicycle society giraffe
::  bip0039:1               :   33  =b :   22 c ::    scrub prosper artefact
::  skey:1                  :   33  =b :   13 c ::    oat bloc slot
::  pgp:1                   :   16  =b :   17 c ::    brackish bifocals
::  eff-large:1             :   38.7 b :   20 c ::    stratus stopped stir
::  eff-short:1             :   31.0 b :   15 c ::    juror desk undo
::  eff-unique:1            :   31.0 b :   25 c ::    opossum unroll vulnerable
::  nato:4                  :   20.6 b :   23 c ::    lima juliett tree oscar
::  uuid-v4                 :  122  =b :   36 c ::    79817f78-d1e9-4d28-b819-b6de695614dd
::  ip-127                  :   23.9 b :   14 c ::    127.138.44.178
::  ip-10                   :   23.9 b :   12 c ::    10.45.220.97
::  ip-172                  :   20.8 b :   13 c ::    172.6.206.246
::  ip-192                  :   15.9 b :   14 c ::    192.168.184.54
::  ip-mac                  :   40  =b :   17 c ::    02:0d:f5:26:2c:bf
::  bytes-hex:16            :  128  =b :   32 c ::    e19fe700e027493457c6ae2782e183cd
::  timestamp-date-time     :    0  =b :   19 c ::    2023-01-10-17-40-03
::  timestamp-date          :    0  =b :   10 c ::    2023-01-10
::  timestamp-time          :    0  =b :    8 c ::    17-40-03
::  timestamp-sec           :    0  =b :   10 c ::    1673372403
::  timestamp-sec-hex       :    0  =b :   10 c ::    0063bda2f3
::  timestamp-nano          :    0  =b :   19 c ::    1673372403336003058
::  timestamp-nano-hex      :    0  =b :   18 c ::    0017390355bdc8c512
::  timestamp-flake         :    0  =b :    9 c ::    726687603
::  timestamp-flake-hex     :    0  =b :    8 c ::    2b505f73
::  flake:2                 :   64  =b :   26 c ::    2b505f73-c10e1333-c5c2da57
~~~~

Show only patterns that have between 64 and 90 bits of entropy:
~~~~
z-tokens patterns -b 64 -B 90
~~~~
~~~~
::  digits-base10:20        :   66.4 b :   24 c ::    5043 3734 7348 1192 0253
::  digits-base2:64         :   64  =b :   71 c ::    00011010 10011101 10110010 10001010 11110110 11010111 00111001 00001111
::  digits-base8:24         :   72  =b :   29 c ::    1450 2511 2733 5136 4335 7161
::  digits-base16:16        :   64  =b :   19 c ::    3026 b4cb c74a 52a7
::  digits-base32-hex:16    :   80  =b :   19 c ::    6mfr sfsl t893 hu9f
::  digits-base32-rfc:16    :   80  =b :   19 c ::    ryqy k7t5 brzq d5jg
::  digits-base64-url:12    :   72  =b :   14 c ::    RVZx hvs0 pqTb
::  digits-base64-rfc:12    :   72  =b :   14 c ::    1s2+ ek51 PwjI
::  digits-base58:12        :   70.2 b :   14 c ::    rTKk 32vP Zsvj
::  digits-base62:12        :   71.4 b :   14 c ::    bLl7 zLeN xkEo
::  digits-bech32:16        :   80  =b :   19 c ::    jah6 cmm8 x077 0p00
::  digits-z85:10           :   64.0 b :   11 c ::    $?#$c KSD}A
::  ascii-lower:16          :   75.2 b :   19 c ::    dnel jjlq qfgo swar
::  ascii-mixed:12          :   68.4 b :   14 c ::    dwdF EOUO qouA
::  ascii-symbols:16        :   80  =b :   19 c ::    "*[~ ](~& ;>*; (}?~
::  ascii-any:12            :   78.6 b :   14 c ::    :0d~ +`u7 ,8j:
::  cv-lower:5              :   67.1 b :   24 c ::    ridu suxe cabu liko kulu
::  cv-mixed:4              :   69.7 b :   19 c ::    febI PAfB CeLo CAnA
::  cv-plus-a:4             :   67.0 b :   24 c ::    zide qutu ceyi yexo 9743
::  cv-plus-b:4             :   67.0 b :   24 c ::    nuxo yazo wuwe zoyu MB23
::  cv-plus-c:4             :   68.7 b :   24 c ::    giyi vika foha haha XB5-
::  proquint-lower:4        :   64  =b :   23 c ::    digun hotap guhuv ruzal
::  koremutake-a:5          :   70  =b :   28 c ::    fuke brydru bebe tregra vori
::  koremutake-b:4          :   84  =b :   31 c ::    pudraky kidruhy bifrufra bipaba
::  mnemonic:2              :   64.0 b :   41 c ::    maximum donor reflex - sugar bombay point
::  bip0039:2               :   66  =b :   41 c ::    vendor loan bounce - media fantasy embark
::  skey:2                  :   66  =b :   30 c ::    coat knit list - tote noah off
::  pgp:4                   :   64  =b :   67 c ::    endow tobacco - orca pedigree - rematch component - trauma molasses
::  eff-large:2             :   77.5 b :   49 c ::    slackness kerchief essence - wasting pelt garbage
::  nato:13                 :   67.2 b :   74 c ::    oscar yankee romeo ait quebec six tango golf wun november foxtrot six zero
::  bytes-hex:8             :   64  =b :   16 c ::    5fc1daf18c09e040
::  flake:2                 :   64  =b :   26 c ::    2b50605b-027c20d4-561c870a
...
~~~~

Show only patterns that have all types of characters
(lower and upper letters, digits, and symbols):
~~~~
z-tokens patterns -A
~~~~
~~~~
::  digits-base64-url:12    :   72  =b :   14 c ::    SJuG nGur T1Bx
::  digits-base64-rfc:12    :   72  =b :   14 c ::    aWRp lIFz V7vf
::  digits-z85:10           :   64.0 b :   11 c ::    }P7*O !oB2C
::  ascii-any:12            :   78.6 b :   14 c ::    xs)7 1W|P qF{l
::  cv-plus-c:4             :   68.7 b :   24 c ::    waqu nagu bono webo ZO8"
...
~~~~

Show the shortest pattern usable for authentication purposes:
~~~~
z-tokens patterns --label password --for-authentication --shortest
~~~~
~~~~
::  digits-base16:8         :   32  =b :    9 c ::    d9fc a6b1
::  digits-base32-hex:8     :   40  =b :    9 c ::    eg3s f6ce
::  digits-base32-rfc:8     :   40  =b :    9 c ::    67sx mkpq
::  digits-base64-url:8     :   48  =b :    9 c ::    Z8F1 0IXM
::  digits-base64-rfc:8     :   48  =b :    9 c ::    ijGK r6Tr
::  digits-base58:8         :   46.8 b :    9 c ::    ZcJ5 3VGy
::  digits-base62:8         :   47.6 b :    9 c ::    aHnQ wlrn
::  digits-bech32:8         :   40  =b :    9 c ::    4vjw n404
::  digits-z85:5            :   32.0 b :    5 c ::    Nm=%t
::  ascii-lower:8           :   37.6 b :    9 c ::    pmir ijhy
::  ascii-mixed:8           :   45.6 b :    9 c ::    ZdMZ PBLt
::  ascii-any:8             :   52.4 b :    9 c ::    G~&A =?3D
::  cv-lower:3              :   40.2 b :   14 c ::    jizi tulo hehu
::  cv-mixed:2              :   34.8 b :    9 c ::    WeDo QaHB
::  cv-plus-a:2             :   40.1 b :   14 c ::    badi yogi 0016
::  cv-plus-b:2             :   40.2 b :   14 c ::    ruvo qoxa PB87
::  cv-plus-c:2             :   41.8 b :   14 c ::    xode quxu MU6'
::  proquint-lower:2        :   32  =b :   11 c ::    baboh nozoz
::  koremutake-a:3          :   42  =b :   16 c ::    pune fravy prony
::  koremutake-b:2          :   42  =b :   13 c ::    pynoru gatudy
::  uuid-v4                 :  122  =b :   36 c ::    898b8295-4709-49cf-9d8b-6170fd5b9e0c
::  bytes-hex:4             :   32  =b :    8 c ::    2b96ae6c
~~~~
~~~~
z-tokens patterns --label passphrase --for-authentication --shortest
~~~~
~~~~
::  mnemonic:1              :   32.0 b :   18 c ::    queen detail first
::  bip0039:1               :   33  =b :   20 c ::    potato injury season
::  skey:1                  :   33  =b :   14 c ::    hulk noah yang
::  pgp:2                   :   32  =b :   34 c ::    topmost speculate - acme tolerance
::  eff-large:1             :   38.7 b :   26 c ::    yearly frivolous schilling
::  eff-short:2             :   62.0 b :   36 c ::    verse erase given - groom mumbo lazy
::  eff-unique:2            :   62.0 b :   47 c ::    unknown nectar running - cadillac falcon enzyme
~~~~

Show all details about a certain pattern:
~~~~
z-tokens patterns -p cv:6 --show-all
~~~~
~~~~
**  ~~~~~~~~  cv-lower:6
\_  aliases:  cv:6
\_  labels:   cv-lower cv ascii password memorable
\_  bits:     80.5709
\_  length:   29
\_  usable for:
    \_  cryptography         !! NO !!      with    -47.43  bits of margin
    \_  authentication          OK         with    +48.57  bits of margin
    \_  archival storage     !! NO !!      with    -19.72  bits of margin
    \_  long term storage       OK         with     +3.73  bits of margin
    \_  short term storage      OK         with    +10.58  bits of margin
\_  bruteforce time:
    \_  MD4                     --    2.0  years
    \_  MD5                     --    3.4  years
    \_  SHA1                    --    1.1  decades
    \_  SHA2-256                --    2.6  decades
    \_  SHA3-256                --    1.1  centuries
    \_  PBKDF2-HMAC-MD5         --   12.3  millennia
    \_  PBKDF2-HMAC-SHA1        --   29.8  millennia
    \_  PBKDF2-HMAC-SHA256      --   64.2  millennia
    \_  PBKDF2-HMAC-SHA512      --  182.3  millennia
    \_  scrypt                  --   79.9  millions of years
    \_  GPG                     --   21.2  millennia
    \_  AES-128                 --    2.6  decades
\_  example:  fosi suwe pawu mexi tuka kavu
~~~~

Show all supported patterns identifiers:
~~~~
z-tokens patterns -i
~~~~
~~~~
digits-base10:...
digits-base2:...
digits-base8:...
digits-base16:...
digits-base32-hex:...
digits-base32-rfc:...
digits-base64-url:...
digits-base64-rfc:...
digits-base58:...
digits-base62:...
digits-bech32:...
digits-z85:...
ascii-lower:...
ascii-upper:...
ascii-mixed:...
ascii-symbols:...
ascii-any:...
cv-lower:...
cv-upper:...
cv-mixed:...
cv-plus-a:...
cv-plus-b:...
cv-plus-c:...
proquint-lower:...
proquint-upper:...
koremutake-a:...
koremutake-b:...
mnemonic:...
bip0039:...
skey:...
pgp:...
eff-large:...
eff-short:...
eff-unique:...
nato:...
uuid-v4
ip-127
ip-10
ip-172
ip-192
ip-mac
bytes-hex:...
timestamp-date-time
timestamp-date
timestamp-time
timestamp-sec
timestamp-sec-hex
timestamp-nano
timestamp-nano-hex
timestamp-flake
timestamp-flake-hex
flake:...
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

Use the latest preview release:
~~~~
cargo install --bins --tag preview --git https://github.com/volution/z-tokens --force
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
    https://github.com/volution/z-tokens/releases/download/preview/z-tokens--linux--x86_64--v0.3.0--preview
~~~~
~~~~
curl -s -L -f -S \
    -o /tmp/z-tokens.asc \
    https://github.com/volution/z-tokens/releases/download/preview/z-tokens--linux--x86_64--v0.3.0--preview.asc
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
* tool          : z-tokens
* version       : 0.3.0
* executable    : /usr/local/bin/z-tokens
* executable-0  : z-tokens
* build target  : release
* build number  : 10166, 2023-01-06-03-46-34
* code & issues : https://github.com/volution/z-tokens
* sources git   : 978bdd53cae95f275d78ce60c5ab41b41af24933
* sources hash  : 0662003edbb1109000514694ea39e022
* uname node    : linux
* uname system  : Linux, 6.0.10-1-default, x86_64
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

