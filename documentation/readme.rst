
#########################
tokeng -- Token Generator
#########################


About
=====

This is a small Python application (more accurate a quick script) to randomly generate various tokens like:

 * passwords, with various patterns ranging from easy-to-remember lower-case only, up to complete random characters, etc.;
 * "hex-tokens", similar to UUID's, of various sizes from 8 bits up to 512 bits, including a format which resembles UUID;
 * pins, MAC's, IP address and ports, etc.;


Security
========

**I personally use this tool for all my passwords (both for personal and work use), therefore I trust it fully.**

Internally the code uses Python's `random.SystemRandom <https://docs.python.org/2/library/random.html#random.SystemRandom>`__, which (hopefully) implies that the generated passwords are non-guessable (except by brute-force).

However like with any open-source software, and based on the implied risks, the user should always carefully analyze the source code.


Installation
============

Just copy the `sources` and `scripts` folders somewhere (like for example `/opt/token-generator-git`), and then symlink the `scripts/tokeng` to a folder that is inside your `PATH`.

For example: ::

  git clone -b development https://github.com/cipriancraciun/token-generator.git /tmp/token-generator
  ln -s -T /tmp/token-generator/scripts/tokeng ~/.bin/tokeng


Examples
========

Show the (quick) help: ::

    ./scripts/tokeng help


Generate a single password (ready to be copy-pasted from the terminal) with the pattern `p-aa-4`: ::

    ./scripts/tokeng one p-aa-4

Generate a single password with the pattern `p-aa-4`, and pipe it to `xcrip` in order to copy it into the clipboard: ::

    ./scripts/tokeng one-n p-aa-2 | xclip -in -selection clipboard

.. note::
    The difference between the `one` and `one-n` commands is that the later doesn't output a new line at the end of the password, thus it is suitable for piping it to tools that won't strip the newline.


Generate a couple of passwords (formatted for easy reading) with the pattern `p-aa-4`: ::

    ./scripts/tokeng generate p-aa-4 16

Generate a couple of passwords (formatted for copy-paste) with the pattern `p-aa-4`: ::

    ./scripts/tokeng generate p-aa-4 16 f


