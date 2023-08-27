#!/usr/bin/env bash

declare -A args=()

# @name args
# @description Commands with arguments example
# @version 0.1.0

# @cmd Download a file
# @alias d
# @arg source URL to download.
download() {
  echo "Downloading ${args["source"]} to ${args["target"]}"
  inspect_args
}

# @cmd Upload a file
# @alias u
# @arg source URL to download.
upload() {
  echo "Uploading using ${args["user"]}:${args["password"]}"
  inspect_args
}
