#!/usr/bin/env bash
# @name stdin
# @description STDIN Example
# @version 0.0.1

# @cmd Read from stdin or from a file using an argument
# @arg path="-" Path to file (reads from stdin if empty)
arg() {
  inspect_args

  # Since cat knows how to handle "-" as a value, it will work with boht a file path and "-".
  cat "${args["path"]}"
}


# @cmd Read from stdin or from a file using an option
# @option -p --path="-" Path to file (reads from stdin if empty)
option() {
  inspect_args

  # Since cat knows how to handle "-" as a value, it will work with boht a file path and "-".
  cat "${args["path"]}"
}

