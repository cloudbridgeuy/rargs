#!/usr/bin/env bash
# @name commands-nested
# @version 0.0.1
# @description Sample application with nested commands
# @example Directory command $ dir
# @example Directory help command $ dir -h
# @example Directory file command $ file -h
# @example List directory sub-command $ dir list
# @example List directory sub-command help $ dir list -h

# @cmd Directory commands
# @alias d
# @sub ./commands-nested/dir.sh
# @option -v --verbose Verbose mode
dir() {
	# shellcheck disable=SC2068
	# shellcheck disable=SC2154
	"$sub" ${rargs_input[@]}
}

# @cmd File commands
# @alias f
# @sub ./commands-nested/file.sh
file() {
	# shellcheck disable=SC2068
	# shellcheck disable=SC2154
	"$sub" ${rargs_input[@]}
}
