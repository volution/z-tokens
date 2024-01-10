
z-tokens -- Random tokens generation and related tools

      + about; status;
      + documentation; examples;
      + install;
      + licensing; SBOM;
      + chat on Discord; discuss on GitHub; email author;
      + source code at https://github.com/volution/z-tokens

-------------------------------------------------------------------------------

About

    Welcome to the z-tokens project, available at https://github.com/volution/
    z-tokens.

As the title says, z-tokens is a Rust-based tool for generating tokens
(especially passwords and other entropy-sensitive tokens) and other related
utilities. (Although, at the moment it can only generate tokens, compute
hashes, and a highly experimental encryption / encoding tool.)

This project (more exactly the token generation tool) is at its third
iteration, and has its roots in a 2008 CommonLisp script that had the same
purpose, and follows the second Python-based iteration. (Although the
Rust-based implementation is quite more complex than the previous iterations.)

Needless to say, I've used this tool for the last 10+ years for all my password
generation, thus I find it critical for my own use-cases. I just hope others
also find some use in it.

In terms of types of generated tokens, here are a few of them:

  * passwords -- from easily pronounceable (see the cvs:* and proquint:* ones)
    complete random ASCII dumps;
  * pins -- numeric tokens with various levels of entropy (see the
    digits-base10:* ones);
  * raw tokens -- in various encodings (such as digits-base64-url:*,
    digits-base32-hex:*, digits-z85:*, digits-bech32:*, etc.); (please note
    that such tokens just use the encoding character set, and don't necessarily
    decode correctly in one of those encodings;)
  * bytes -- hex encoded random bytes (see bytes-hex:*);
  * IP and MAC addresses (see ip-10, ip-172, ip-192, and ip-mac);
  * UUIDv4 (see uuid-v4);
  * timestamps in various representations (see timestamp-*); (not for security
    purposes, as they have almost 0 entropy;)
  * time sorted tokens with various levels of entropy (see the flake:* ones);
  * seed phrases (see the mnemonic:*, eff-*:*, or bip0039:* ones);

