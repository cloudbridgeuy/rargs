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
      -f | --format)
        rargs_format="$2"
        shift 2
        ;;
      --)
        shift
        rargs_other_args+=("$@")
        break
        ;;
      -?*)
        rargs_other_args+=("$1")
        shift
        ;;
      *)
        rargs_other_args+=("$1")
        shift
        ;;
    esac
  done
}

root() {
  local rargs_format

  # Parse command arguments
  parse_root "$@"

  if [[ -z "$rargs_format" ]]; then
    rargs_format="json"
  fi
  
  if [[ -n "$rargs_format" ]]; then
    if [[ ! "(json csv)" =~ $rargs_format ]]; then
      printf "\e[31m%s\e[33m%s\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option for " "format" ": " "$rargs_format" >&2
      usage >&2
      exit 1
    fi
  fi
	echo "${rargs_input[*]}"
	# If rargs_other_args[0] is "-" or empty, read from stdin
	if [[ "${rargs_other_args[0]}" == "-" || -z "${rargs_other_args[0]}" ]]; then
		read_stdin
	else
		read_files
	fi
	printf "\nCollected file content:\n%s\n" "$content"
	# Read contents of the provided file(s)
	content=""
	for file in "${rargs_other_args[@]}"; do
		content+=$(cat "$file")
		content+=$'\n'
	done
	# Read from stdin if no file is provided
	if [[ -z "$content" ]]; then
		content=$(cat -)
	fi
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

read_stdin() {
	# Read from stdin
	content=$(cat -)
}
read_files() {
	# Read contents of the provided file(s)
	content=""
	for file in "${rargs_other_args[@]}"; do
		content+=$(cat "$file")
		content+=$'\n'
	done
}

version() {
  echo -n "0.0.1"
}
usage() {
  printf "Catch All with stdin input\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  catch-all-stdin [OPTIONS] ...[FILE]\n"
  printf "  catch-all-stdin -h|--help\n"
  printf "  catch-all-stdin -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Examples:"
  printf "  catch-all-stdin file1 file2 --format csv\n"
  printf "    Read two csv files\n"
  printf "  catch-all-stdin --format json file1 file2\n"
  printf "    Read two json files\n"
  printf "  catch-all-stdin -f csv < file1\n"
  printf "    Read a file from stdin\n"
  printf "  catch-all-stdin -f csv - < file1\n"
  printf "    Read a file from stdin using -\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  FILE\n"
  printf "    Path to one or more files. Reads from stdin if empty or "-"\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -f --format [<FORMAT>]\n"
  printf "    Specify the file format\n"
  printf "    [@default json, @choices json|csv]\n"
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
  declare -A deps=()
  declare -a rargs_other_args=()
  declare -a rargs_input=()
  normalize_rargs_input "$@"
  parse_arguments "${rargs_input[@]}"
  root "${rargs_input[@]}"
}

rargs_run "$@"
