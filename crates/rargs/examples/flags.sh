#!/usr/bin/env bash

# @name minimal
# @version 0.0.1
# @description Flags examples

# @@ This script shows different ways of working with truthy or falsy flags.
# @@ By default, flags are truthy if they are passed as an option, and empty
# @@ otherwise. If you would like your flag to be true by default, provide a
# @@ default value when defining the flag. This will enable the ability to
# @@ provide a negated version of the flag, using a '--no-' or '-n-' prefix
# @@ for the long and short version of the flag.
# @@
# @@ This script exposes three flags that showcase this behavior.
# @@
# @@   1. \`falsy\` : Default flag that is set to \`false\` by default.
# @@   2. \`truthy\`: Flag with a default value that is considered \`truthy\`
# @@                by default.
# @@   3. \`shorty\`: Same as the \`truthy\` flag but configured to also use
# @@                a short version of the flag.
#
# @flag --falsy Falsy flag
# @flag --truthy=1 Truthy flag
# @flag -s --shorty=1 Shorty flag
#
# @example Set the \`falsy\` flag to \`true\` $ --falsy
# @example Set the \`truthy\` flag to \`false\` $ --no-truthy
# @example Set the \`shorty\` flag to \`false\` $ --no-shorty
# @example Set the \`shorty\` flag to \`false\` using the short name $ -n-s
#
# @@
# @@ You can also define flags to be multiple, in which case the value of
# @@ calling the flag multiple time will be an integer with the total count
# @@ flags provided to the command.
#
# @flag -v --verbose* Support multiple verbose flags
#
# @example Return the total count of the multiple flag $ -vvv
# @example Same example as before but with the full value $ --verbose --verbose --verbose
#
root() {
	if [[ -z "$rargs_verbose" ]]; then
		if [[ -n "$rargs_falsy" ]]; then
			echo "falsy == $rargs_falsy"
		else
			echo "falsy == false"
		fi
		if [[ -n "$rargs_truthy" ]]; then
			echo "truthy == $rargs_truthy"
		else
			echo "truthy == false"
		fi
		if [[ -n "$rargs_shorty" ]]; then
			echo "shorty == $rargs_shorty"
		else
			echo "shorty == false"
		fi
	else
		echo "verbose == $rargs_verbose"
	fi
}
