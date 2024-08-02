#!/usr/bin/env bash
# @name catch-all-global
# @version 0.0.1
# @description Catch All global example
# @flag -d --debug Debug mode
# @any! <GLOBAL_REQUIRED_ADDITIONAL_ARGUMENTS> Required additional arguments

# @cmd Command with a simple optional argument
# @arg message Message
# @any! <VALUE_NOTATION> Command specific required additional arguments
no-multiple() {
	if [[ -n "$rargs_debug" ]]; then
		set -x
	fi

	echo "${rargs_input[*]}"
}

# @cmd With a multiple required argument
# @arg message+ Message
# @any Optional additional arguments
multiple() {
	if [[ -n "$rargs_debug" ]]; then
		set -x
	fi

	echo "${rargs_input[*]}"
}

# @cmd Any arguments without description
# @any
other() {
	if [[ -n "$rargs_debug" ]]; then
		set -x
	fi

	echo "${rargs_input[*]}"
}

root() {
	if [[ -n "$rargs_debug" ]]; then
		set -x
	fi

	echo "${rargs_input[*]}"
}
