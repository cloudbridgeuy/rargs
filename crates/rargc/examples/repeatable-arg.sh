#!/usr/bin/env bash
# @name repeatable-arg
# @description Sample application to demonstrate the use of repeatable arguments
# @version 0.0.1

# @cmd Formats
# @arg formats+[=jpg|png|gif] <FORMATS> One or more formats to process
# @option -a --action*[upcase|downcase] <ACTION> Action to perform
formats() {
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

# @arg files+ <FILES> One or more files to process
# @option -a --action*[upcase|downcase] <ACTION> Action to perform
# @example Multiple files $ README.md LICENSE
# @example Use a glob $ *.md
root() {
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
