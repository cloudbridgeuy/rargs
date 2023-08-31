#!/usr/bin/env bash
# @name commands-default-force
# @version 0.0.1
# @description Sample application that uses the default command option
# @example Run the default command $
# @example Print the global help $ --help
# @example Download something $ download something
# @example Upload something $ upload something
# @example Upload something using the default command $ something
# @default upload

# @cmd Upload a file
# @alias u
# @arg source! URL to download from
upload() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

# @cmd Download a file
# @alias d
# @arg source! URL to download from
download() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}
