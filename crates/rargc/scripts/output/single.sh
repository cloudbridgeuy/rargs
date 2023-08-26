#!/usr/bin/env bash

# This script was generated by rargc 0.0.0 (https://rargc.cloudbridge.uy)
# Modifying it manually is not recommended

if [[ "${BASH_VERSINFO:-0}" -lt 4 ]]; then
  printf "bash version 4 or higher is required\n" >&2
  exit 1
fi


version() {
  echo "1.0.0"
}


usage() {
  printf "Script without subcommands.\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  single [OPTIONS]\n"
  printf "  single -h|--help\n"
  printf "  single --version\n"
  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -o --option [<OPTION>]\n"
  printf "    Option with any value\n"
  printf "\n\033[4m%s\033[0m\n" "Flags:"
  printf "  -f --flag\n"
  printf "    Verbose mode\n"
  printf "  -h --help\n"
  printf "    Print help\n"
  printf "  --version\n"
  printf "    Pring version\n"
}


parse_arguments() {
  
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      --version)
        version
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

      -f | --flag)
        args['--flag']=1
        shift
        ;;
      -o | --option)
        args['option']=$2
        shift 2
        ;;

      *)
        break
        ;;
    esac
  done


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
}



initialize() {
  if [[ -n "${DEBUG:-}" ]]; then
    set -x
  fi

  set -e
}


run() {
  declare -A args=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  # Global script code
  echo "flag: ${args["--flag"]}"
  echo "option: ${args["option"]}"
  
}


initialize
run "$@"
