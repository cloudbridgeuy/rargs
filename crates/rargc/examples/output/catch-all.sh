#!/usr/bin/env bash
# This script was generated by rargc 0.0.0 (https://rargc.cloudbridge.uy)
# Modifying it manually is not recommended

if [[ "${BASH_VERSINFO:-0}" -lt 4 ]]; then
  printf "bash version 4 or higher is required\n" >&2
  exit 1
fi

if [[ -n "${DEBUG:-}" ]]; then
  set -x
fi

set -e

parse_root() {

  while [[ $# -gt 0 ]]; do
    key="$1"
    case "$key" in
      -d | --debug)
        args['--debug']=1
        shift
        ;;
      -?*)
        printf "invalid option: %s\n" "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z ${args['message']+x} ]]; then
          args['message']=$key
          shift
        else
          args['message']="${args['message']} \"$key\""
          shift
        fi
        ;;
    esac
  done
}
root() {
  # Parse command arguments
  parse_root "${input[@]}"

  
  if [[ -z "${args['message']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "message" >&2
    usage >&2
    exit 1
  fi
  if [[ -z "${args['--debug']}" ]]; then
    set -x
  fi
  inspect_args
}



normalize_input() {
  local arg flags

  while [[ $# -gt 0 ]]; do
    arg="$1"
    if [[ $arg =~ ^(--[a-zA-Z0-9_\-]+)=(.+)$ ]]; then
      input+=("${BASH_REMATCH[1]}")
      input+=("${BASH_REMATCH[2]}")
    elif [[ $arg =~ ^(-[a-zA-Z0-9])=(.+)$ ]]; then
      input+=("${BASH_REMATCH[1]}")
      input+=("${BASH_REMATCH[2]}")
    elif [[ $arg =~ ^-([a-zA-Z0-9][a-zA-Z0-9]+)$ ]]; then
      flags="${BASH_REMATCH[1]}"
      for ((i = 0; i < ${#flags}; i++)); do
        input+=("-${flags:i:1}")
      done
    else
      input+=("$arg")
    fi

    shift
  done
}

inspect_args() {
  if ((${#args[@]})); then
    readarray -t sorted_keys < <(printf '%s\n' "${!args[@]}" | sort)
    echo args:
    for k in "${sorted_keys[@]}"; do echo "- \${args[$k]} = ${args[$k]}"; done
  else
    echo args: none
  fi

  if ((${#deps[@]})); then
    readarray -t sorted_keys < <(printf '%s\n' "${!deps[@]}" | sort)
    echo
    echo deps:
    for k in "${sorted_keys[@]}"; do echo "- \${deps[$k]} = ${deps[$k]}"; done
  fi
}


version() {
  echo "0.0.1"
}

usage() {
  printf "Catch All example\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  catch-all [OPTIONS] MESSAGE... \n"
  printf "  catch-all -h|--help\n"
  printf "  catch-all -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  MESSAGE\n"
  printf "    Message\n"
  printf "    [@required, @multiple]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -d --debug\n"
  printf "    Debug mode\n"
  printf "  -h --help\n"
  printf "    Print help\n"
  printf "  -v --version\n"
  printf "    Pring version\n"
}

parse_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -v|--version)
        version
        exit
        ;;
      -h|--help)
        usage
        exit
        ;;
      *)
        break
        ;;
    esac
  done
  action="${1:-}"

  case $action in
    -h|--help)
      usage
      exit
      ;;
    "")
      action="root"
      ;;
    *)
      action="root"
      ;;
  esac
}

run() {
  declare -A args=()
  declare -A deps=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  root
}

run "$@"
