#!/bin/bash

set -e -u -o pipefail || exit 1

sources="$( dirname "$( readlink -e "${0}" )" )"

test -f "${sources}/tokeng.py"

if test "${#}" -eq 0
then
	exec "${sources}/python" -E -O -O -u "${sources}/tokeng.py"
else
	exec "${sources}/python" -E -O -O -u "${sources}/tokeng.py" "${@}"
fi

exit 1
