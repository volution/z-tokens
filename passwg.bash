#!/bin/bash

set -e -u -o pipefail || exit 1

if test "${#}" -eq 0
then
	exec python2 ./passwg.py
else
	exec python2 ./passwg.py "${@}"
fi

exit 1
