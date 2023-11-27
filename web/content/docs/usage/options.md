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

## Creating a New Script

You can start writing a new **Rargs** script in any directory on your machine. All Bash scripts are valid **Rargs** sources. You can begin with an existing script or create a new one. If you prefer not to start with an empty script, you can run `rargs new`.

```bash
rargs new --destination ./src minimal
```

This command will generate a minimal template for you to start with.

## Scopes

**Rargs** scripts are configured using `comment decorators`. These tags define every feature supported by the framework. At build time, **Rargs** will use the information inside the `comment decorators` to configure the build script.

> **Rargs** ignores and removes all comments at build time. To make a comment persist, add an extra `#` at the beginning of the line.

Each `comment tag` is assigned to either the script or one of its commands. Tags assigned to the script are part of the `root` scope, while those assigned to a command belong to that `function` scope.

> Double `##` comments in the `root` scope are hoisted to the beginning of the script. Similarly, double `##` comments in each `function` scope are hoisted to the start of the function.

## Script Metadata

You can add metadata to your script at the root level using `comment decorators`. Here's an example:

```bash
#!/usr/bin/env bash
# @name minimal
# @version 0.0.1
# @description Sample minimal application without commands
```

These tags set the `name`, `version`, and description of the app. This information appears in the `--help` and `--version` commands.

> The `--version` option is automatically generated when you add the `@version` tag.

The full list of available metadata `comment decorators` tags is:

| Tag            | Description                       |
| -------------- | --------------------------------- |
| `@name`        | The name of the application.      |
| `@version`     | The version of the application.   |
| `@description` | A description of the application. |
| `@author`      | The author of the application.    |
| `@license`     | The license of the application.   |
| `@default`     | The default command to run.       |

## Commands

A **Rargs** script consists of one or more commands. A command is a Bash function that you can call from the command line. It can have arguments, flags, options, dependencies, rules, and more. Not all Bash functions are exposed as commands, and some can be set as private.

To define a Bash function as a command, add the `@cmd` tag to it. Here's an example:

```bash
#!/usr/bin/env bash
# @name minimal

# @cmd Greet the world
hello() {
  echo "Hello World!"
}
```

This exposes the `hello` command from the script.

> The `@cmd` tag requires a brief description of the command's purpose. This description is used in the help message.

Every **Rargs** script must have at least one command. If commands don't fit your script, create it with a single special function called `root`. This function is invoked when you call the script without any commands. You can assign the same behavior to a `root` function as to any other command function.

Each `command decorator` under the `@cmd` tag configures the command. The full list of available `command decorator` tags is:

| Tag        | Description                                     |
| ---------- | ----------------------------------------------- |
| `@option`  | Define an option for the command.               |
| `@flag`    | Define a flag for the command.                  |
| `@arg`     | Define an argument for the command.             |
| `@env`     | Define an environment variable for the command. |
| `@rule`    | Define a rule for the command.                  |
| `@dep`     | Define a dependency for the command.            |
| `@private` | Set the command as private.                     |
| `@example` | Add an example of how to use this command.      |

After the `@cmd` tag and any additional configuration, add the Bash function that executes when the command is called. The command's scope ends when **Rargs** detects the end of the function declaration.

All `command decorator` tags at the `root` scope are declared regardless of the line. However, all `command decorator` tags for a function must be defined between the `@cmd` tag and the function declaration.

---

In the following sections, we'll dive deeper into each `command decorator` tag and how to use them.

[Show me how options work →](../../usage/options)
