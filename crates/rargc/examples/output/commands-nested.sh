#!/usr/bin/env bash

# This script was generated by rargc 0.0.0 (https://rargc.cloudbridge.uy)
# Modifying it manually is not recommended

if [[ "${BASH_VERSINFO:-0}" -lt 4 ]]; then
  printf "bash version 4 or higher is required\n" >&2
  exit 1
fi


version() {
  echo "0.0.1"
}

usage() {
  printf "Sample application with nested commands\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  commands-nested [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  commands-nested -h|--help\n"
  printf "  commands-nested -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Examples:"
  printf "  commands-nested dir\n"
  printf "    Directory command\n"
  printf "  commands-nested dir -h\n"
  printf "    Directory help command\n"
  printf "  commands-nested file -h\n"
  printf "    Directory file command\n"
  printf "  commands-nested dir list\n"
  printf "    List directory sub-command\n"
  printf "  commands-nested dir list -h\n"
  printf "    List directory sub-command help\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  dir ..... Directory commands
  file .... File commands
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
    d|dir)
        action="dir"
        input=("${input[@]:1}")
        ;;
    f|file)
        action="file"
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


dir_usage() {
  printf "Directory commands\n"
  printf "\n\033[4m%s\033[0m %s\n" "Alias:" "d"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  dir [OPTIONS]\n"
  printf "  dir -h|--help\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -h --help\n"
  printf "    Print help\n"
}

parse_dir_arguments() {

  while [[ $# -gt 0 ]]; do
    key="$1"
    case "$key" in
      *)
        break
        ;;
    esac
  done
}

# Directory commands
dir() {

  local sub="/Users/guzmanmonne/Projects/Rust/rargc/crates/rargc/examples/output/commands-nested/dir.sh"
  # shellcheck disable=SC2068
  # shellcheck disable=SC2154
  "$sub" ${input[@]}
}

file_usage() {
  printf "File commands\n"
  printf "\n\033[4m%s\033[0m %s\n" "Alias:" "f"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  file [OPTIONS]\n"
  printf "  file -h|--help\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -h --help\n"
  printf "    Print help\n"
}

parse_file_arguments() {

  while [[ $# -gt 0 ]]; do
    key="$1"
    case "$key" in
      *)
        break
        ;;
    esac
  done
}

# File commands
file() {

  local sub="/Users/guzmanmonne/Projects/Rust/rargc/crates/rargc/examples/output/commands-nested/file.sh"
  # shellcheck disable=SC2068
  # shellcheck disable=SC2154
  "$sub" ${input[@]}
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
  # Call the right command action
  case "$action" in
    "dir")
      parse_dir_arguments "${input[@]}"
      shift $#
      dir
      ;;
  
    "file")
      parse_file_arguments "${input[@]}"
      shift $#
      file
      ;;
  esac
}


initialize
run "$@"
