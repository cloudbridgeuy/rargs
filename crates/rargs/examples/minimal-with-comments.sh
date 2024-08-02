#!/usr/bin/env bash
# @name minimal
# @version 0.0.1
# @description Sample minimal application without commands
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
# @example Download a file from the internet $ example.com
# @example Download a file from the internet and force save it to ./output $ example.com ./output -f

## This line comment will be added to the final script
# This one will not make it.
root() {
	echo "${rargs_input[*]}"
}

## Comments that begin with a double hash will be added to the final script
# Those that begin with a single hash will not.
## Comments that are included before a @cmd tag will be considered root comments, and be hoisted to
## the top of the script. If you want your comments to be available near the function definition,
## place them after the @cmd tag.
# @cmd Download a file
# @alias d
# @alias down
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
# @example Download a file from the internet $ download example.com
# @example Download a file from the internet and force save it to ./output $ download example.com ./output -f
## This comment will be placed on top of the function definition.
download() {
	echo "${rargs_input[*]}"
	bottom
}
