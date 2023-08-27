#!/usr/bin/env bash

# @name args
# @description Commands with arguments example
# @version 0.1.0
# @default simplest

# @cmd Simplest
# @arg simplest Simplest positional argument definition.
simplest() {
  :;
}

# @cmd Choices
# @arg choices[a|b|c] Positional argument with choices.
choices() {
  :;
}

# @cmd Required
# @arg required! Required positional argument.
required() {
  :;
}

# @cmd Default
# @arg default=default_value Positional argument with default value.
default() {
  :;
}

# @cmd Value Notation
# @arg value <VALUE_NOTATION> Positional argument with value notation.
value() {
  :;
}

# @cmd Choice + Default
# @arg choice_default[=a|b|c] Positional argument with choices and default value.
choice_default() {
  :;
}

# @cmd Choice + Default + Value Notation
# @arg choice_default_value[=a|b|c] <VALUE_NOTATION> Positional argument with choices, default value and value notation.
choice_default_value() {
  :;
}
