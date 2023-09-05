#!/usr/bin/env bash
# @name catch-all-global
# @version 0.0.1
# @description Catch All global example
# @flag -d --debug Debug mode
# @any! <GLOBAL_REQUIRED_ADDITIONAL_ARGUMENTS> Required additional arguments

# @cmd Command with a simple optional argument
# @arg message Message
# @any! <VALUE_NOTATION> Command specific required additional arguments
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

# Root command
root() {
  if [[ -n "${args['--debug']}" ]]; then
    set -x
  fi

  inspect_args
}
