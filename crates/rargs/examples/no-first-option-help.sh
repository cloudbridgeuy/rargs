#!/usr/bin/env bash
# @name First option help
# @version 0.0.1
# @description Script to test if the 'first-option-help' rule is working.
# @author @cloudbridgeuy
#

# @flag -v --verbose Verbose mode
root() {
	echo "Hello, World!"
}

# @cmd Download a file
# @alias d
# @alias down
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
# @example Download a file from the internet $ download example.com
# @example Download a file from the internet and force save it to ./output $ download example.com ./output -f
download() {
	echo "${rargs_input[*]}"
}

# @cmd Upload a file
# @alias u
# @arg source! URL to download from
# @option -u --user Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
	echo "${rargs_input[*]}"
}
