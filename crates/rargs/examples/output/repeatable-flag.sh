#!/usr/bin/env bash
# This script was generated by rargs 0.0.0 (https://rargs.cloudbridge.uy)
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
      -v | --verbose)
        if [[ -z "$rargs_verbose" ]]; then
          rargs_verbose=1
        else
          rargs_verbose=$(("$rargs_verbose" + 1))
        fi
        shift
        ;;
      -d | --data)
        rargs_data+=("$2")
        shift 2
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ "$key" == "" ]]; then
          break
        fi
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
        exit 1
        ;;
    esac
  done
}

root() {
  local rargs_verbose
  declare -a rargs_data

  # Parse command arguments
  parse_root "$@"

  
  if [[ "${#rargs_data[@]}" == 0 ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "data" >&2
    usage >&2
    exit 1
  fi
	echo "Data elements:"
	for i in "${rargs_data[@]}"; do
		echo "$i"
	done
	# The --verbose arg will contain the number of times it was used by the user
	echo ""
	echo "Verbosity level: $rargs_verbose"
	echo ""
	echo "${rargs_input[*]}"
}


normalize_rargs_input() {
  local arg flags

  while [[ $# -gt 0 ]]; do
    arg="$1"
    if [[ $arg =~ ^(--[a-zA-Z0-9_\-]+)=(.+)$ ]]; then
      rargs_input+=("${BASH_REMATCH[1]}")
      rargs_input+=("${BASH_REMATCH[2]}")
    elif [[ $arg =~ ^(-[a-zA-Z0-9])=(.+)$ ]]; then
      rargs_input+=("${BASH_REMATCH[1]}")
      rargs_input+=("${BASH_REMATCH[2]}")
    elif [[ $arg =~ ^-([a-zA-Z0-9][a-zA-Z0-9]+)$ ]]; then
      flags="${BASH_REMATCH[1]}"
      for ((i = 0; i < ${#flags}; i++)); do
        rargs_input+=("-${flags:i:1}")
      done
    else
      rargs_input+=("$arg")
    fi

    shift
  done
}

version() {
  echo -n "0.0.1"
}
usage() {
  printf "Sample application to demonstrate the use of repeatable flags\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  repeatable-arg -d|--data <DATA> [OPTIONS]\n"
  printf "  repeatable-arg -h|--help\n"
  printf "  repeatable-arg --version\n"
  printf "\n\033[4m%s\033[0m\n" "Examples:"
  printf "  repeatable-arg -d 1 -d 2 -d 3\n"
  printf "    Multiple data values\n"
  printf "  repeatable-arg -d 1 -d 2 -d 3 -v\n"
  printf "    Multiple data values with verbosity enabled\n"
  printf "  repeatable-arg -d 1 -d 2 -d 3 -vvv\n"
  printf "    Multiple data values with high verbosity enabled\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -d --data <DATA>\n"
  printf "    Provide data values\n"
  printf "    [@multiple]\n"
  printf "  -v --verbose\n"
  printf "    Set verbosity level\n"printf "    [@multiple]"
  printf "  -h --help\n"
  printf "    Print help\n"
  printf "  --version\n"
  printf "    Print version\n"
}

parse_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      --version)
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

rargs_run() {
  declare -a rargs_input=()
  normalize_rargs_input "$@"
  parse_arguments "${rargs_input[@]}"
  root "${rargs_input[@]}"
}

rargs_run "$@"
