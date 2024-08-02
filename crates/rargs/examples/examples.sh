#!/usr/bin/env bash
# @name examples
# @description A script with long examples
# @version 0.1.0
# @example The simplest way of providing a prompt $ "Write me a poem about the scripting language bash."
# @example Add a system prompt $ --role system --content "You are acting as an ironical writer." "Write me a poem about the scripting language bash."
# @example Add a previous assistant message $ --role system --content "You are acting as an ironical writer." --role assistant --content "Hello my dear user! How can I assist you today!?." "Write me a poem about the scripting language bash."
root() {
	echo Hello World
}
