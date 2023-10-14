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
  printf "Call the Huggingface API through curl\n"
  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  huggingface [OPTIONS] [COMMAND] [COMMAND_OPTIONS]\n"
  printf "  huggingface -h|--help\n"
  printf "  huggingface -v|--version\n"
  printf "\n\033[4m%s\033[0m\n" "Commands:"
  cat <<EOF
  fill-mask .... Fill Mask
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
      *)
        break
        ;;
    esac
  done
  action="${1:-}"

  case $action in
    fill-mask)
      action="fill-mask"
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
fill-mask_usage() {
  printf "Fill Mask\n"
  printf "Tries to fill a hole with a missing word (token to be precise).\n"

  printf "\n\033[4m%s\033[0m\n" "Usage:"
  printf "  fill-mask [OPTIONS] TEXT\n"
  printf "  fill-mask -h|--help\n"
  printf "\n\033[4m%s\033[0m\n" "Examples:"
  printf "  fill-mask \"The capital of France is [MASK]\"\n"
  printf "    Replace the value of [MASK] with the most likely word\n"
  printf "\n\033[4m%s\033[0m\n" "Arguments:"
  printf "  TEXT\n"
  printf "    Text with mask to fill\n"
  printf "    [@required]\n"

  printf "\n\033[4m%s\033[0m\n" "Options:"
  printf "  -m --model [<MODEL>]\n"
  printf "    Model to use.\n"
  printf "    [@default bert-base-uncased]\n"
  printf "  -u --url [<URL>]\n"
  printf "    Hugging Face API base URL.\n"
  printf "    [@default https://api-inference.huggingface.co/models]\n"
  printf "  --no-use-cache\n"
  printf "    Do not use the cache layer on the interface API to speedup requests.\n"
  printf "  --wait-for-model\n"
  printf "    If the model is not ready, wait for it instead of receiving a 503 error.\n"
  printf "  -h --help\n"
  printf "    Print help\n"
}
parse_fill-mask_arguments() {
  while [[ $# -gt 0 ]]; do
    case "${1:-}" in
      *)
        break
        ;;
    esac
  done

  while [[ $# -gt 0 ]]; do
    key="$1"
    case "$key" in
      --no-use-cache)
        rargs_no_use_cache=1
        shift
        ;;
      --wait-for-model)
        rargs_wait_for_model=1
        shift
        ;;
      -m | --model)
        rargs_model="$2"
        shift 2
        ;;
      -u | --url)
        rargs_url="$2"
        shift 2
        ;;
      -h|--help)
        rargs_help=1
        shift 1
        ;;
      -?*)
        printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid option: " "$key" >&2
        exit 1
        ;;
      *)
        if [[ -z "$rargs_text" ]]; then
          rargs_text=$key
          shift
        else
          printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Invalid argument: " "$key" >&2
          exit 1
        fi
        ;;
    esac
  done
}
# Fill Mask
# Tries to fill a hole with a missing word (token to be precise).
fill-mask() {
  # Parse environment variables
  
  if [[ -z "${HUGGING_FACE_API_TOKEN:-}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required environment variable: " "HUGGING_FACE_API_TOKEN" >&2
    fill-mask_usage >&2
    exit 1
  fi

  # Parse command arguments
  parse_fill-mask_arguments "$@"

  # Rule `no-first-option-help`: Render the global or command usage if the `-h|--help` option is
  #                              is provided anywhere on the command, not just as the first option.
  #                              Handling individual functions case by case.
  if [[ -n "$rargs_help" ]]; then
    fill-mask_usage
    exit 0
  fi
  
    
  if [[ -z "$rargs_model" ]]; then
    rargs_model="bert-base-uncased"
  fi
    
    
  if [[ -z "$rargs_url" ]]; then
    rargs_url="https://api-inference.huggingface.co/models"
  fi
    
  
  if [[ -z "$rargs_text" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required option: " "text" >&2
    fill-mask_usage >&2
    exit 1
  fi
  body="$(jo inputs="$rargs_text")"
  if [[ -n "$rargs_no_use_cache" ]]; then
    body="$(jo -f <(echo -n "$body") use_cache=false)"
  fi
  if [[ -n "$rargs_wait_for_model" ]]; then
    body="$(jo -f <(echo -n "$body") wait_for_model=true)"
  fi
  curl -sL "$rargs_url/$rargs_model" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $HUGGING_FACE_API_TOKEN" \
    -d "$body"
}

run() {
  declare -A deps=()
  declare -a input=()
  normalize_input "$@"
  parse_arguments "${input[@]}"
  # Rule `no-first-option-help`: Render the global or command usage if the `-h|--help` option is
  #                              is provided anywhere on the command, not just as the first option.
  #                              Handling the case where no action was selected.
  if [[ -n "$rargs_help" ]] && [[ -z "$action" ]]; then
    usage
    exit 0
  fi
  # Check global dependencies
  
  for dependency in curl jo; do
    if ! command -v $dependency >/dev/null 2>&1; then
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing dependency: " "$dependency" >&2
      printf "Please install \e[32mcurl\e[0m with \e[32mbrew\e[0m or \e[32maapt-get\e[0m\n" >&2
      exit 1
    else
      deps["$dependency"]="$(command -v $dependency | head -n1)"
    fi
  done

  # Check global environment variables
  
  if [[ -z "${HUGGING_FACE_API_TOKEN:-}" ]]; then
    printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing required environment variable: " "HUGGING_FACE_API_TOKEN" >&2
    usage >&2
    exit 1
  fi

  # Call the right command action
  case "$action" in
    "fill-mask")
      fill-mask "${input[@]}"
      exit
      ;;
    "")
      printf "\e[31m%s\e[33m%s\e[31m\e[0m\n\n" "Missing command. Select one of " "fill-mask" >&2
      usage >&2
      exit 1
      ;;
    
  esac
}

run "$@"
