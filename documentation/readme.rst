
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

Generate a single password (ready to be copy-pasted) with the pattern `p-aa-4`: ::

    ./scripts/tokeng p-aa-4

Generate a couple of passwords (formatted for easiy reading) with the pattern `p-aa-4`: ::

    ./scripts/tokeng generate p-aa-4 16

Generate a couple of passwords (formatted for copy-paste) with the pattern `p-aa-4`: ::

    ./scripts/tokeng generate p-aa-4 16 f

Show all the supported patterns: ::

    ./scripts/tokeng patterns


Notice
======

For details about the copyright and licensing, please consult the `notice <./documentation/licensing/notice.txt>`__ file in the ``documentation/licensing`` folder.  (In short the code is licensed under GPL 3 or later.)
