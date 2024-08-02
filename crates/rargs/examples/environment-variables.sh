#!/usr/bin/env bash
# @name environment-variables
# @version 0.0.1
# @description Sample application that requires environment variables
# @env DEBUG Enable debug mode

# @cmd Verify your user
# @option -s --secret! Your secret key
# @option -m --multiple* Multiple values
# @env ENVIRONMENT! Your environment
# @env SECRET:secret Your secret key
# @env MULTIPLE:multiple Multiple values are also supported
verify() {
	echo "${rargs_input[*]}"
}

# @env ENVIRONMENT Your environment
root() {
	echo "${rargs_input[*]}"
	echo "DEBUG: ${DEBUG:-}"
	echo "ENVIRONMENT: ${ENVIRONMENT:-}"
	echo "SECRET: ${SECRET:-}"
}
