#!/usr/bin/env bash

# @name commands
# @description Commands example
# @version 0.1.0

# @cmd Download a file
# @alias d
# @flag -f --force Overwrite existing files
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @example $ example.com
# @example $ example.com ./output -f
download() {
  # shellcheck disable=SC2154
  echo "${rargc[@]}"
}

# @cmd Upload a file
# @alias u
# @option -u --user! Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
  # shellcheck disable=SC2154
  echo "${rargc[@]}"
}
