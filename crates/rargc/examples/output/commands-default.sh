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
  printf "Sample application that uses the default command option\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  commands [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  commands -h|--help\n"
  printf "  commands -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Examples:"
  printf "  commands \n"
  printf "    Run the default command\n"
  printf "  commands --help\n"
  printf "    Print the global help\n"
  printf "  commands download something\n"
  printf "    Download something\n"
  printf "  commands upload something\n"
  printf "    Upload something\n"
  printf "  commands something\n"
  printf "    Upload something using the default command\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  download .... Download a file
  upload ...... Upload a file
EOF
  printf "  [@default upload]\n"

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
    d|download)
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
      ;;
    *)
      action="upload"
      ;;
  esac
}

download_usage() {
  printf "Download a file\n"
  printf "\n\033[4m%s\033[0m %s\n" "Alias:" "d"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  download [OPTIONS] SOURCE \n"
  printf "  download -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  SOURCE\n"
  printf "    URL to download from\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
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
      -?*)
        printf "invalid option: %s\n" "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z ${args['source']+x} ]]; then
          args['source']=$key
          shift
        else
          printf "Invalid argument: %s\n" "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Download a file
download() {
  # Parse command arguments
  parse_download_arguments "${input[@]}"

  
  if [[ -z "${args['source']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "source" >&2
    download_usage >&2
    exit 1
  fi
  echo "# this file is located in './crates/rargc/examples/output.sh'"
  echo "# you can edit it freely and regenerate (it will not be overwritten)"
  inspect_args
}

upload_usage() {
  printf "Upload a file\n"
  printf "\n\033[4m%s\033[0m %s\n" "Alias:" "u"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  upload [OPTIONS] SOURCE \n"
  printf "  upload -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  SOURCE\n"
  printf "    URL to download from\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -f --force\n"
  printf "    Force upload\n"
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
      -f | --force)
        args['--force']=1
        shift
        ;;
      -?*)
        printf "invalid option: %s\n" "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z ${args['source']+x} ]]; then
          args['source']=$key
          shift
        else
          printf "Invalid argument: %s\n" "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Upload a file
upload() {
  # Parse command arguments
  parse_upload_arguments "${input[@]}"

  
  if [[ -z "${args['source']}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "source" >&2
    upload_usage >&2
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
  # Call the right command action
  case "$action" in
    "download")
      download
      exit
      ;;
    "upload")
      upload
      exit
      ;;
    "")
      usage
      exit
      ;;
  esac
  usage
}

run "$@"
