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
  prefix="rargs_"
  args="$(set | grep ^$prefix || true)"
  if [[ -n "$args" ]]; then
    echo
    echo args:
    for var in $args; do
      echo "- $var" | sed 's/=/ = /g'
    done
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
      input=("${input[@]:1}")
      ;;
    remove)
      action="remove"
      input=("${input[@]:1}")
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
  # Parse command arguments
  parse_list_arguments "$@"

  
  if [[ -z "$rargs_path" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "path" >&2
    list_usage >&2
    exit 1
  fi
  inspect_args
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
  # Parse command arguments
  parse_remove_arguments "$@"

  
  if [[ -z "$rargs_path" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "path" >&2
    remove_usage >&2
    exit 1
  fi
  inspect_args
}

run() {
  declare -A deps=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  # Call the right command action
  case "$action" in
    "list")
      list "${input[@]}"
      exit
      ;;
    "remove")
      remove "${input[@]}"
      exit
      ;;
    "")
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing command. Select one of " "list, remove" >&2
      usage >&2
      exit 1
      ;;
    
  esac
}

run "$@"
