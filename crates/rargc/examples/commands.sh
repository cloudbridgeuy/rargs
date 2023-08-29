#!/usr/bin/env bash
# @name commands
# @version 0.0.1
# @description Sample application
# @example Download command help $ download -h
# @example Download command example $ download [OPTIONS] SOURCE [TARGET]
# @example Upload command help $ upload -h
# @example Upload command example $ upload [OPTIONS] SOURCE

# @cmd Download a file
# @alias d ddd
# @alias down
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
# @example Download a file from the internet $ download example.com
# @example Download a file from the internet and force save it to ./output $ download example.com ./output -f
download() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

# @cmd Upload a file
# @alias u
# @arg source! URL to download from
# @option -u --user Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

