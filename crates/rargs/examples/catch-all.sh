#!/usr/bin/env bash
# @name catch-all
# @version 0.0.1
# @description Catch All example
# @flag -d --debug Debug mode

# @cmd Required additional arguments
# @arg message Message
# @any!
required() {
	if [[ -n "$rargs_debug" ]]; then
		set -x
	fi

	echo "${rargs_input[*]}"
}

# @cmd Command with a simple optional argument
# @arg message Message
# @any! <VALUE_NOTATION> Required additional arguments
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
