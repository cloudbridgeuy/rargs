+++
title = "Flags"
description = "Learn how to use flags in your Rargs scripts."
date = 2024-08-07T08:20:00+00:00
updated = 2024-08-07T08:20:00+00:00
draft = false
weight = 30
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

Flags are simple version of options that don't take values, but can be called multiple times in order to change the emphasis of the flag.

## Basic Usage

To define a flag, use the `@flag` tag. Here's an example:

```bash
#!/usr/bin/env bash
# @name flags
# @description Sample application with flags

# @cmd Greeting function
# @flag --verbose Verbosity level
greet() {
  if [[ -n "$rargs_verbose" ]]; then
    echo "Hello, World!"
  fi
}
```

This script defines a single flag called `verbose` that controls the behavior of the script.

> All `arguments`, `options`, `flags`, and other **Rargs** related runtime resources are prefixed with `rargs_` to prevent collisions with your script.

After building this script, calling the `greet` command like with the `--verbose` flag, you will see `Hello, World!` printed to `stdout`.

```bash
$ ./flags greet --verbose
Hello, John Doe!
```

By default, flags are `falsy` and are set to `true` when defined. If you want the opposite behavior you can give the flag a default `truthy` value. Then, you would disable this value by running the `flag` with a `no` prefix in front of the flag name.

```bash
# @cmd Greeting function
# @flag --verbose=true
greet() {
  if [[ -z "$rargs_verbose" ]]; then
    echo "Hello, World!"
  fi
}
```

With this version of the script, you would need to provide the `--no-verbose` option in order to print something to `stdout`.

```bash
$ ./flags greet --no-verbose
```

> The `truthy` value can be anything but the string: `false`.

### Short names

When defining your flags you can also provide a short name for it.

```bash
# @cmd Greeting function
# @flag -v --verbose
greet() {
  echo "Hello, $rargs_name!"
}
```

You can then reference the `verbose` flag when calling your script using its long and short form (`--verbose` or `-v`).

### Multiple Flags

**Rargs** also support multiple values on flags, though they work differently than `options`. Flags
don't take in values, yet you may want to use them to express different levels of a variable
by calling it more than once. A common pattern is to define a `--verbose` flag that also
supports a short option: `-v`. You can then repeat the `--verbose` or `-v` flag multiple
times to increase the verbosity level of your command.

> Remember you can also concatenate multiple short variables like this `-vvv`.

To mark a `flag` as multiple add the `*` operator to the end of the long definition. Now,
the value of that flag will be an integer value with the count of times the flag
was passed to the command. For example, `-vvv` will set the `rargs_verbose` variable
inside the script to `3`.

```bash
cat <<-'EOF' | rargs run - -vvv
# @flag -v --verbose* Verbosity level
root() {
  for ((i = 1; i <= rargs_verbose; i++)); do
    echo "Verbose #$i"
  done
}
EOF
```

## Configuration

| Operator                            | Description                                        |
| ----------------------------------- | -------------------------------------------------- |
| `# @flag --name`                    | Falsy flag.                                        |
| `# @flag -n --name`                 | Falsy flag with short form.                        |
| `# @flag --name=1`                  | Truthy flag (use it with `--no-` or `-n-` prefix.) |
| `# @flag --name*`                   | Multiple value flag.                               |
| `# @flag --name+`                   | One or more.                                       |
| `# @flag --name Option description` | With description.                                  |

Check the examples directory to look for ways to combine these operators.

---

Next, we'll learn how to use additional features of the framework in your **Rargs** scripts.

[Tell me more →](../../usage/other)
