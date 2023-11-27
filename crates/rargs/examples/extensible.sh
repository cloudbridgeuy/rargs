#!/usr/bin/env bash
# @name extensible
# @description Sample application that can be externally extended
# @version 0.0.1
# @example Run a command found in path prefixed with "extensible-" $ example [OPTIONS] [ARGS]

# @any <EXTERNAL_COMMAND> External command to run
root() {
	echo "Here you would call the following command"
	echo "  external-${rargs_other_args[0]}" "${rargs_other_args[@]:1}"
}

# @cmd Download a file
# @alias d
# @arg source! <SOURCE> File to download
download() {
	echo "Download"
	inspect_args
}

# @cmd Upload a file
# @alias u
# @arg source! <SOURCE> File to upload
upload() {
	echo "Upload"
	inspect_args
}
