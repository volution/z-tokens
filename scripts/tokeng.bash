#!/bin/bash

set -e -E -u -o pipefail -o noclobber -o noglob +o braceexpand || exit 1
trap 'printf "[ee] failed: %s\n" "${BASH_COMMAND}" >&2' ERR || exit 1


_root="$( readlink -e -- "$( dirname -- "$( readlink -e -- "${0}" )" )/.." )"
_scripts="${_root}/scripts"
_sources="${_root}/sources"


_python_arguments=(
		-u # unbuffered `stdin` and `stdout`
		-O -O # optimizations enabled
		-B # disable writing `*.py[oc]`
		-E # ignore `PYTHON*` environment variables
		-S # disable `sys.path` manipulation
		-s # disable user-site
		-R # hash randomization
)

_python_environment=(
		TERM="${TERM}"
		TMPDIR="${TMPDIR:-/tmp}"
)

_python_exec=(
		env -i "${_python_environment[@]}"
		"${_scripts}/python" "${_python_arguments[@]}"
)


if test "${#}" -eq 0 ; then
	exec "${_python_exec[@]}" "${_sources}/tokeng.py"
else
	exec "${_python_exec[@]}" "${_sources}/tokeng.py" "${@}"
fi

exit 1
