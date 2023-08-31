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
      -?*)
        printf "invalid option: %s\n" "$key" >&2
        exit 1
        ;;
      *)
        printf "Invalid argument: %s\n" "$key" >&2
        exit 1
        ;;
    esac
  done
}
root() {
  # Parse environment variables
  

  # Parse command arguments
  parse_root "${input[@]}"

  inspect_args
  echo "DEBUG: ${DEBUG:-}"
  echo "ENVIRONMENT: ${ENVIRONMENT:-}"
  echo "SECRET: ${SECRET:-}"
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
  printf "Sample application that requires environment variables\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  environment-variables [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  environment-variables -h|--help\n"
  printf "  environment-variables -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  verify .... Verify your user
EOF

  printf "\n\033[4m%s\033[0m\n" "Options:"
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
    verify)
      action="verify"
      input=("${input[@]:1}")
      ;;
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

verify_usage() {
  printf "Verify your user\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  verify -s|--secret <SECRET> [OPTIONS]\n"
  printf "  verify -h|--help\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -m --multiple [<MULTIPLE>]\n"
  printf "    Multiple values\n"
  printf "    [@multiple]\n"
  printf "  -s --secret <SECRET>\n"
  printf "    Your secret key\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_verify_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        verify_usage
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
      -m | --multiple)
        if [[ -z ${args['multiple']+x} ]]; then
            args['multiple']=$2
          else
            args['multiple']="${args['multiple']:-} $2"
          fi
        shift 2
        ;;
      -s | --secret)
        args['secret']=$2
        shift 2
        ;;
      -?*)
        printf "invalid option: %s\n" "$key" >&2
        exit 1
        ;;
      *)
        printf "Invalid argument: %s\n" "$key" >&2
        exit 1
        ;;
    esac
  done
}
# Verify your user
verify() {
  # Parse environment variables
  
  if [[ -z "${ENVIRONMENT:-}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required environment variable: " "ENVIRONMENT" >&2
    verify_usage >&2
    exit 1
  fi
  if [[ -n "${MULTIPLE:-}" ]]; then
    args['multiple']="${MULTIPLE:-}"
  fi
  if [[ -n "${SECRET:-}" ]]; then
    args['secret']="${SECRET:-}"
  fi

  # Parse command arguments
  parse_verify_arguments "${input[@]}"

  
  if [[ -z "${args['secret']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "secret" >&2
    verify_usage >&2
    exit 1
  fi
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

run() {
  declare -A args=()
  declare -A deps=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  # Check global environment variables
  

  # Call the right command action
  case "$action" in
    "verify")
      verify
      exit
      ;;
  esac
  root
}

run "$@"