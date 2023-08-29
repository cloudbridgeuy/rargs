#!/usr/bin/env bash
# @name dir
# @version 0.0.1
# @description Directory commands

# @cmd Show files in the directory
# @arg path! Directory path
list() {
  inspect_args
}

# @cmd Remove directory
# @arg path! Directory path
# @flag -f --force Remove even if not empty
remove() {
  inspect_args
}
