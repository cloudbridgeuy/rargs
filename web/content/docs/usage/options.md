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

Options provide a more advanced method to pass information to your script. They are not positional, meaning that their order does not matter, but they must be prefixed with their respective names. With **Rargs**, you can streamline the process of parsing these options and define _required_, _multiple_, _pre-defined_ values, or a combination of all for your options.

## Basic Usage

To define options, use the `@option` tag. Here's an example:

```bash
#!/usr/bin/env bash
# @name options
# @description Sample application with options

# @cmd Greeting function
# @option --name The name of the person to greet
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
```

By default, options are optional. To mark an option as _required_, append a `!` to its name. Here's an example:

```bash
# @cmd Greeting function
# @option --name!
greet() {
  echo "Hello, $rargs_name!"
}
```

You can also provide an argument description to indicate to the user what the argument represents.

```bash
# @cmd Greeting function
# @option --name! Name of the person to greet.
greet() {
  echo "Hello, $rargs_name!"
}
```

> The _description_ is optional but highly encouraged. It makes your script easier to work with and improves the output of the `usage`.

### Required Options

By default, `options` are _optional_. To mark an `option` as _required_, append a `!` to its name.

```bash
# @cmd Greeting function
# @option --name!
greet() {
  echo "Hello, $rargs_name!"
}
```

If you don't provide a required argument when calling the script, **Rargs** will throw an error.

### Short names

When defining your options you can also provide a short name to reference the option.

```bash
# @cmd Greeting function
# @option -n --name!
greet() {
  echo "Hello, $rargs_name!"
}
```

You can then reference the `name` option using its long and short form (`--name` or `-n`).

### Default Values

Use the `=` operator to define a default value for your option.

```bash
# @cmd Greeting function
# @option --name=World
```

You can provide values with spaces by enclosing the text in quotes.

```bash
# @option --title="Rargs Example"
```

### Pre-defined Values

If your options should accept only a predefined list of values, you can use the `[]` operator with a list of values separated by a pipe operator (`|`).

```bash
# @option --categories[foo|bar|"foo bar"]
```

> As the example shows, you can use quotes (`"`) to define strings that include white space.

If one of those values should be used as default, set it as the first element of the list with the `=` operator in front.

```bash
# @option --categories[=foo|bar|"foo bar"]
```

### Value Notation

**Rargs** will do its best to create an easy-to-read usage output for your scripts, indicating the requirements of the options through different presentations of a value notation. A value notation is a keyword used to describe the value of an option, accompanied by additional operators like `<>`, `[]`, and `...` to indicate if the argument is `required`, `optional`, or `multiple`.

The value notation will be represented by default as the name of the argument in uppercase, but you can force **Rargs** to use a custom value using the `<>` operator with the name that should be used.

```bash
# @option --bucket <AWS_S3_BUCKET>
```

The output of this argument will look something like this:

```txt
cat <<-EOF | rargs run - -h
#!/usr/bin/env bash
# @name value-notation
# @option --bucket <AWS_S3_BUCKET> AWS S3 Bucket.
# @option --region[=us-east-1|us-west-2] <AWS_REGION> AWS Region.
root() {
  # pass
}
EOF

Usage:
  value-notation [OPTIONS]
  value-notation -h|--help

Options:
  --bucket [<AWS_S3_BUCKET>]
    AWS S3 Bucket.
  --region [<AWS_REGION>]
    AWS Region.
    [@default us-east-1, @choices us-east-1|us-west-2]
  -h --help
    Print help
```

### Multiple Values

**Rargs** supports defining zero-or-more or one-or-more values for your options when using the `*` or `+` operator respectively.

```bash
# @option --zero-or-more*
# @option --one-or-more+
```

Default and predefined values can be used with the `*` and `+` operators.

```bash
# @option --zero-or-more*[foo|bar|baz]
# @option --one-or-more+[=foo|bar|baz]
```

The difference between the two operators is that the `+` operator requires at least one value to be present when invoking the command.

```txt
cat <<-EOF | rargs run - -h
#!/usr/bin/env bash
# @name multiple
# @option --zero-or-more* Zero or more arguments.
root() {
  # pass
}
EOF

Usage:
  multiple [OPTIONS]
  multiple -h|--help

Options:
  --zero-or-more [<ZERO-OR-MORE>]
    Zero or more arguments.
    [@multiple]
  -h --help
    Print help
```

**One or more values**:

```txt
cat <<-EOF | rargs run - -h
#!/usr/bin/env bash
# @name multiple
# @option --one-or-more+ One or more arguments.
root() {
  # pass
}
EOF

Usage:
  multiple --one-or-more <ONE-OR-MORE> [OPTIONS]
  multiple -h|--help

Options:
  --one-or-more <ONE-OR-MORE>
    One or more arguments.
    [@multiple]
  -h --help
    Print help
```

## Configuration

Every operator, except the `*` and `+`, can be combined together.

| Operator                              | Description                      |
| ------------------------------------- | -------------------------------- |
| `# @option --name`                    | Optional.                        |
| `# @option -n --name`                 | Short and Long form.             |
| `# @option --name!`                   | Required.                        |
| `# @option --name*`                   | Zero or more.                    |
| `# @option --name+`                   | One or more.                     |
| `# @option --name Option description` | With description.                |
| `# @option --name=foo`                | With default.                    |
| `# @option --name[foo\|bar\|baz]`     | Pre-defined values.              |
| `# @option --name[=foo\|bar\|baz]`    | Pre-defined values with default. |
| `# @option --name <VALUE_NOTATION>`   | Custom value notation.           |

Check the examples directory to look for ways to combine these operators.

---

Next, we'll learn how to define `flags` in your **Rargs** scripts.

[Show me how options work →](../../usage/options)
