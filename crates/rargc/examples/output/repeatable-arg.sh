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
      -a | --action)
        if [[ -z ${args['action']+x} ]]; then
            args['action']="$2"
          else
            args['action']="${args['action']:-} $2"
          fi
        shift 2
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z ${args['files']+x} ]]; then
          args['files']=$key
          shift
        else
          args['files']="${args['files']} \"$key\""
          shift
        fi
        ;;
    esac
  done
}

root() {
  # Parse command arguments
  parse_root "$@"

  
  eval "local args_action=(${args['action']})"
  for value in "${args_action[@]}"; do
    if [[ ! "(upcase downcase)" =~ ${value} ]]; then
      printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "action" ": " "${value}" >&2
      usage >&2
      exit 1
    fi
  done
  
  if [[ -z "${args['files']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "files" >&2
    usage >&2
    exit 1
  fi
  # Convert the space delimited string to an array
  files=""
  actions=""
  eval "files=(${args[files]})"
  eval "actions=(${args[action]})"
  echo
  echo "files:"
  for file in "${files[@]}"; do
    echo "  path: $file"
    for action in "${actions[@]}"; do
      case "$action" in
        upcase)
          echo "    upcase ${file^^}"
          ;;
        downcase)
          echo "    downcase ${file,,}"
          ;;
      esac
    done
  done
  echo
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
  printf "Sample application to demonstrate the use of repeatable arguments\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  repeatable-arg [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  repeatable-arg -h|--help\n"
  printf "  repeatable-arg -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Examples:"
  printf "  repeatable-arg README.md LICENSE\n"
  printf "    Multiple files\n"
  printf "  repeatable-arg *.md\n"
  printf "    Use a glob\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  FILES\n"
  printf "    One or more files to process\n"
  printf "    [@required, @multiple]\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  formats .... Formats
EOF

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -a --action [<ACTION>]\n"
  printf "    Action to perform\n"
  printf "    [@multiple, @choices upcase, downcase]\n"
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
    formats)
      action="formats"
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
formats_usage() {
  printf "Formats\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  formats [OPTIONS] FORMATS...\n"
  printf "  formats -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  FORMATS\n"
  printf "    One or more formats to process\n"
  printf "    [@required, @multiple, @default jpg, @choices jpg, png, gif]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -a --action [<ACTION>]\n"
  printf "    Action to perform\n"
  printf "    [@multiple, @choices upcase, downcase]\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_formats_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        formats_usage
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
      -a | --action)
        if [[ -z ${args['action']+x} ]]; then
            args['action']="$2"
          else
            args['action']="${args['action']:-} $2"
          fi
        shift 2
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z ${args['formats']+x} ]]; then
          args['formats']=$key
          shift
        else
          args['formats']="${args['formats']} \"$key\""
          shift
        fi
        ;;
    esac
  done
}
# Formats
formats() {
  # Parse command arguments
  parse_formats_arguments "$@"

  
  if [[ -z "${args['formats']}" ]]; then
    args['formats']="jpg"
  fi
  
  if [[ -n "${args['action']}" ]]; then
    eval "local args_action=(${args['action']})"
    for value in "${args_action[@]}"; do
      if [[ ! "(upcase downcase)" =~ ${value} ]]; then
        printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "action" ": " "${value}" >&2
        formats_usage >&2
        exit 1
      fi
    done
  fi
  if [[ -n "${args['formats']}" ]]; then
    eval "local args_formats=(${args['formats']})"
    for value in "${args_formats[@]}"; do
      if [[ ! "(jpg png gif)" =~ ${value} ]]; then
        printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "formats" ": " "${value}" >&2
        formats_usage >&2
        exit 1
      fi
    done
  fi
  
  if [[ -z "${args['formats']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "formats" >&2
    formats_usage >&2
    exit 1
  fi
  # Convert the space delimited string to an array
  local formats=""
  local actions=""
  eval "formats=(${args[formats]})"
  eval "actions=(${args[action]})"
  for format in "${formats[@]}"; do
    echo "format: $format"
    for action in "${actions[@]}"; do
      case "$action" in
        upcase)
          echo "  upcase ${format^^}"
          ;;
        downcase)
          echo "  downcase ${format,,}"
          ;;
      esac
    done
  done
}

run() {
  declare -A args=()
  declare -A deps=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  # Call the right command action
  case "$action" in
    "formats")
      formats "${input[@]}"
      exit
      ;;
    root)
      root "${input[@]}"
      exit
      ;;
    "")
      root "${input[@]}"
      ;;
    
  esac
}

run "$@"