Other related tools part of this project are:

  * z-tokens hash -- !! EXPERIMENTAL !! -- hash strings, stdin or files with
    various hashes (SHA1/SHA2/SHA3 families, Blake2 and Blake3, MD5, various
    non cryptographic hashes and CRC's, etc.);
  * z-tokens exchange -- !! EXPERIMENTAL !! -- encrypt/decrypt (with one or
    more X25519 PKI keys, pins, passwords, and even via the SSH agent) or armor
    /dearmor (similar to Base64 but with more features);
  * z-tokens encode -- (not yet implemented) -- encode / decode strings, stdin
    or files to / from various encodings (Base*, Bech32, Z85, etc.);
  * z-tokens secrets -- (not yet implemented) -- storing, sharing and using
    security sensitive data in operational and development scripts; (for the
    gist of the idea see this document;)
  * z-tokens oracles -- (not yet implemented) -- using various techniques to
    implement useful "oracles"; (see this article;)

-------------------------------------------------------------------------------

Status

The Rust code is quite fresh and thus not thoroughly tested and reviewed.

However, it relies on solid Rust libraries (like rand) and it doesn't do much
(like touching the file-system or talking to the network), thus it shouldn't
break anything.

The only critical code at the moment, --- which could have major security
implications by weakening the generated tokens --- is the random token
generation, especially the selection from the pattern character sets. Although
I haven't thoroughly reviewed this, it's simple and just delegates to the rand
library for the actual random number handling.

The generated patterns should provide the stated entropy bits, as per z-tokens
patterns, because these values are computed based on the actual patterns. Thus
even if I missed a character in a given pattern character set, if your token
matches your entropy requirements, then it should be safe enough (provided the
random selection doesn't have issues).

-------------------------------------------------------------------------------

Documentation

Besides what is available by running z-tokens help and this document
(especially the examples section) there is no other documentation at the
moment.

That being said, just run the following and start experimenting with the
commands. (If there is need for documentation, besides the frugally -h for each
command, I have failed in one of the mandatory requirements, that of being
"simple to use".)

Get some help:

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

For how to install see the dedicated section.

-------------------------------------------------------------------------------

Examples

How to generate passwords and other tokens

Generate a few consonant-vowel passwords, and display for transcribing (do not
copy-paste the spaces) (all the commands are almost equivalent):

z-tokens generate
z-tokens generate -p cvs:4
z-tokens generate -p cvs:4 -C
z-tokens generate -p cvs:4 -C -c 20

hogi ruve suna dopi
cati regi lihi zuro
....

Generate a few consonant-vowel passwords, and output for copy-pasting (again
all the commands are equivalent):

z-tokens g
z-tokens g cvs:4
z-tokens g cvs:4
z-tokens g cvs:4 -c 1

z-tokens generate -C -c 1
z-tokens generate -p cvs:4 -C -c 1
z-tokens generate -p cvs:4 -C -c 1

lupoferocirejipa

Generate one UUIDv4 token, and pipe it to another process:

z-tokens g uuid-v4 | clipboard copy
z-tokens generate -p uuid-v4 -C -c 1 | clipboard copy

How to list available patters

The z-tokens patterns command usually outputs a table with the following
format:

  * the first column is the pattern identifier (to be used with z-tokens g
    {pattern} or z-tokens generate -p {pattern} ...);
  * the b column shows the bits of entropy a given token pattern has; i.e. the
    amount of work for brute-force attack; if the number doesn't have a . (not
    even .0) it implies that exact number of bits;
  * the c column shows the token length with spaces or separators (or without
    mandatory separators if -C is used);
  * the last column is an example of such a token; you can safely use it, with
    or without spaces or separators as required (although without is suggested,
    unless these separators are mandatory);

Show a selection of supported patterns (mainly those up to 40 characters in
length):

z-tokens patterns

##  NOTE:  This output is truncated!
::  digits-base10:10        :   33.2 b :   10 c ::    2607 2034 43
::  digits-base2:12         :   12  =b :   12 c ::    10100011 0011
::  digits-base8:10         :   30  =b :   10 c ::    3304 7551 40
::  digits-base16:10        :   40  =b :   10 c ::    9cfa a090 0a
::  digits-base32-hex:10    :   50  =b :   10 c ::    ovul 08jj 7h
::  digits-base32-rfc:10    :   50  =b :   10 c ::    vqsp 7ekh a2
::  digits-base64-url:10    :   60  =b :   10 c ::    0rcg 6sCj 4O
::  digits-base64-rfc:10    :   60  =b :   10 c ::    qgrV Eieg Gf
::  digits-base58:10        :   58.5 b :   10 c ::    6NLe 7noy ZZ
::  digits-base62:10        :   59.5 b :   10 c ::    SlkQ XEBd wv
::  digits-bech32:10        :   50  =b :   10 c ::    zedu z5xp 0r
::  digits-z85:10           :   64.0 b :   10 c ::    6soh- qaITe
::  ascii-lower:10          :   47.0 b :   10 c ::    dkso nkju tt
::  ascii-mixed:10          :   57.0 b :   10 c ::    oRFd AGru hO
::  ascii-symbols:10        :   50  =b :   10 c ::    ]:*} '%}+ $>
::  ascii-any:10            :   65.5 b :   10 c ::    4vKw +J]O w?
::  cva-lower:3             :   40.2 b :   12 c ::    dago pimu maha
::  cva-mixed:3             :   52.2 b :   12 c ::    WaPE hUmO LaHE
::  cva-plus-a:3            :   40.1 b :   12 c ::    yeso picu 6255
::  cva-plus-b:3            :   40.2 b :   12 c ::    nuxi fama FO92
::  cva-plus-c:3            :   41.8 b :   12 c ::    xino xeme WI7+
::  cvs-lower:3             :   37.9 b :   12 c ::    jitu lefe dufo
::  cvs-mixed:3             :   49.9 b :   12 c ::    vIdU nUdO rETa
::  cvs-plus-a:3            :   38.5 b :   12 c ::    soje mame 9380
::  cvs-plus-b:3            :   38.2 b :   12 c ::    bini felu TU98
::  cvs-plus-c:3            :   39.9 b :   12 c ::    tina nudo MI2`
::  proquint-lower:2        :   32  =b :   10 c ::    purig gural
::  koremutake-a:2          :   28  =b :   10 c ::    mesto stori
::  koremutake-b:2          :   42  =b :   13 c ::    gepedi roprumu
::  mnemonic:2              :   21.3 b :   10 c ::    floor minus
::  bip0039:2               :   22  =b :   13 c ::    inquiry muffin
::  skey:3                  :   33  =b :   12 c ::    gone lien sour
::  pgp:1                   :   16  =b :   16 c ::    blockade universe
::  eff-large:2             :   25.8 b :   11 c ::    sly dividend
::  eff-short:2             :   20.6 b :   10 c ::    ride shaky
::  eff-unique:2            :   20.6 b :   19 c ::    ergonomic vanquished
::  pets-medium-1           :   10.0 b :   10 c ::    wasp
::  pets-medium-2           :   20.2 b :   13 c ::    plucky-dragon
::  pets-medium-3           :   30.9 b :   25 c ::    reputably-exceeding-pewee
::  pets-medium-4           :   36.9 b :   21 c ::    loudly-brave-tin-grub
::  pets-small-2            :   17.6 b :   14 c ::    content-marmot
::  pets-small-3            :   25.6 b :   23 c ::    verbally-noble-basilisk
::  pets-small-4            :   31.6 b :   28 c ::    cleanly-advanced-russet-mink
::  nato:2                  :   10.3 b :   11 c ::    kilo romeo
::  bytes-hex:5             :   40  =b :   10 c ::    bc4c68b98a
::  token-hex-48            :   48  =b :   12 c ::    a45cd37217be
::  token-hex-64            :   64  =b :   16 c ::    a7624d9e2ebcc557
::  token-hex-96            :   96  =b :   24 c ::    633f32ab8f2428200396d128
::  token-hex-128           :  128  =b :   32 c ::    554fbc4d1e9a402bc433c1330b6cda97
::  token-hex-256           :  256  =b :   65 c ::    3496bd413a615db6be25724d23618259-5e481bb9854a06afc644a3aed60f98ba
::  token-hex-512           :  512  =b :  131 c ::    07ef2a758f48ff61adc58fe01001f264-bc6c29d7fa066c40690b640fcbda433d-e6cdfbef9a1dee [...]
::  uuid-v4                 :  122  =b :   36 c ::    72ca5b32-c82d-4369-a5f1-acbb44c8a35b
::  ip-127                  :   23.9 b :   15 c ::    127.207.105.152
::  ip-10                   :   23.9 b :   13 c ::    10.34.199.232
::  ip-172                  :   20.8 b :   12 c ::    172.24.88.33
::  ip-192                  :   15.9 b :   13 c ::    192.168.5.146
::  ip-mac                  :   40  =b :   17 c ::    02:0a:49:80:33:98
::  timestamp-date-time     :    0  =b :   19 c ::    2024-01-08-16-54-37
::  timestamp-date          :    0  =b :   10 c ::    2024-01-08
::  timestamp-sec           :    0  =b :   10 c ::    1704732877
::  timestamp-sec-hex       :    0  =b :   10 c ::    00659c28cd
::  timestamp-nano          :    0  =b :   19 c ::    1704732877803775245
::  timestamp-nano-hex      :    0  =b :   18 c ::    0017a86d85326c6cdf
::  flake:32                :   32  =b :   17 c ::    2d2ee54d-aea4234d

Show only patterns that have between 64 and 90 bits of entropy:

z-tokens patterns -b 64 -B 90

##  NOTE:  This output is truncated!
::  digits-base10:20        :   66.4 b :   24 c ::    1813 7864 2596 7265 4317
::  digits-base8:22         :   66  =b :   27 c ::    0413 2005 2622 3341 0730 24
::  digits-base16:16        :   64  =b :   19 c ::    cba6 395c c7cb 8d04
::  digits-base32-hex:14    :   70  =b :   17 c ::    5v7k fb2q ret1 di
::  digits-base32-rfc:14    :   70  =b :   17 c ::    wwcr fwyd f5d4 jf
::  digits-base64-url:12    :   72  =b :   14 c ::    kwD8 EEIg 5Zlv
::  digits-base64-rfc:12    :   72  =b :   14 c ::    pdr1 Tx2C cPbq
::  digits-base58:12        :   70.2 b :   14 c ::    3dDa TXS3 v9cM
::  digits-base62:12        :   71.4 b :   14 c ::    2RqQ Jys3 ss1h
::  digits-bech32:14        :   70  =b :   17 c ::    d4rq ajdj pkcj p8
::  digits-z85:10           :   64.0 b :   11 c ::    0R>6l >PAw-
::  ascii-lower:14          :   65.8 b :   17 c ::    esbz grfr bdnd dv
::  ascii-mixed:12          :   68.4 b :   14 c ::    uRJv CdSi EieN
::  ascii-symbols:14        :   70  =b :   17 c ::    /&:$ :)_] &?%, $&
::  ascii-any:10            :   65.5 b :   12 c ::    q@7Q |Jun S=
::  cva-lower:5             :   67.1 b :   24 c ::    feqi sebo tana quvi gexi
::  cva-mixed:4             :   69.7 b :   19 c ::    gafi Ramu Jayu bUra
::  cva-plus-a:5            :   67.0 b :   24 c ::    doqe gemu vaca romo 0407
::  cva-plus-b:5            :   67.0 b :   24 c ::    depu tofu fiqe jugi TI40
::  cva-plus-c:5            :   68.7 b :   24 c ::    pixe zaya jiri giso QA6(
::  cvs-lower:6             :   75.8 b :   29 c ::    fibe depe jebu hizo vipa duzi
::  cvs-mixed:4             :   66.5 b :   19 c ::    Lefu LUsU veJI dEZe
::  cvs-plus-a:6            :   76.5 b :   29 c ::    gota zadu nube toze tovu 0989
::  cvs-plus-b:6            :   76.1 b :   29 c ::    reda pire jiri hoju gido NE39
::  cvs-plus-c:5            :   65.2 b :   24 c ::    jimu mehi deza cuhu RE3`
::  proquint-lower:4        :   64  =b :   23 c ::    rosuj juduj movuj roriz
::  koremutake-a:5          :   70  =b :   31 c ::    brana brudro dryfi studro prejo
::  koremutake-b:4          :   84  =b :   30 c ::    brufrapa mygryvo mosyla batuhy
::  mnemonic:6              :   64.0 b :   36 c ::    menu hilton front soda harbor quebec
::  bip0039:6               :   66  =b :   44 c ::    grocery oval scatter execute broccoli anchor
::  skey:6                  :   66  =b :   27 c ::    glow hear sal pew jeff nice
::  pgp:4                   :   64  =b :   64 c ::    spearhead liberty obtuse apollo retouch torpedo waffle pocketful
::  eff-large:5             :   64.6 b :   41 c ::    bullion elastic viselike avenue antiviral
::  eff-short:7             :   72.3 b :   37 c ::    talon lint skid doll sting ninth walk
::  eff-unique:7            :   72.3 b :   59 c ::    vehicle grape eliminator pretzel dyslexia alphabet hedgehog
::  nato:13                 :   67.2 b :   81 c ::    tango niner oscar alfa juliett charlie xray sierra mike yankee uniform bravo xra [...]
::  bytes-hex:8             :   64  =b :   16 c ::    080720ee13ea3842
::  token-hex-64            :   64  =b :   16 c ::    da6738bf39f35716
::  flake:64                :   64  =b :   26 c ::    2d2ee589-9bd1f65e-f10cb875

