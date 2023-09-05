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
      -p | --protocol)
        args['protocol']="$2"
        shift 2
        ;;
      -u | --user)
        args['user']="$2"
        shift 2
        ;;
      -?*)
        printf "invalid option: %s\n" "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z ${args['region']+x} ]]; then
          args['region']=$key
          shift
        elif [[ -z ${args['environment']+x} ]]; then
          args['environment']=$key
          shift
        else
          printf "Invalid argument: %s\n" "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}

root() {
  # Parse command arguments
  parse_root "${input[@]}"

  
  if [[ -z "${args['protocol']}" ]]; then
    args['protocol']="ssh"
  fi
  if [[ -z "${args['environment']}" ]]; then
    args['environment']="development"
  fi
  
  if [[ -n "${args['protocol']}" ]]; then
    if [[ ! "(ssh ftp http)" =~ ${args['protocol']} ]]; then
      printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "protocol" ": " "${args['protocol']}" >&2
      usage >&2
      exit 1
    fi
  fi
  if [[ -n "${args['user']}" ]]; then
    if [[ ! "(user admin)" =~ ${args['user']} ]]; then
      printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "user" ": " "${args['user']}" >&2
      usage >&2
      exit 1
    fi
  fi
  if [[ -n "${args['region']}" ]]; then
    if [[ ! "(eu us)" =~ ${args['region']} ]]; then
      printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "region" ": " "${args['region']}" >&2
      usage >&2
      exit 1
    fi
  fi
  if [[ -n "${args['environment']}" ]]; then
    if [[ ! "(development stagin production)" =~ ${args['environment']} ]]; then
      printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "environment" ": " "${args['environment']}" >&2
      usage >&2
      exit 1
    fi
  fi
  
  if [[ -z "${args['user']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "user" >&2
    usage >&2
    exit 1
  fi
  if [[ -z "${args['region']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "region" >&2
    usage >&2
    exit 1
  fi
  echo "Root"
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
  printf "Sample showing the use of arg and option whitelist (allowed values)\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  whitelist -u|--user <USER> [OPTIONS] REGION [ENVIRONMENT] \n"
  printf "  whitelist -h|--help\n"
  printf "  whitelist -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  REGION\n"
  printf "    Region to connect to\n"
  printf "    [@required, @choices eu, us]\n"
  printf "  ENVIRONMENT\n"
  printf "    Environment to connect to\n"
  printf "    [@default development, @choices development, stagin, production]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -p --protocol [<PROTOCOL>]\n"
  printf "    Protocol to connect with\n"
  printf "    [@default ssh, @choices ssh, ftp, http]\n"
  printf "  -u --user <USER>\n"
  printf "    User name\n"
  printf "    [@choices user, admin]\n"
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
