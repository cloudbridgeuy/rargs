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

After building this script, you should be able to call the `greet` command like this:

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

> The _description_ is optional but highly encouraged. It makes your script easier to work with and improves the output of the `usage`.

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

### Pre-defined Values

If your arguments should only accept a known list of values, you can use the `[]` operator with a list of values separated by the pipe operator (`|`).

```bash
# @arg categories[foo|bar|"foo bar"]
```

> As the example shows, you can use quotes (`"`) to define strings that include whitespace.

If one of those values should be used as default, set it as the first element of the list with the `=` operator in front.

```bash
# @arg categories[=foo|bar|"foo bar"]
```

### Value Notation

**Rargs** will do its best to create an easy-to-read usage output for your scripts, indicating the requirements of the arguments through different presentations of a value notation. A value notation is a keyword used to describe the value of an argument, accompanied by additional operators like `<>`, `[]`, and `...` to indicate if the argument is `required`, `optional`, or `multiple`.

The value notation will be represented by default as the name of the argument in uppercase, but you can force **Rargs** to use a custom value using the `<>` operator with the name that should be used.

```bash
# @arg bucket <AWS_S3_BUCKET>
```

The output of this argument will look something like this:

```txt
cat <<-EOF | rargs run - -h
#!/usr/bin/env bash
# @name value-notation
# @arg bucket <AWS_S3_BUCKET> AWS S3 Bucket.
# @arg region[=us-east-1|us-west-2] <AWS_REGION> AWS Region.
EOF

Usage:
  value-notation [OPTIONS] AWS_S3_BUCKET [AWS_REGION]
  value-notation -h|--help

Arguments:
  AWS_S3_BUCKET
    AWS S3 Bucket.
    [@required]
  AWS_REGION
    AWS Region.
    [@default us-east-1, @choices us-east-1|us-west-2]

Options:
  -h --help
    Print help
```

### Multiple Values

**Rargs** supports defining zero-or-more or one-or-more values for your arguments when using the `*` or `+` operator respectively.

```bash
# @arg zero-or-more*
# @arg one-or-more+
```

Default and predefined values can be used with the `*` and `+` operators.

```bash
# @arg zero-or-more*[foo|bar|baz]
# @arg one-or-more+[=foo|bar|baz]
```

The difference between the two operators is that the `+` operator requires at least one value to be present when invoking the command.

**Zero or more values**:

```txt
cat <<-EOF | rargs run - -h
#!/usr/bin/env bash
# @name multiple
# @arg zero-or-more* Zero or more arguments.
EOF

Usage:
  multiple [OPTIONS] [ZERO-OR-MORE...]
  multiple -h|--help

Arguments:
  ZERO-OR-MORE
    Zero or more arguments.
    [@multiple]

Options:
  -h --help
    Print help
```

**One or more values**:

```txt
cat <<-EOF | rargs run - -h
#!/usr/bin/env bash
# @name multiple
# @arg one-or-more+ One or more arguments.
EOF

Usage:
  multiple [OPTIONS] ONE-OR-MORE...
  multiple -h|--help

Arguments:
  ONE-OR-MORE
    One or more arguments.
    [@required, @multiple]

Options:
  -h --help
    Print help
```

## Configuration

Every operator, except the `*` and `+`, can be combined together.

| Operator                           | Description                      |
| ---------------------------------- | -------------------------------- |
| `# @arg name`                      | Optional.                        |
| `# @arg name!`                     | Required.                        |
| `# @arg name*`                     | Zero or more.                    |
| `# @arg name+`                     | One or more.                     |
| `# @arg name Argument description` | With description.                |
| `# @arg name=foo`                  | With default.                    |
| `# @arg name[foo\|bar\|baz]`       | Pre-defined values.              |
| `# @arg name[=foo\|bar\|baz]`      | Pre-defined values with default. |
| `# @arg name <VALUE_NOTATION>`     | Custom value notation.           |

Check the examples directory to look for ways to combine these operators.

---

Next, we'll learn how to define `options` in your **Rargs** scripts.

[Show me how `options` work →](../../usage/options)
