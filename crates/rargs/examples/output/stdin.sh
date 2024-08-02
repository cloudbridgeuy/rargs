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
  printf "STDIN Example\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  stdin [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  stdin -h|--help\n"
  printf "  stdin -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  arg ....... Read from stdin or from a file using an argument
  option .... Read from stdin or from a file using an option
EOF

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -h --help\n"
  printf "    Print help\n"
  printf "  -v --version\n"
  printf "    Print version\n"
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
    arg)
      action="arg"
      rargs_input=("${rargs_input[@]:1}")
      ;;
    option)
      action="option"
      rargs_input=("${rargs_input[@]:1}")
      ;;
    -h|--help)
      usage
      exit
      ;;
    "")
      ;;
    *)
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid command: " "$action" >&2
      exit 1
      ;;
  esac
}
arg_usage() {
  printf "Read from stdin or from a file using an argument\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  arg [OPTIONS] [PATH]\n"
  printf "  arg -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  PATH\n"
  printf "    Path to file (reads from stdin if empty)\n"
  printf "    [@default -]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_arg_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        arg_usage
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
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z "$rargs_path" ]]; then
          rargs_path=$key
          shift
        else
          printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Read from stdin or from a file using an argument
arg() {
  local rargs_path
  # Parse command arguments
  parse_arg_arguments "$@"

  
    
  if [[ -z "$rargs_path" ]]; then
    rargs_path="-"
  fi
    
	echo "${rargs_input[*]}"
	# Since cat knows how to handle "-" as a value, it will work with boht a file path and "-".
	cat "$rargs_path"
}
option_usage() {
  printf "Read from stdin or from a file using an option\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  option [OPTIONS]\n"
  printf "  option -h|--help\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -p --path [<PATH>]\n"
  printf "    Path to file (reads from stdin if empty)\n"
  printf "    [@default -]\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_option_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        option_usage
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
      -p | --path)
        rargs_path="$2"
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
# Read from stdin or from a file using an option
option() {
  local rargs_path
  # Parse command arguments
  parse_option_arguments "$@"

  
    
  if [[ -z "$rargs_path" ]]; then
    rargs_path="-"
  fi
    
	echo "${rargs_input[*]}"
	# Since cat knows how to handle "-" as a value, it will work with boht a file path and "-".
	cat "$rargs_path"
}

rargs_run() {
  declare -A deps=()
  declare -a rargs_input=()
  normalize_rargs_input "$@"
  parse_arguments "${rargs_input[@]}"
  # Call the right command action
  case "$action" in
    "arg")
      arg "${rargs_input[@]}"
      exit
      ;;
    "option")
      option "${rargs_input[@]}"
      exit
      ;;
    "")
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing command. Select one of " "arg, option" >&2
      usage >&2
      exit 1
      ;;
    
  esac
}

rargs_run "$@"
