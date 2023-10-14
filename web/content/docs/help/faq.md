+++
title = "FAQ"
description = "Answers to frequently asked questions."
date = 2021-05-01T19:30:00+00:00
updated = 2021-05-01T19:30:00+00:00
draft = false
weight = 30
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Answers to frequently asked questions."
toc = true
top = false
+++

## What is the Rargs?

Rargs is a Bash framework based on other tools like [Bashly](https://bashly.dannyb.co/) and
[Argc](https://github.com/sigoden/argc) that aims to simplify the process of creating bash scripts
that include:

- Good help messages.
- Good argument, flag, and option parsing.
- Good handling of environment variables.
- Good handling of script dependencies.
- Good suport for script sub-commands.
- Inline documentation.

All `rargs` configuration is done in the script itself, as `comment decorators`. Meaning, normal
bash comments using `#` plus an ` @` followed by a `tag`. For example, the `# @option` decorator
will let you configure an option argument to be passed to the script of a sub-command.

The idea is for the script documentation to live alongside the script itself, this making it easier
to keep the documentation up. It's very common for developers to forget to document scripts, that
then become key parts of their infrastructure, but others are afraid to touch them or use them
properly since it's not evident to understand how it works.

## Contact the creator?

Send *CloudBridgeUY* an E-mail:

- <admin@cloudbridge.com.uy>
