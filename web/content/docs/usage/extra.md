+++
title = "Extra Features"
description = "Learn how other features exposed by Rargs work"
date = 2024-08-07T08:20:00+00:00
updated = 2024-08-07T08:20:00+00:00
draft = false
weight = 40
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

On this section, we've outlined some extra features exposed by **Rargs** that add some quality
of life improvements, and take you scrtipts to the next level.

## Long Help Output

Most **Rargs** decorators can be used to improve the usage output of you script. Nonetheless
there may be times where you want to include more than a single line of information, or
split the lines between other configuration to better explain the logic behind it.

You can use the `@@` decorator to do exactly this. Every commend decorated with this
operator will be parsed as a single value of the help message, and will be appended appropriately
in the usage output.

```bash
#!/usr/bin/env bash

# @name minimal
# @version 0.0.1
# @description Flags examples

# @@ You can use the @help decorator to split the output of your usage
# @@ message into multiple lines.
# @@
# @@ You can also interleave these lines with other decorators.
# @arg name="World" Name of the entity to greet
#
## Double '#' Can be used to add comments that won't be treated as part
## of the usage output.
#
# @@
# @@ You can continue with the usage text here. Notice how you can implement
# @@ empty new lines by using only the @@ decorator.
root() {
  echo "Hello $rargs_name"
}
```

## Examples

A typical thing to add to a script is some inline examples on how to run the application.
**Rargs** exposes an `@example` decorator that allows you define ways to call your script
and a brief explanation of what they do.

The `@example` decorator expects a brief description of what the example is for and
the command itself, separated by a `$` operator. This value will be replaced in the usage
output with the name of the command, or script depending of the scope.

```bash
# @example Directory command $ dir
# @example Directory help command $ dir -h
# @example Directory file command $ file -h
# @example List directory sub-command $ dir list
# @example List directory sub-command help $ dir list -h
```

These examples will have the following text appear on the usage output, assuming the
script is called `examples`.

```
Examples:
  examples dir
    Directory command
  examples dir -h
    Directory help command
  examples file -h
    Directory file command
  examples dir list
    List directory sub-command
  examples dir list -h
    List directory sub-command help
```

## Any parameters

Use the `@any` decorator to configure the script or a command to parse a dynamic list
of additional arguments. This can be useful when we want to handle parsing the arguments
ourselves, or when we need to pass them to a different script or binary.

You can reference the additional arguments inside your scripts through the `rargs_other_args`
variable, defined as an array of values.

```bash
cat <<-'EOF' | rargs run - -h
# @help Show how to use the @any decorator.
# @any <GIT_ARGUMENTS> Git command arguments and options.
root() {
	echo "Pass the additional arguments to an external app like git."
	git ${rargs_other_args[@]}
}
EOF
```

Just as other arguments decorators, the `@any` decorator accepts a value notation script
and a description to include in the usage output.

## Environment Variables

**Rargs** can help you enforce the existence of pre-defined environment variables, and
allows you to overwrite `options` and `arguments` with the values of environment
variables.

To use this feature use `@env` decorator at the script or command scope.

### Required Environment Variable

Use the `!` operator to mark an environment variable as required.

```bash
# @env REQUIRED_ENVIRONMENT_VARIABLE!
```

### Assign an environment variable to a command or argument

To assign the value of an environment variable to a positional argument or an option,
use the `:` operator followed by the name of the variable you want to assign the value to.

```bash
# @arg positional-argument! Positional argument
# @option -e --example Example Option
# @env POSITIONAL_ARGUMENT:positional-argument
# @env EXAMPLE:example
```

> You can't mark an environment variable as required with the `!` operator when assigning
> its value to another variable. If you need to enforce the requirement of this value
> use the `!` operator on the `@arg` or `@option` definition.

## Dependencies

Most script require external dependencies to be available in the `$PATH` during its
execution. This could be binaries or other scripts. **Rargs** exposes the `@dep` decorator
to ensure that all the required binaries or scripts are available on runtime. It
also allows you to set an error message, which you can use to give the user instructions
on how to install the missing dependency.

```bash
# @dep curl Please install curl with brew install curl or apt install curl
```

If you want to add some colors, add the appropriate ANSI escape codes to the message.

