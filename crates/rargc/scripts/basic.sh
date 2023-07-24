#!/usr/bin/env bash
# @name basic
# @description Basic bash script example
# @version 1.0.0
# @author Guzmán Monné, Foo, Bar
# @help I'm not convinced about the usefulness of this script, but it's just an example

# @cmd Greet the user
# @option -g --greeting="Hello" Greeting to use
# @arg -u --user User name
greet() {
  local greeting="${rargc["greeting"]}"
  local user="${rargc["user"]}"
  echo "$greeting $user"
}
