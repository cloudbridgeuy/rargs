#!/usr/bin/env bash
# @name catch-all
# @version 0.0.1
# @description Catch All example
# @flag -d --debug Debug mode
# @rule catch-all

# @cmd Command with a simple optional argument
# @arg message Message
no-multiple() {
  if [[ -z "${args['--debug']}" ]]; then
    set -x
  fi

  inspect_args
}

# @cmd With a multiple required argument
# @arg message+ Message
multiple() {
  if [[ -z "${args['--debug']}" ]]; then
    set -x
  fi

  inspect_args
}
