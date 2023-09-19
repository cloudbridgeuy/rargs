#!/usr/bin/env bash
# @name mix-lines
# @version 0.0.1
# @description Sample script that shows how lines between commands are handled
# @example Download command help $ download -h
# @example Download command example $ download [OPTIONS] SOURCE [TARGET]
# @example Upload command help $ upload -h
# @example Upload command example $ upload [OPTIONS] SOURCE
# @rule no-force-default

set +x

top() {
  echo top
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
  inspect_args
  bottom
}

middle() {
  echo middle
}

# @cmd Upload a file
# @alias u
# @arg source! URL to download from
# @option -u --user Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
  inspect_args
  top
}

bottom() {
  echo bottom
}

root() {
  top
  middle
  bottom
}
