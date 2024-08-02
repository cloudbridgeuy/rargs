#!/usr/bin/env bash
# @name file
# @version 0.0.1
# @description File commands

# @cmd Show file contents
# @arg path! Path to file
show() {
	echo "${rargs_input[*]}"
}

# @cmd Edit the file
# @arg path! Path to file
remove() {
	echo "${rargs_input[*]}"
}
