#!/usr/bin/env bash
# @name command-aliases
# @version 0.0.1
# @description Sample application
# @example Download command help $ download -h
# @example Download command example $ download [OPTIONS] SOURCE [TARGET]
# @example Upload command help $ upload -h
# @example Upload command example $ upload [OPTIONS] SOURCE

# @cmd Download a file
# @describe Every '@cmd' tag before a function defines an individual command
# @describe that can be called independently from the terminal.
# @desceibe
# @describe Some times the name of this command is too long, or there other
# @describe ways to describe the same command. When that happens, you can
# @describe use the '@alias' tag and provide a different way of calling
# @describe this command.
# @describe
# @describe Another good use-case of the `@alias` tag is for deprecating
# @describe commands.
# @alias d
# @alias down
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
# @example Download a file from the internet $ download example.com
# @example Download a file and use a different alias $ down example.com
download() {
	echo "${rargs_input[*]}"
}

# @cmd Upload a file
# @alias u
# @alias push
# @arg source! URL to download from
# @option -u --user Username to use for logging in
# @option -p --password Password to use for logging in
# @example Upload a file to the internet $ upload example.com
# @example Upload a file and use a different alias $ push example.com
upload() {
	echo "${rargs_input[*]}"
}
