#!/usr/bin/env bash
# @name dependencies
# @version 0.0.1
# @description Sample application that requires dependencies

# @cmd Download a file
# @alias d
# @alias down
# @dep git,curl,shmurl
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
download() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

# @cmd Upload a file
# @alias u
# @dep mini-docker install with \e[32mgem install mini-docker\e[0m\
# @dep docker visit https://docker.com for more information
# @arg source! URL to download from
# @option -u --user Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