```bash
# @dep curl Please install \e[32mcurl\e[0m with \e[32mbrew install curl\e[0m or \e[32mapt install curl\e[0m
```

You can also provide a list of dependencies as strings separated by a comma (`,`) and
can personalize the message by referring to the missing dependency as `$dep` when
writing your help message.

```bash
# @dep curl,jo Please install \e[32m$dep\e[0m with \e[32mbrew\e[0m or \e[32maapt-get\e[0m
```

## Rules

Even though **Rargs** has a lot of opinions on how to write CLI applications, it still offer
some escape hatches that you can use to tune your script just how you want. Rules are one
big way to do this.

Basically, setting up a rule changes a core behavior of **Rargs** either at the `root` or
`command` scope. You can use it to disable the `--help` flags, or change the way that
it should trigger.

To set a rule use the `@rule` operator followed by the name of the rule you want to enforce.
If you set this inside the scope of an `@cmd` command, then the rule will only be applied
to the command, and not to the entire script.

On this section, we'll outline all the rules available.

### No first option help

By default, **Rargs** expects the `--help` option to be passed as the first argument of
the call. If you add any argument before you'll get an error message like this:

```bash
Invalid option: --help
```

The `no-first-option-help` rule disable this behavior, allowing you to provide the
`--help` option at any place, avorting the execution of the script, to present the
script usage information.

```bash
# @rule no-first-option-help
```

### No force default

This rule disable the `default` assigning mechanics of **Rargs**, letting you mark
an `option` or `argument` as having a default value, yet letting you decide on
your own how this value will be set.

```bash
# @rule no-force-default
```

### Use global deps for root.

Allow the usage of the `@dep` decorator at the global scope to work for scripts
with a single `root` command.

```bash
# @rule use-global-deps-for-root
```

### Use gloable envs for root

Allow the usage of the `@env` decorator at the global scope to work for sciprs
with a single `root` command.

```bash
# @rule use-global-envs-for-root
```

## External Sub-Commands

> **Important**: This is a _WIP_ and it may suffer changes in the future.

Bash scripts tend to be fairly long when we add all this functionality, so it
makes sense that we might want to split the code into more manageable chunks.
The `@sub` decorator attempts to simplify the process of writing a CLI app
in Bash, defined on multiple files.

It works by declaring the `@sub` decorator, followed by a **relative** path
to the script that should be called instead. This next script can be another
**Rargs** powered script, a normal Bash script, a script written on a different
language, or a binary. This script (and all the others defined in the same
manner) will be bundled together, and deployed along the main file in the
directory defined in the `rargs build` command. All the `sub` files will be
placed following the same relative directory structure they had before.

The next step is to define how the relative script should be called. To do
this, you can leverage other **Rargs** decorators to parse the inputs before
calling script. You do this by using a special `$sub` variable that will be
populated with the right path to the relative file. It's up to you to
define how it should be called and what parameters should be provided.

> The `@sub` decorator is meant to be used at the command level, after a
> `@cmd` decorator.

**`./commands-nested.sh`**

```bash
#!/usr/bin/env bash
# @name commands-nested
# @version 0.0.1
# @description Sample application with nested commands

# @cmd Directory commands
# @sub ./commands-nested/dir.sh
# @flag --dry-run Output the command to `stodut` instead of calling the relative script.
dir() {
  if [[ -n "$rargs_dry_run" ]]; then
    echo "$sub ${rargs_input[*]}"
  fi

	"$sub" ${rargs_input[@]}
}
```

> You can use the `$rargs_input` variable to refer to all the arguments parsed by **Rargs**.

**`./commands-nested/dir.sh`**:

```bash
#!/usr/bin/env bash
# @name dir
# @version 0.0.1
# @description Directory commands

# @cmd Show files in the directory
# @arg path! Directory path
list() {
	echo "${rargs_input[*]}"
}

# @cmd Remove directory
# @arg path! Directory path
# @flag -f --force Remove even if not empty
remove() {
	echo "${rargs_input[*]}"
}
```

---

You've now seen all the features provided by **Rargs**. Next, we'll go over
some examples that show the tool in action, that you can also use as inspiration
for your scripts.

[Show me the examples →](../../usage/examples)

```

```
