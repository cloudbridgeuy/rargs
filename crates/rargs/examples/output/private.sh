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
# Root level lines
# shellcheck disable=SC2154
version() {
  echo "0.0.1"
}
usage() {
  printf "Sample application with private commands\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  private [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  private -h|--help\n"
  printf "  private -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  connect .... Connect to the metaverse
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
    c|connect)
      action="connect"
      input=("${input[@]:1}")
      ;;
    connect-ftp)
      action="connect-ftp"
      input=("${input[@]:1}")
      ;;
    connect-ssh)
      action="connect-ssh"
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
connect_usage() {
  printf "Connect to the metaverse\n"
  printf "\n\033[4m%s\033[0m %s\n" "Alias:" "c"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  connect -p|--protocol <PROTOCOL> [OPTIONS] HOST\n"
  printf "  connect -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  HOST\n"
  printf "    Hostname to connect to\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -p --protocol <PROTOCOL>\n"
  printf "    Protocol to use for connection\n"
  printf "    [@choices ftp, ssh]\n"
  printf "  -v --verbose\n"
  printf "    Verbose output\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_connect_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        connect_usage
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
      -v | --verbose)
        rargs_verbose=1
        shift
        ;;
      -p | --protocol)
        rargs_protocol="$2"
        shift 2
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z "$rargs_host" ]]; then
          rargs_host=$key
          shift
        else
          printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Connect to the metaverse
connect() {
  # Parse command arguments
  parse_connect_arguments "$@"

  
  if [[ -n "$rargs_protocol" ]]; then
    if [[ ! "(ftp ssh)" =~ $rargs_protocol ]]; then
      printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "protocol" ": " "$rargs_protocol" >&2
      connect_usage >&2
      exit 1
    fi
  fi
  
  if [[ -z "$rargs_protocol" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "protocol" >&2
    connect_usage >&2
    exit 1
  fi
  if [[ -z "$rargs_host" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "host" >&2
    connect_usage >&2
    exit 1
  fi
  if [[ "$rargs_protocol" == "ftp" ]]; then
    connect-ftp --username ftp_user
  else
    connect-ssh --username ssh_ser
  fi
}
connect-ftp_usage() {
  printf "Connect using FTP\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  connect-ftp [OPTIONS] HOST\n"
  printf "  connect-ftp -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  HOST\n"
  printf "    Hostname to connect to\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  --username [<USERNAME>]\n"
  printf "    Username\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_connect-ftp_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        connect-ftp_usage
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
      --username)
        rargs_username="$2"
        shift 2
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z "$rargs_host" ]]; then
          rargs_host=$key
          shift
        else
          printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Connect using FTP
connect-ftp() {
  # Parse command arguments
  parse_connect-ftp_arguments "$@"

  
  if [[ -z "$rargs_host" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "host" >&2
    connect-ftp_usage >&2
    exit 1
  fi
  echo "FTP!!!"
  inspect_args
}
connect-ssh_usage() {
  printf "Connect using SSH\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  connect-ssh [OPTIONS] HOST\n"
  printf "  connect-ssh -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  HOST\n"
  printf "    Hostname to connect to\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  --username [<USERNAME>]\n"
  printf "    Username\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_connect-ssh_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      -h|--help)
        connect-ssh_usage
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
      --username)
        rargs_username="$2"
        shift 2
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z "$rargs_host" ]]; then
          rargs_host=$key
          shift
        else
          printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Connect using SSH
connect-ssh() {
  # Parse command arguments
  parse_connect-ssh_arguments "$@"

  
  if [[ -z "$rargs_host" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "host" >&2
    connect-ssh_usage >&2
    exit 1
  fi
  echo "SSH!!!"
  inspect_args
}

run() {
  declare -A deps=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  # Call the right command action
  case "$action" in
    "connect")
      connect "${input[@]}"
      exit
      ;;
    "connect-ftp")
      connect-ftp "${input[@]}"
      exit
      ;;
    "connect-ssh")
      connect-ssh "${input[@]}"
      exit
      ;;
    "")
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing command. Select one of " "connect" >&2
      usage >&2
      exit 1
      ;;
    
  esac
}

run "$@"