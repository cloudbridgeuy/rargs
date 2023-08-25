#!/usr/bin/env bash

echo "Hello from the other side!!!"
echo "Your editor of choice is ${EDITOR:-}"

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
  echo "Downloading ${args["source"]} to ${args["target"]}"
  inspect_args
}

# @cmd Upload a file
# @alias u
# @option -u --user! Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
  # shellcheck disable=SC2154
  echo "Uploading using ${args["user"]}:${args["password"]}"
  inspect_args
}
