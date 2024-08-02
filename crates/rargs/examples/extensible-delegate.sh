#!/usr/bin/env bash
# @name extensible
# @description Extensible delegate command example using git
# @version 0.0.1
# @example Delegate a command that will be called with all the arguments $ log [GIT_OPTIONS]

# @any <EXTERNAL_COMMAND> External command to run
root() {
	echo "Here you would call the following command"
	# shellcheck disable=SC2068
	echo "  git" ${rargs_other_args[@]}
}

# @cmd Download a file
# @alias d
# @arg source! <SOURCE> File to download
download() {
	echo "Download"
	echo "${rargs_input[*]}"
}

# @cmd Upload a file
# @alias u
# @arg source! <SOURCE> File to upload
upload() {
	echo "Upload"
	echo "${rargs_input[*]}"
}
