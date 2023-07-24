#!/usr/bin/env bash
# @name basic
# @description Basic bash script example
# @version 1.0.0
# @author Guzmán Monné, Foo, Bar
# @help I'm not convinced about the usefulness of this script, but it's just an example
# @default greet
# @flag -v --verbose Verbose mode
# @option -z --options-with-required![one|two|three] Option with values and required
# @option -R --options-with-multiple*[one|two|three] Option with values and multiple
# @option -M --options-with-multiple-and-required+[one|two|three] Option with values, required, and multiple

# @cmd Greet the user
# @flag -v --verbose Verbose mode
# @option -r --required! Required option
# @option -m --multiple* Multiple option
# @option -d --default="Default value" Default value option
# @option -V --option-with-value-notation <FOO> Option with value notation
# @option -p --multiple-and-required+ Multiple and required option
# @option -o --options[one|two|three] Option with values
# @option -O --options-with-default[=one|two|three] Option with values and default
# @arg required! Required positional argument
# @arg multiple* Multiple positional argument
# @arg default="Default value" Default value positional argument
# @arg multiple-and-required+ Multiple and required positional argument
# @arg p0 <FOO> Positional Argument with value notation
# @arg p1[one|two|three] Positional Argument with values
# @arg p2[=one|two|three] Positional Argument with values and default
foo() {
  echo "${rargc[@]}"
}
