#!/usr/bin/env bash
# shellcheck disable=SC2154
# @name private
# @version 0.0.1
# @description Sample application with private commands

# @cmd Connect to the metaverse
# @alias c
# @option -p --protocol![ftp|ssh] Protocol to use for connection
# @flag -v --verbose Verbose output
# @arg host! Hostname to connect to
connect() {
	if [[ "$rargs_protocol" == "ftp" ]]; then
		connect-ftp --username ftp_user "$rargs_host"
	else
		connect-ssh --username ssh_ser "$rargs_host"
	fi
}

# @cmd Connect using FTP
# @private
# @option --username Username
# @arg host! Hostname to connect to
connect-ftp() {
	echo "FTP!!!"
	echo "${rargs_input[*]}"
}

# @cmd Connect using SSH
# @private
# @option --username Username
# @arg host! Hostname to connect to
connect-ssh() {
	echo "SSH!!!"
	echo "${rargs_input[*]}"
}
