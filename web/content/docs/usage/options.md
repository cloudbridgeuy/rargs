+++
title = "Options"
description = "Learn how to use options in your Rargs scripts."
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

Options are a more advanced way to pass information to your script. They are not positional, meaning the order in which you pass them doesn't matter. With **Rargs**, you can streamline the process of parsing these options and define _required_, _multiple_, and _pre-defined_ values for your options.

## Basic Usage

To define options, use the `@option` tag. Here's an example:

```bash
#!/usr/bin/env bash
# @name options
# @description Sample application with options

# @cmd Greeting function
# @option -n --name The name of the person to greet
greet() {
  echo "Hello, $rargs_name!"
}
```

This script defines a single option called `name`. You can access the value of this option with the `$rargs_name` variable.

> All `arguments`, `options`, `flags`, and other **Rargs** related runtime resources are prefixed with `rargs_` to prevent collisions with your script.

After building this script, you should be able to call the `greet` command like this:

```bash
$ ./options greet --name "John Doe"
Hello, John Doe!

$ ./options greet -n "Jane Doe"
Hello, Jane Doe!
```

By default, options are optional. To mark an option as _required_, append a `!` to its name. Here's an example:

```bash
# @cmd Greeting function
# @option -n --name! The name of the person to greet
greet() {
  echo "Hello, $rargs_name!"
}
```

If you don't provide a required option when calling the script, **Rargs** will throw an error.

You can also opt out of defining a short option by omitting the `-` prefix, but you always need to provide the long name of the option.

```bash
# @cmd Greeting function
# @option --name! The name of the person to greet
greet() {
  echo "Hello, $rargs_name!"
}
```

## Configuration

| Tag                                     | Description                                                                       |
| --------------------------------------- | --------------------------------------------------------------------------------- |
| `<short> <long> <description>`          | Defines an optional option with a short and long option.                          |
| `<name>! <description>`                 | Defines a required option.                                                        |
| `<name>=foo <description>`              | Defines an option with a default value.                                           |
| `<name>="foo bar" <description>`        | Defines an option with a default value that includes spaces.                      |
| `<name>[foo\|bar\|baz] <description>`   | Defines an option that accepts only a list of pre-defined values.                 |
| `<name>[=foo\|bar\|baz] <description>`  | Defines an option that accepts a list of pre-defined values with a default value. |
| `<name>* <description>`                 | Defines an optional multiple option.                                              |
| `<name>+ <description>`                 | Defines a required multiple option.                                               |
| `<name> <VALUE_NOTATION> <description>` | Defines an option with a custom value notation.                                   |
| `<name>*=a <description>`               | Defines an optional option that accepts multiple values and has a default.        |
| `<name>+=a <description>`               | Defines a required option that accepts multiple values and has a default.         |
| `<name>*[a\|b\|c] <description>`        | Defines an optional option that accepts multiple values and has a default.        |
| `<name>+[a\|b\|c] <description>`        | Defines a required option that accepts multiple values and has a default.         |
| `<name>*[=a\|b\|c] <description>`       | Defines an optional option that accepts multiple values and has a default.        |

Some important notes:

- Argument descriptions can include spaces and ANSI escape sequences.
- Multiple options expect the option to be provided before each value. For example: `-m foo -m bar -m baz`.
- The values for _multiple_ options are stored in an array.
- The default value for an option that only supports a list of pre-defined values should be the first value in the list.

## Example

```bash
# @cmd A command with all the valid option configurations.
# @option -s --long Optional short and long option.
# @option --required! Required option.
# @option --default=foo Default option.
# @option --multiple* Optional multiple option.
# @option --multiple-required+ Required multiple option.
# @option --value-notation <VALUE_NOTATION> Option with a different value notation.
# @option --with-options[foo|bar|baz] Option with options.
# @option --with-options-and-default[=foo|bar|baz] Option with options and a default.
# @option --with-options-and-required![foo|bar|baz] Required option with options.
# @option --with-options-and-multiple*[foo|bar|baz] Option with options and multiple.
# @option --with-options-and-multiple-required+[foo|bar|baz] Required option with options and multiple.
options() {
        echo "optional: $rargs_long"
        echo "required: $rargs_required"
        echo "default: $rargs_default"
  # **Important**: Remember that multiple options are stored in an array!
        echo "multiple: ${rargs_multiple[*]}"
        echo "multiple-required: ${rargs_multiple_required[*]}"
        echo "value-notation: $rargs_value_notation"
        echo "with-options: $rargs_with_options"
        echo "with-options-and-default: $rargs_with_options_and_default"
        echo "with-options-and-required: $rargs_with_options_and_required"
        echo "with-options-and-multiple: ${rargs_with_options_and_multiple[*]}"
        echo "with-options-and-multiple-required: ${rargs_with_options_and_multiple_required[*]}"
}
```

---

Next, we'll learn how to define `flags` in your **Rargs** scripts.

[Show me how options work →](../../usage/options)