Show all the supported patterns (formatted for easy reading): ::

    ./scripts/tokeng patterns

    | d-04             |   4 ( 10.3 bits) | 6462
    | d-08             |   8 ( 20.7 bits) | 1534-4561
    | d-12             |  12 ( 31.0 bits) | 3512-2664-2664
    | d-16             |  16 ( 41.4 bits) | 3153-6145-2156-4254
    | d-20             |  20 ( 51.7 bits) | 3561-4533-5226-4132-5132
    | d-24             |  24 ( 62.0 bits) | 5614-2216-5464-3333-6262-2341
    | d-28             |  28 ( 72.4 bits) | 2366-2632-6345-4523-4564-1436-3156
    | d-32             |  32 ( 82.7 bits) | 2344-2331-4654-5521-3264-1662-2451-4333
    | d-48             |  48 (124.1 bits) | 1555-4465-3523-6565-2541-2656-4522-5 ...
    | d-64             |  64 (165.4 bits) | 1613-4312-6312-3316-3322-4221-5223-4 ...
    | ip-address-1     |   1 (  8.0 bits) | 251
    | ip-address-127   |   0 (  0.0 bits) | 127.92.57.144
    | ip-address-2     |   2 ( 16.0 bits) | 92.73
    | ip-address-3     |   3 ( 24.0 bits) | 175.238.177
    | ip-address-4     |   4 ( 32.0 bits) | 67.42.157.156
    | ip-port-a        |   1 ( 12.0 bits) | 52854
    | ip-port-b        |   1 ( 14.0 bits) | 60123
    | mac              |  12 ( 48.0 bits) | 27:a9:83:74:ee:e8
    | mac-xen          |   6 ( 24.0 bits) | 00:50:56:f4:d3:70
    | p-aa-1           |   4 ( 13.4 bits) | nixi
    | p-aa-2           |   8 ( 26.9 bits) | sela-nopu
    | p-aa-3           |  12 ( 40.3 bits) | velo-sipa-ceyu
    | p-aa-4           |  16 ( 53.7 bits) | faqu-coku-hiyu-kave
    | p-aa-5           |  20 ( 67.1 bits) | wubi-gola-qide-sata-qimu
    | p-aa-6           |  24 ( 80.6 bits) | nata-fesu-rayu-somi-roha-japi
    | p-aa-7           |  28 ( 94.0 bits) | qoje-poho-zigo-tolu-yuto-rito-veme
    | p-aa-8           |  32 (107.4 bits) | wiyu-qica-xopa-reru-feku-kaxu-lobu-bixi
    | p-ab-1           |   4 ( 17.4 bits) | cOcU
    | p-ab-2           |   8 ( 34.9 bits) | PIcE-xomo
    | p-ab-3           |  12 ( 52.3 bits) | bIka-BumE-LIpu
    | p-ab-4           |  16 ( 69.7 bits) | nAsi-peDU-CUYU-cIYi
    | p-ab-5           |  20 ( 87.1 bits) | QAsA-rOpA-rIpE-daSU-vucO
    | p-ab-6           |  24 (104.6 bits) | DefI-golu-nERe-bIMu-DITU-yeFe
    | p-ab-7           |  28 (122.0 bits) | CIXe-daqA-Cayi-ceQE-WEWi-hIbE-tuZO
    | p-ab-8           |  32 (139.4 bits) | dukO-GOWI-SaRA-meFI-MOfU-cElU-Geri-KIji
    | p-ba-2           |   8 ( 26.7 bits) | zagu-3984
    | p-ba-3           |  12 ( 40.1 bits) | hohi-poyi-4176
    | p-ba-4           |  16 ( 53.6 bits) | baqe-biro-gura-2246
    | p-bb-2           |   8 ( 30.7 bits) | MUdI-6301
    | p-bb-3           |  12 ( 48.1 bits) | BaHi-DiMA-0758
    | p-bb-4           |  16 ( 65.6 bits) | nIqe-lASi-PAVa-4916
    | p-bc-2           |   8 ( 26.8 bits) | bece-GI-07
    | p-bc-3           |  12 ( 40.2 bits) | wifo-vele-DE-58
    | p-bc-4           |  16 ( 53.6 bits) | zecu-xote-velu-QO-60
    | p-bd-2           |   8 ( 28.5 bits) | wumi-HE-3[
    | p-bd-3           |  12 ( 41.9 bits) | digo-qimu-ZI-0"
    | p-bd-4           |  16 ( 55.3 bits) | zapo-zuqo-wofu-MU-4+
    | p-da-1           |   7 ( 25.1 bits) | cutu 10^
    | p-da-2           |  14 ( 50.1 bits) | muji 11. jeva 48{
    | p-da-3           |  21 ( 75.2 bits) | gopi 85} jupu 44< hegu 21#
    | p-da-4           |  28 (100.3 bits) | qono 73+ tedu 29_ qoha 16: wuco 14%
    | p-db-1           |   7 ( 29.1 bits) | MayE 42{
    | p-db-2           |  14 ( 58.1 bits) | ZElu 43! jiMe 47{
    | p-db-3           |  21 ( 87.2 bits) | suNa 54? TIXU 48` QuDO 87^
    | p-db-4           |  28 (116.3 bits) | labe 43} BAVI 23] xaZU 48: vaje 21|
    | r-04             |   4 ( 26.2 bits) | {%-8
    | r-08             |   8 ( 52.4 bits) | 2.0] Ey<|
    | r-16             |  16 (104.9 bits) | nm/Q R(y# ]&Dd /x[p
    | r-24             |  24 (157.3 bits) | JjTx y%J$ ]4~G Qi5j |},< $h&m
    | r-32             |  32 (209.7 bits) | q\wi M1=y +R}5 (j/{ xx1y iKpL }ffK ZHEE
    | r-48             |  48 (314.6 bits) | 9vrb msA} x#F? 2cpm 4~?# )BM6 6lU8 ? ...
    | r-64             |  64 (419.5 bits) | 3E%s t+C= 5#EC h/*W gZ.M YC"^ :)fh = ...
    | uuid             |  32 (128.0 bits) | 95b4840c-046e-7cd1-e4af-1e50ee8656a4
    | x-008            |   2 (  8.0 bits) | 1f
    | x-016            |   4 ( 16.0 bits) | f038
    | x-032            |   8 ( 32.0 bits) | c3266994
    | x-064            |  16 ( 64.0 bits) | ad33f518a8ceff74
    | x-096            |  24 ( 96.0 bits) | 034da39173f71ea302b197c2
    | x-128            |  32 (128.0 bits) | 466213d4a2c1fce5d4bd0185fce178e3
    | x-160            |  40 (160.0 bits) | 045a153a13ce4c4659e0c174d0720b16130cec6d
    | x-192            |  48 (192.0 bits) | 1a6608d5947b844ebfdbc8b43ee685d5051f ...
    | x-224            |  56 (224.0 bits) | 77c75400f9c9cfe34bc4a3addebac7051684 ...
    | x-256            |  64 (256.0 bits) | 9677948a8ecc9efa69bf47ee07cd020eb091 ...
    | x-384            |  96 (384.0 bits) | 42879765dc16018d472e7bdc47c055281622 ...
    | x-512            | 128 (512.0 bits) | 3536b9dfda0b1946fa50e6958f00f87e65a2 ...


Notice
======

**Short version: the code is licensed under GPL 3 or later.**

For details about the copyright and licensing, please consult the `notice <./documentation/licensing/notice.txt>`__ file in the ``documentation/licensing`` folder.

