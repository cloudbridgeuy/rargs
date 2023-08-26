#!/usr/bin/env bash

echo "Hello from the other side!!!"
echo "Your editor of choice is ${EDITOR:-}"

# @name commands
# @description Commands example
# @version 0.1.0

# @cmd Download a file
# @alias d
# @arg source URL to download.
download() {
  # shellcheck disable=SC2154
  echo "Downloading ${args["source"]} to ${args["target"]}"
  inspect_args
}

# @cmd Upload a file
# @alias u
# @arg source URL to download.
upload() {
  # shellcheck disable=SC2154
  echo "Uploading using ${args["user"]}:${args["password"]}"
  inspect_args
}