Show only patterns that have all types of characters (lower and upper letters,
digits, and symbols):

z-tokens patterns -A

##  NOTE:  This output is truncated!
::  digits-base64-url:4     :   24  =b :    4 c ::    cL4q
::  digits-base64-rfc:6     :   36  =b :    6 c ::    QcGI um
::  digits-z85:5            :   32.0 b :    5 c ::    @VYQL
::  ascii-any:4             :   26.2 b :    4 c ::    w,nn
::  cva-plus-c:2            :   28.4 b :    8 c ::    yiva VE4?
::  cvs-plus-c:2            :   27.2 b :    8 c ::    lolu CU4&

Show the shortest pattern usable for authentication purposes:

z-tokens patterns --label password --for-authentication --shortest

::  digits-base10:10        :   33.2 b :   12 c ::    8543 1427 28
::  digits-base16:8         :   32  =b :    9 c ::    0f84 6ef7
::  digits-base32-hex:8     :   40  =b :    9 c ::    58tj pqk2
::  digits-base32-rfc:8     :   40  =b :    9 c ::    jeig 5tnh
::  digits-base64-url:6     :   36  =b :    7 c ::    qAsM G8
::  digits-base64-rfc:6     :   36  =b :    7 c ::    6Rjm Xq
::  digits-base58:6         :   35.1 b :    7 c ::    gMSG Vt
::  digits-base62:6         :   35.7 b :    7 c ::    PKzj Uu
::  digits-bech32:8         :   40  =b :    9 c ::    jppl c4we
::  digits-z85:5            :   32.0 b :    5 c ::    u)4wV
::  ascii-lower:8           :   37.6 b :    9 c ::    oklv iysm
::  ascii-mixed:6           :   34.2 b :    7 c ::    IGIn Pj
::  ascii-any:6             :   39.3 b :    7 c ::    n~QN ~a
::  cva-lower:3             :   40.2 b :   14 c ::    vebu guhi mixo
::  cva-mixed:2             :   34.8 b :    9 c ::    JEro kEfo
::  cva-plus-a:3            :   40.1 b :   14 c ::    yahe zage 9120
::  cva-plus-b:3            :   40.2 b :   14 c ::    nobe zovi SO24
::  cva-plus-c:3            :   41.8 b :   14 c ::    beru covu XA1~
::  cvs-lower:3             :   37.9 b :   14 c ::    page jave bepa
::  cvs-mixed:2             :   33.2 b :    9 c ::    NoGa LusE
::  cvs-plus-a:3            :   38.5 b :   14 c ::    pete pusu 3082
::  cvs-plus-b:3            :   38.2 b :   14 c ::    zato zuci NU88
::  cvs-plus-c:3            :   39.9 b :   14 c ::    zogu cale BO1%
::  proquint-lower:2        :   32  =b :   11 c ::    rohop dazup
::  koremutake-a:3          :   42  =b :   17 c ::    frysti jami fysti
::  koremutake-b:2          :   42  =b :   16 c ::    rikabra metraste
::  bytes-hex:4             :   32  =b :    8 c ::    048d6f93
::  token-hex-64            :   64  =b :   16 c ::    eb2ce46f76fe28e3
::  token-hex-96            :   96  =b :   24 c ::    19e6c477c441483642c40b9e
::  token-hex-128           :  128  =b :   32 c ::    ac6dc428d8b9a8aab4f7cf5ac740b39f
::  uuid-v4                 :  122  =b :   36 c ::    cd86f08f-014d-4acf-8f20-6cfee673f7f2

