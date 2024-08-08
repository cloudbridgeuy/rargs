#!/usr/bin/env bash

# @name minimal
# @version 0.0.1
# @description Flags examples
# @flag --falsy Falsy flag
# @flag --truthy=1 Truthy flag
# @flag -s --shorty=1 Shorty flag

root() {
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
}
