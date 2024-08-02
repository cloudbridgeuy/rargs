#!/usr/bin/env bash
# @name environment-variables
# @version 0.0.1
# @description Sample application that requires environment variables
# @rule use-global-envs-for-root

# @cmd Verify your user
# @option -s --secret! Your secret key
# @env SECRET:secret Your secret key
verify() {
	echo "${rargs_input[*]}"
}

# @env ENVIRONMENT! Your environment
root() {
	echo "${rargs_input[*]}"
}
