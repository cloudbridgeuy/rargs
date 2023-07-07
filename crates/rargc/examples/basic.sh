#!/usr/bin/env bash
# @name basic
# @description Basic bash script example
# @version 1.0.0

# @cmd Greet the user
# @option -g --greeting="Hello" Greeting to use
# @arg -u --user User name
greet() {
  local greeting="${rargc["greeting"]}"
  local user="${rargc["user"]}"
  echo "$greeting $user"
}
