#!/usr/bin/env bash
# @name dependencies-alt
# @version 0.0.1
# @description Demonstrates how to require your script's user to have at least one of a list of dependencies (commands) installed prior to using your script.
# @rule use-global-deps-for-root

# @cmd Download a file
# @alias d
# @alias down
# @dep [foo,bar,baz,git] scm Install with \e[32mgem install foo bar baz\e[0m
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
# @dep docker visit https://docker.com for more information
# @dep git You don't have git?
# @dep foo
# @arg source! URL to download from
# @option -u --user Username to use for logging in
# @option -p --password Password to use for logging in
upload() {
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

# @dep fail This is meant to fail
# @dep again Also this
root() {
  echo "Fallback to root command"
}
