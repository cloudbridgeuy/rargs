#!/usr/bin/env bash
# @name minus-v
# @version 0.0.1
# @description Example that replaces the default behavior of -v and -h
# @flag -v --verbose Show verbose output
# @option -h --host Show verbose output
# @example Set host $ -h|--host localhost
# @example Set verbose mode on $ -v|--verbose

root() {
  inspect_args
}
