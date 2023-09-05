#!/usr/bin/env bash
# @name catch-all
# @version 0.0.1
# @description Catch All example
# @flag -d --debug Debug mode

# @cmd Command with a simple optional argument
# @arg message Message
# @any! <VALUE_NOTATION> Required additional arguments
no-multiple() {
  if [[ -n "${args['--debug']}" ]]; then
    set -x
  fi

  inspect_args
}

# @cmd With a multiple required argument
# @arg message+ Message
# @any Optional additional arguments
multiple() {
  if [[ -n "${args['--debug']}" ]]; then
    set -x
  fi

  inspect_args
}

# @cmd Any arguments without description
# @any
other() {
  if [[ -n "${args['--debug']}" ]]; then
    set -x
  fi

  inspect_args
}
