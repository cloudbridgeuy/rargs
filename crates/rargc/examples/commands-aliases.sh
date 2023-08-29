#!/usr/bin/env bash
# @name command-aliases
# @version 0.0.1
# @description Sample application
# @example Download command help $ download -h
# @example Download command example $ download [OPTIONS] SOURCE [TARGET]
# @example Upload command help $ upload -h
# @example Upload command example $ upload [OPTIONS] SOURCE

# @cmd Download a file
# @alias d
# @alias down
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
# @example Download a file from the internet $ download example.com
# @example Download a file and use a different alias $ down example.com
download() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
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
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

