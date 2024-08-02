#!/usr/bin/env bash
# @name dependencies
# @version 0.0.1
# @description Sample application that requires dependencies
# @rule use-global-deps-for-root

# @cmd Download a file
# @alias d
# @alias down
# @dep foo,bar,baz install with \e[32mgem install foo bar baz\e[0m
# @dep git You can install git with \e[32mapt install git\e[0m
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
download() {
	echo "${rargs_input[*]}"
}

# @cmd Upload a file
# @alias u
# @dep docker visit https://docker.com for more information
# @dep foo
# @dep git You don't have git?
# @arg source! URL to download from
# @option -u --user Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
	echo "${rargs_input[*]}"
}

# @dep fail This is meant to fail
# @dep again Also this
root() {
	echo "Fallback to root command"
}