z-tokens patterns --label passphrase --for-authentication --shortest

::  mnemonic:3              :   32.0 b :   20 c ::    bonus silicon client
::  bip0039:3               :   33  =b :   19 c ::    pattern limit until
::  skey:3                  :   33  =b :   14 c ::    stew dart gina
::  eff-large:3             :   38.7 b :   23 c ::    alfalfa stamp outspoken
::  eff-short:4             :   41.3 b :   21 c ::    cling aids aide widow
::  eff-unique:4            :   41.3 b :   37 c ::    judiciary upkeep lumberjack asparagus

Show all details about a certain pattern:

z-tokens patterns -p cvs:6 --show-all

**  ~~~~~~~~  cvs-lower:6
\_  aliases:  cvs:6
\_  labels:   cvs-lower cvs cv letters password pronounceable memorable
\_  bits:     75.8631
\_  length:   29  (with spaces)
\_  length:   24  (without spaces)
\_  characters:
    \_  letters:  24
    \_  l. upper: 0
    \_  l. lower: 24
    \_  digits:   0
    \_  symbols:  0
    \_  no space: 24
\_  usable for:
    \_  cryptography         !! NO !!      with    -52.14  bits of margin
    \_  authentication          OK         with    +43.86  bits of margin
    \_  archival storage     !! NO !!      with    -24.43  bits of margin
    \_  long term storage    !! NO !!      with     -0.98  bits of margin
    \_  short term storage      OK         with     +5.87  bits of margin
