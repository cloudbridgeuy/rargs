#!/usr/bin/env bash
# @name whitelist
# @description Sample showing the use of arg and option whitelist (allowed values)
# @version 0.0.1

# @option -u --user![user|admin] <USER> User name
# @option -p --protocol[=ssh|ftp|http] <PROTOCOL> Protocol to connect with
# @arg region![eu|us] <REGION> Region to connect to
# @arg environment[=development|staging|production] <ENVIRONMENT> Environment to connect to
root() {
	echo "${rargs_input[*]}"
}
