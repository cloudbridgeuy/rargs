#!/usr/bin/env bash
# shellcheck disable=SC2154
# @name single-with-rules
# @description Script without subcommands.
# @version 1.0.0
# @author Foo, Bar, Baz
# @rule no-first-option-help
# @help Prints the flags and options provided globally.
# @flag -f --flag Verbose mode
# @option -o --option Option with any value

echo "flag: ${args["--flag"]}"
echo "option: ${args["option"]}"
