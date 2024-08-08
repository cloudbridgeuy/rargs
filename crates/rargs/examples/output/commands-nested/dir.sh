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
  printf "Directory commands\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  dir [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  dir -h|--help\n"
  printf "  dir -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  list ...... Show files in the directory
  remove .... Remove directory
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
    list)
      action="list"
      rargs_input=("${rargs_input[@]:1}")
      ;;
    remove)
      action="remove"
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
list_usage() {
  printf "Show files in the directory\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  list [OPTIONS] PATH\n"
  printf "  list -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  PATH\n"
  printf "    Directory path\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_list_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        list_usage
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
# Show files in the directory
list() {
  local rargs_path

  # Parse command arguments
  parse_list_arguments "$@"

  
  if [[ -z "$rargs_path" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "path" >&2
    list_usage >&2
    exit 1
  fi
	echo "${rargs_input[*]}"
}
remove_usage() {
  printf "Remove directory\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  remove [OPTIONS] PATH\n"
  printf "  remove -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  PATH\n"
  printf "    Directory path\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -f --force\n"
  printf "    Remove even if not empty\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_remove_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        remove_usage
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
      -f | --force)
        rargs_force=1
        shift
        ;;
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
# Remove directory
remove() {
  local rargs_force
  local rargs_path

  # Parse command arguments
  parse_remove_arguments "$@"

  
  if [[ -z "$rargs_path" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "path" >&2
    remove_usage >&2
    exit 1
  fi
	echo "${rargs_input[*]}"
}

rargs_run() {
  declare -A deps=()
  declare -a rargs_input=()
  normalize_rargs_input "$@"
  parse_arguments "${rargs_input[@]}"
  # Call the right command action
  case "$action" in
    "list")
      list "${rargs_input[@]}"
      exit
      ;;
    "remove")
      remove "${rargs_input[@]}"
      exit
      ;;
    "")
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing command. Select one of " "list, remove" >&2
      usage >&2
      exit 1
      ;;
    
  esac
}

rargs_run "$@"
