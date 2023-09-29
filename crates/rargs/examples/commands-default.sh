#!/usr/bin/env bash
# @name commands
# @version 0.0.1
# @description Sample application that uses the default command option
# @example Run the default command $
# @example Print the global help $ --help
# @example Download something $ download something
# @example Upload something $ upload something
# @example Upload something using the default command $ something
# @default upload
# @rule no-force-default

# @cmd Upload a file
# @alias u
# @flag -f --force Force upload
# @arg source! URL to download from
upload() {
  inspect_args
}

# @cmd Download a file
# @alias d
# @arg source! URL to download from
download() {
  inspect_args
}



