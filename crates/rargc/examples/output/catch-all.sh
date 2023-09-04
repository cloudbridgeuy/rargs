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

  if ((${#other_args[@]})); then
    echo
    echo other_args:
    echo "- \${other_args[*]} = ${other_args[*]}"
    for i in "${!other_args[@]}"; do
      echo "- \${other_args[$i]} = ${other_args[$i]}"
    done
  fi
}
version() {
  echo "0.0.1"
}
usage() {
  printf "Catch All example\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  catch-all [OPTIONS] [COMMAND] [COMMAND_OPTIONS] ...\n"
  printf "  catch-all -h|--help\n"
  printf "  catch-all -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  multiple ....... With a multiple required argument
  no-multiple .... Command with a simple optional argument
EOF

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
    multiple)
      action="multiple"
      input=("${input[@]:1}")
      ;;
    no-multiple)
      action="no-multiple"
      input=("${input[@]:1}")
      ;;
    -h|--help)
      usage
      exit
      ;;
    "")
      ;;
    *)
      printf "Invalid command: %s\n" "$action" >&2
      exit 1
      ;;
  esac
}
multiple_usage() {
  printf "With a multiple required argument\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  multiple [OPTIONS] MESSAGE... \n"
  printf "  multiple -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  MESSAGE\n"
  printf "    Message\n"
  printf "    [@required, @multiple]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -d --debug\n"
  printf "    Debug mode\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_multiple_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        multiple_usage
        exit
        ;;
      *)
        break
        ;;
    esac
  done

  while [[ $# -gt 0 ]]; do
    key="$1"
    case "$key" in
      -d | --debug)
        args['--debug']=1
        shift
        ;;
      --)
        shift
        other_args+=("$@")
        break
        ;;
      -?*)
        other_args+=("$1")
        shift
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
# With a multiple required argument
multiple() {
  # Parse command arguments
  parse_multiple_arguments "${input[@]}"

  
  if [[ -z "${args['message']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "message" >&2
    multiple_usage >&2
    exit 1
  fi
  if [[ -z "${args['--debug']}" ]]; then
    set -x
  fi
  inspect_args
}
no-multiple_usage() {
  printf "Command with a simple optional argument\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  no-multiple [OPTIONS] [MESSAGE] \n"
  printf "  no-multiple -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  MESSAGE\n"
  printf "    Message\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -d --debug\n"
  printf "    Debug mode\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_no-multiple_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        no-multiple_usage
        exit
        ;;
      *)
        break
        ;;
    esac
  done

  while [[ $# -gt 0 ]]; do
    key="$1"
    case "$key" in
      -d | --debug)
        args['--debug']=1
        shift
        ;;
      --)
        shift
        other_args+=("$@")
        break
        ;;
      -?*)
        other_args+=("$1")
        shift
        ;;
      *)
        if [[ -z ${args['message']+x} ]]; then
          args['message']=$key
          shift
        else
          other_args+=("$1")
          shift
        fi
        ;;
    esac
  done
}
# Command with a simple optional argument
no-multiple() {
  # Parse command arguments
  parse_no-multiple_arguments "${input[@]}"

  if [[ -z "${args['--debug']}" ]]; then
    set -x
  fi
  inspect_args
}
run() {
  declare -A args=()
  declare -A deps=()
  declare -a other_args=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  # Call the right command action
  case "$action" in
    "multiple")
      multiple
      exit
      ;;
    "no-multiple")
      no-multiple
      exit
      ;;
    "")
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing command. Select one of" "multiple,no-multiple" >&2
      usage >&2
      exit 1
      ;;
    
  esac
}

run "$@"
