+++
title = "Arguments"
description = "Learn how to use arguments in your Rargs scripts."
date = 2023-09-27T08:20:00+00:00
updated = 2023-09-27T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

Arguments are the simplest way to pass information to your script. They are positional, meaning the order you pass them in is important. With **Rargs**, you can streamline the process of parsing these arguments and define _required_, _multiple_, and _pre-defined_ values for your arguments.

## Basic Usage

To define arguments, use the `@arg` tag. Here's an example:

```bash
#!/usr/bin/env bash
# @name arguments
# @description Sample application with arguments

# @cmd Greeting function
# @arg name The name of the person to greet
greet() {
  echo "Hello, $rargs_name!"
}
```

This script defines a single argument called `name`. Access the value of this argument with the `$rargs_name` variable.

> All `arguments`, `options`, `flags`, and other **Rargs** related runtime resources are prefixed with `rargs_` to prevent collisions with your script.

By default, arguments are optional. To mark an argument as _required_, append a `!` to its name. Here's an example:

```bash
# @cmd Greeting function
# @arg name! The name of the person to greet
greet() {
  echo "Hello, $rargs_name!"
}
```

If you don't provide a required argument when calling the script, **Rargs** will throw an error.

Below is a list of all available `comment decorators` for arguments:

| Tag                                          | Description                                                                                                           |
| -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `@arg <name> <description>`                  | Defines an optional argument.                                                                                         |
| `@arg <name>! <description>`                 | Defines a required argument.                                                                                          |
| `@arg <name>=foo <description>`              | Defines an argument with a default value.                                                                             |
| `@arg <name>="foo bar" <description>`        | Defines an argument with a default value that includes spaces.                                                        |
| `@arg <name>[foo\|bar\|baz] <description>`   | Defines an argument that accepts only a list of pre-defined values.                                                   |
| `@arg <name>[=foo\|bar\|baz] <description>`  | Defines an argument that accepts a list of pre-defined values with a default value.                                   |
| `@arg <name>* <description>`                 | Defines an optional multiple argument.                                                                                |
| `@arg <name>+ <description>`                 | Defines a required multiple argument.                                                                                 |
| `@arg <name>*[=a\|b\|c] <description>`       | Defines an optional argument that supports predefined values, allows multiple values, has a default, and is optional. |
| `@arg <name>*[=a\|b\|c] <description>`       | Defines a required argument that supports predefined values, allows multiple values, has a default, and is optional.  |
| `@arg <name> <VALUE_NOTATION> <description>` | Defines an argument with a custom value notation.                                                                     |

Some important notes:

1. Argument descriptions can include spaces and ANSI escape sequences.
2. Use double quotes to set default values that contain spaces.
3. Escape the `|` character when defining a list of pre-defined values.
4. The default value for an argument that only supports a list of pre-defined values should be the first value in the list.

> The current implementation of `multiple` arguments requires all positional arguments to be _required_. This is a known issue that will be addressed in future versions.

Here's an example of a script using all the available `comment decorators` for arguments:

```bash
#!/usr/bin/env bash
# shellcheck disable=SC2154
# @name example
# @version 0.1.0
# @description A rargs script template

# @cmd Subcommand example
# @arg optional-argument Optional argument
# @arg required-argument! Required argument
# @arg default-argument=foo Default argument
# @arg default-argument-with-spaces="foo bar" Default argument with spaces
# @arg with-options[foo|bar|baz] Argument with options
# @arg with-options-and-default[=foo|bar|baz] Argument with options and default
# @arg with-different-value-notation <VALUE_NOTATION> Argument with a different value notation
# @arg with-multiple-optional* Multiple optional argument
## The next lines are commented out because `rargs` does not support handling more than one multiple argument.
###arg with-multiple-required+ Multiple required argument
###arg with-multiple-optional-and-options*[foo|bar|baz] Multiple optional argument with options
###arg with-multiple-optional-options-and-default*[=foo|bar|baz] Multiple optional argument with options and default
###arg with-multiple-required-and-options+[foo|bar|baz] Required multiple argument with options
###arg with-multiple-required-options-and-default+[=foo|bar|baz] Required multiple argument with options and default
subcommand() {
        echo "optional-argument: $rargs_optional_argument"
        echo "required-argument: $rargs_required_argument"
        echo "default-argument: $rargs_default_argument"
        echo "default-argument-with-spaces: $rargs_default_argument_with_spaces"
        echo "with-options: $rargs_with_options"
        echo "with-options-and-default: $rargs_with_options_and_default"
        echo "with-different-value-notation: $rargs_with_different_value_notation"
        echo "with-multiple-optional: ${rargs_with_multiple_optional[*]}"
        echo "with-multiple-required: ${rargs_with_multiple_required[*]}"
        echo "with-multiple-optional-and-options: ${rargs_with_multiple_optional_and_options[*]}"
        echo "with-multiple-optional-options-and-default: ${rargs_with_multiple_optional_options_and_default[*]}"
        echo "with-multiple-required-and-options: ${rargs_with_multiple_required_and_options[*]}"
        echo "with-multiple-required-options-and-default: ${rargs_with_multiple_required_options_and_default[*]}"
}
```

---

Next, we'll learn how to define options in your **Rargs** scripts.

[Show me how options work →](../../usage/options)
