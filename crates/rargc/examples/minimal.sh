#!/usr/bin/env bash
# @name minimal
# @version 0.0.1
# @description Sample minimal application without commands
# @arg source! URL to download from
# @arg target Target filename (default: same as source)
# @flag -f --force Overwrite existing files
# @example Download a file from the internet $ example.com
# @example Download a file from the internet and force save it to ./output $ example.com ./output -f

root() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}