\_  bruteforce time:
    \_  MD4                     --   27.3  days
    \_  MD5                     --    1.6  months
    \_  SHA1                    --    5.1  months
    \_  SHA2-256                --   11.9  months
    \_  SHA3-256                --    4.3  years
    \_  PBKDF2-HMAC-MD5         --    4.7  centuries
    \_  PBKDF2-HMAC-SHA1        --    1.1  millennia
    \_  PBKDF2-HMAC-SHA256      --    2.5  millennia
    \_  PBKDF2-HMAC-SHA512      --    7.0  millennia
    \_  scrypt                  --    3.1  millions of years
    \_  GPG                     --    8.1  centuries
    \_  AES-128                 --   11.9  months
\_  example:  difa dana mine niho zunu reze


Show all supported patterns identifiers:

z-tokens patterns -i

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
cva-lower:...
cva-upper:...
cva-mixed:...
cva-plus-a:...
cva-plus-b:...
cva-plus-c:...
cvs-lower:...
cvs-upper:...
cvs-mixed:...
cvs-plus-a:...
cvs-plus-b:...
cvs-plus-c:...
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
pets-medium-1
pets-medium-2
pets-medium-3
pets-medium-4
pets-small-1
pets-small-2
pets-small-3
pets-small-4
nato:...
bytes-hex:...
token-hex-16
token-hex-24
token-hex-32
token-hex-48
token-hex-64
token-hex-96
token-hex-128
token-hex-256
token-hex-512
uuid-v4
ip-127
ip-10
ip-172
ip-192
ip-mac
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

-------------------------------------------------------------------------------

