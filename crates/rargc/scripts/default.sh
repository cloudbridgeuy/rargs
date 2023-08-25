#!/usr/bin/env bash
# shellcheck disable=SC2154
# @name default
# @description Script without subcommands.
# @version 1.0.0
# @author Foo, Bar, Baz
# @default main
# @help Prints the flags and options provided globally.
# @flag -v --verbose Verbose mode

# @cmd Main function
# @option -o --option Option with any value
# @flag -f --flag Flag option
main() {
  echo "flag: ${args["--flag"]}"
  echo "option: ${args["option"]}"
}

