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
  # Parse command arguments
  parse_root "$@"

  # Check dependencies
  dependency="fail"
  if ! command -v $dependency >/dev/null 2>&1; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
    printf "This is meant to fail\n" >&2
    exit 1
  else
    deps["$dependency"]="$(command -v $dependency | head -n1)"
  fi
  dependency="again"
  if ! command -v $dependency >/dev/null 2>&1; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
    printf "Also this\n" >&2
    exit 1
  else
    deps["$dependency"]="$(command -v $dependency | head -n1)"
  fi

  echo "Fallback to root command"
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
  printf "Sample application that requires dependencies\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  dependencies [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  dependencies -h|--help\n"
  printf "  dependencies -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  download .... Download a file
  upload ...... Upload a file
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
    d|down|download)
      action="download"
      input=("${input[@]:1}")
      ;;
    u|upload)
      action="upload"
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
download_usage() {
  printf "Download a file\n"
  printf "\n\033[4m%s\033[0m %s\n" "Alias:" "d, down"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  download [OPTIONS] SOURCE [TARGET]\n"
  printf "  download -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  SOURCE\n"
  printf "    URL to download from\n"
  printf "    [@required]\n"
  printf "  TARGET\n"
  printf "    Target filename (default: same as source)\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -f --force\n"
  printf "    Overwrite existing files\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_download_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        download_usage
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
        if [[ -z "$rargs_source" ]]; then
          rargs_source=$key
          shift
        elif [[ -z "$rargs_target" ]]; then
          rargs_target=$key
          shift
        else
          printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Download a file
download() {
  # Parse command arguments
  parse_download_arguments "$@"

  # Check dependencies
  for dependency in foo bar baz; do
    if ! command -v $dependency >/dev/null 2>&1; then
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
      printf "install with \e[32mgem install foo bar baz\e[0m\n" >&2
      exit 1
    else
      deps["$dependency"]="$(command -v $dependency | head -n1)"
    fi
  done
  dependency="git"
  if ! command -v $dependency >/dev/null 2>&1; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
    printf "You can install git with \e[32mapt install git\e[0m\n" >&2
    exit 1
  else
    deps["$dependency"]="$(command -v $dependency | head -n1)"
  fi

  
  if [[ -z "$rargs_source" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "source" >&2
    download_usage >&2
    exit 1
  fi
  inspect_args
}
upload_usage() {
  printf "Upload a file\n"
  printf "\n\033[4m%s\033[0m %s\n" "Alias:" "u"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  upload [OPTIONS] SOURCE\n"
  printf "  upload -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  SOURCE\n"
  printf "    URL to download from\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -p --password [<PASSWORD>]\n"
  printf "    Password to use for logging in\n"
  printf "  -u --user [<USER>]\n"
  printf "    Username to use for logging in\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_upload_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        upload_usage
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
      -p | --password)
        rargs_password="$2"
        shift 2
        ;;
      -u | --user)
        rargs_user="$2"
        shift 2
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z "$rargs_source" ]]; then
          rargs_source=$key
          shift
        else
          printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Upload a file
upload() {
  # Parse command arguments
  parse_upload_arguments "$@"

  # Check dependencies
  dependency="docker"
  if ! command -v $dependency >/dev/null 2>&1; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
    printf "visit https://docker.com for more information\n" >&2
    exit 1
  else
    deps["$dependency"]="$(command -v $dependency | head -n1)"
  fi
  dependency="foo"
  if ! command -v $dependency >/dev/null 2>&1; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
    exit 1
  else
    deps["$dependency"]="$(command -v $dependency | head -n1)"
  fi
  dependency="git"
  if ! command -v $dependency >/dev/null 2>&1; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
    printf "You don't have git?\n" >&2
    exit 1
  else
    deps["$dependency"]="$(command -v $dependency | head -n1)"
  fi

  
  if [[ -z "$rargs_source" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "source" >&2
    upload_usage >&2
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
    "download")
      download "${input[@]}"
      exit
      ;;
    "upload")
      upload "${input[@]}"
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
