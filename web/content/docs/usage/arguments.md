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

To define arguments, use the `@arg` tag.

```bash
#!/usr/bin/env bash
# @name arguments
# @description Sample application with arguments

# @cmd Greeting function
# @arg name
greet() {
  echo "Hello, $rargs_name!"
}
```

This script defines a single argument called `name`. Access the value of this argument with the `$rargs_name` variable.

> All `arguments`, `options`, `flags`, and other **Rargs** related runtime resources are prefixed with `rargs_` to prevent collisions with your script.

After building this script you should be able to call the `greet` command like this:

```bash
$ ./arguments greet "John Doe"
"Hello, John Doe!"
```

You can also provide an argument description to indicate to the user what the argument represents.

```bash
# @cmd Greeting function
# @arg name Name of the person to greet.
greet() {
  echo "Hello, $rargs_name!"
}
```

> The _description_ is optional but highly encouraged. It makes you script easier to work with, and improved the output of the `usage`.

### Required Arguments

By default, arguments are _optional_. To mark an argument as _required_, append a `!` to its name.

```bash
# @cmd Greeting function
# @arg name!
greet() {
  echo "Hello, $rargs_name!"
}
```

If you don't provide a required argument when calling the script, **Rargs** will throw an error.

### Default Values

Use the `=` operator to define a default value for your argument.

```bash
# @cmd Greeting function
# @arg name=World
greet() {
  echo "Hello, $rargs_name!"
}
```

You can provide values with spaces by enclosing the text in quotes.

```bash
# @arg title="Rargs Example"
```

## Configuration

| Tag                                     | Description                                                                                                           |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `<name> <description>`                  | Defines an optional argument.                                                                                         |
| `<name>! <description>`                 | Defines a required argument.                                                                                          |
| `<name>=foo <description>`              | Defines an argument with a default value.                                                                             |
| `<name>="foo bar" <description>`        | Defines an argument with a default value that includes spaces.                                                        |
| `<name>[foo\|bar\|baz] <description>`   | Defines an argument that accepts only a list of pre-defined values.                                                   |
| `<name>[=foo\|bar\|baz] <description>`  | Defines an argument that accepts a list of pre-defined values with a default value.                                   |
| `<name>* <description>`                 | Defines an optional multiple argument.                                                                                |
| `<name>+ <description>`                 | Defines a required multiple argument.                                                                                 |
| `<name>*[=a\|b\|c] <description>`       | Defines an optional argument that supports predefined values, allows multiple values, has a default, and is optional. |
| `<name>*[=a\|b\|c] <description>`       | Defines a required argument that supports predefined values, allows multiple values, has a default, and is optional.  |
| `<name> <VALUE_NOTATION> <description>` | Defines an argument with a custom value notation.                                                                     |

Some important notes:

- Argument descriptions can include spaces and ANSI escape sequences.
- Use double quotes to set default values that contain spaces.
- The default value for an argument that only supports a list of pre-defined values should be the first value in the list.
- The values for _multiple_ arguments are stored inside a bash array.

> The current implementation of `multiple` arguments requires all positional arguments to be _required_. This is a known issue that will be addressed in future versions.

## Example

```bash
#!/usr/bin/env bash
# shellcheck disable=SC2154
# @name example
# @version 0.1.0
# @description A rargs script template

# @cmd A command with all the valid argument configurations.
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
arguments() {
  echo "Hello world"
}
```

---

Next, we'll learn how to define `options` in your **Rargs** scripts.

[Show me how `options` work →](../../usage/options)
