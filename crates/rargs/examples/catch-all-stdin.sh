#!/usr/bin/env bash
# @name catch-all-stdin
# @version 0.0.1
# @description Catch All with stdin input

# @option -f --format[=json|csv] Specify the file format
# @any <FILE> Path to one or more files. Reads from stdin if empty or "-"
# @example Read two csv files $ file1 file2 --format csv
# @example Read two json files $ --format json file1 file2
# @example Read a file from stdin $ -f csv < file1
# @example Read a file from stdin using - $ -f csv - < file1
root() {
	inspect_args

	# If rargs_other_args[0] is "-" or empty, read from stdin
	if [[ "${rargs_other_args[0]}" == "-" || -z "${rargs_other_args[0]}" ]]; then
		read_stdin
	else
		read_files
	fi

	printf "\nCollected file content:\n%s\n" "$content"

	# Read contents of the provided file(s)
	content=""
	for file in "${rargs_other_args[@]}"; do
		content+=$(cat "$file")
		content+=$'\n'
	done

	# Read from stdin if no file is provided
	if [[ -z "$content" ]]; then
		content=$(cat -)
	fi
}

read_stdin() {
	# Read from stdin
	content=$(cat -)
}

read_files() {
	# Read contents of the provided file(s)
	content=""
	for file in "${rargs_other_args[@]}"; do
		content+=$(cat "$file")
		content+=$'\n'
	done
}
