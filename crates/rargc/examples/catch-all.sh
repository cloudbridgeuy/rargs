#!/usr/bin/env bash
# @name catch-all
# @version 0.0.1
# @description Catch All example
# @rule catch-all

# @flag -d --debug Debug mode
# @arg message+ Message
root() {
  if [[ -z "${args['--debug']}" ]]; then
    set -x
  fi

  inspect_args
}

