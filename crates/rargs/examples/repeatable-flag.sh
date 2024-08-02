#!/usr/bin/env bash
# @name repeatable-arg
# @description Sample application to demonstrate the use of repeatable flags
# @version 0.0.1

# @option -d --data+ Provide data values
# @flag -v --verbose* Set verbosity level
# @example Multiple data values $ -d 1 -d 2 -d 3
# @example Multiple data values with verbosity enabled $ -d 1 -d 2 -d 3 -v
# @example Multiple data values with high verbosity enabled $ -d 1 -d 2 -d 3 -vvv
root() {
	echo "Data elements:"
	for i in "${rargs_data[@]}"; do
		echo "$i"
	done

	# The --verbose arg will contain the number of times it was used by the user
	echo ""
	echo "Verbosity level: $rargs_verbose"
	echo ""

	echo "${rargs_input[*]}"
}
