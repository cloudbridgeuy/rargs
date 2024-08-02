#!/usr/bin/env bash
# @name dir
# @version 0.0.1
# @description Directory commands

# @cmd Show files in the directory
# @arg path! Directory path
list() {
	echo "${rargs_input[*]}"
}

# @cmd Remove directory
# @arg path! Directory path
# @flag -f --force Remove even if not empty
remove() {
	echo "${rargs_input[*]}"
}
