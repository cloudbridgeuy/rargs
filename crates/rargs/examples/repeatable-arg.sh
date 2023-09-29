#!/usr/bin/env bash
# @name repeatable-arg
# @description Sample application to demonstrate the use of repeatable arguments
# @version 0.0.1

# @cmd Formats
# @arg formats+[=jpg|png|gif] <FORMATS> One or more formats to process
# @option -a --action*[upcase|downcase] <ACTION> Action to perform
formats() {
  for format in "${rargs_formats[@]}"; do
    echo "format: $format"
    for action in "${rargs_action[@]}"; do
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
  echo
  echo "files:"
  for file in "${rargs_files[@]}"; do
    echo "  path: $file"
    for action in "${rargs_action[@]}"; do
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