Installation

At the moment, due to the Rust cross-compilation hurdles, one can install
z-tokens by building from sources with cargo, or downloading the already built
Linux or OSX binaries.

Building from sources

At the moment z-tokens requires at least Rust 1.75 or later.

Use the latest development branch:

cargo install --bins --git https://github.com/volution/z-tokens --force

Use the latest preview release:

cargo install --bins --tag preview --git https://github.com/volution/z-tokens --force

It should build at least on the following platforms:

  * Linux:
      + x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl (i.e. Intel and AMD
        64 bits) (both with static and dynamic linking);
      + i686-unknown-linux-gnu, i686-unknown-linux-musl (i.e. Intel and AMD 32
        bits) (both with static and dynamic linking);
      + aarch64-unknown-linux-gnu, aarch64-unknown-linux-musl (i.e. ARM 64
        bits, like RaspberryPi 3-4-5) (both with static and dynamic linking);
      + armv7-unknown-linux-gnueabihf, armv7-unknown-linux-musleabihf (i.e.
        ARMv7 32 bits) (both with static and dynamic linking);
      + arm-unknown-linux-gnueabihf, arm-unknown-linux-musleabihf (i.e. ARMv6
        32 bits, like Raspberry Pi 1-2) (both with static and dynamic linking);
  * OSX: x86_64-apple-darwin, aarch64-apple-darwin;
  * Windows: x86_64-pc-windows-gnu;
  * Android: x86_64-linux-android, i686-linux-android, aarch64-linux-android,
    armv7-linux-androideabi; (i.e. to be used under Termux or similar, on a
    real Android device like a tablet or phone;)
  * FreeBSD: not tested, but I don't see why it shouldn't build;
  * OpenBSD: not tested, but I don't see why it shouldn't build;
  * WebAssembly: wasm32-wasi (i.e. truly portable on any system that is able to
    run a WebAssembly WASI runtime like wasmtime);

The build status was assessed by running cargo check --target ... and then
cargo build --target ... (cross compiling from Linux with the required tooling
installed). (Also, for all of these targets there are pre-built executables as
described in the next section.)

Where I have access, I also run the executables under the native OS, and so far
all of the above seem to have worked (at a certain point).

Downloading pre-built binaries

  * download the executable and (optional) signature (replace linux with darwin
    (for OSX), freebsd, openbsd, etc., and remove the glibc token, and use
    x86_64 or aarch64 matching your processor, and in case of Linux replace
    glibc with musl and possibly glibc--static and musl--static, there is even
    support for WebAssembly WASI):

curl -s -L -f -S \
    -o /tmp/z-tokens \
    https://github.com/volution/z-tokens/releases/download/preview/z-tokens--linux--x86_64--glibc--v0.4.0--preview

curl -s -L -f -S \
    -o /tmp/z-tokens.sig \
    https://github.com/volution/z-tokens/releases/download/preview/z-tokens--linux--x86_64--glibc--v0.4.0--preview.sig

  * (optional) verify the executable (the -P ... is the public key):

minisign \
    -V \
    -P 'RWTNU8euCgU67tydQTW8Obk/C/aOiRComoFnFpiTzPwjDc39BU4R3M4g' \
    -x /tmp/z-tokens.sig \
    -m /tmp/z-tokens

  * check that the output is`:

Signature and comment signature verified
Trusted comment: z-tokens--linux--x86_64--glibc--v0.4.0--preview

  * change the executable permissions:

chmod a=rx /tmp/z-tokens

  * copy the executable on the $PATH:

sudo cp /tmp/z-tokens /usr/local/bin/z-tokens

  * check that it works:

z-tokens --version

* tool          : z-tokens
* version       : 0.4.0
* executable    : /usr/local/bin/z-tokens
* executable-0  : z-tokens
* build target  : release
* build number  : 15721, 2024-01-08-18-09-22
* code & issues : https://github.com/volution/z-tokens
* sources git   : 9644b5af5f817a854523e4e837352641e5794c6a
* sources hash  : a8d6bc01f668ff3b1b7bae6bcbfddf49
* uname node    : my-computer
* uname system  : Linux, 6.6.6-6-default, x86_64

-------------------------------------------------------------------------------

Notice (copyright and licensing)

Notice -- short version

The code is licensed under GPL 3 or later.

Notice -- long version

For details about the copyright and licensing, please consult the notice.txt
file in the documentation/licensing folder.

If someone requires the sources and/or documentation to be released under a
different license, please email the authors, stating the licensing
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

