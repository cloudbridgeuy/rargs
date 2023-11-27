+++
title = "Why Choose Rargs?"
description = "The reason why I build this tool."
date = 2023-11-24T08:20:00+00:00
updated = 2023-11-24T08:20:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

My journey with **Rargs** began when my interest in Rust met my need for improved Bash scripts.

I've always wanted scripts that explain themselves and show users how they work. Too often, I've seen scripts with scant or missing documentation. And when there is documentation, it's hard to know if it's up-to-date without diving into the code. So, I searched for a tool that could streamline my workflow and avoid these issues.

Two tools truly stood out: [Bashly](https://bashly.dannyb.co/) and [Argc](https://github.com/sigoden/argc). `Argc` appealed to me with its use of `jsdoc`-like comments to define functions' arguments, options, and flags. And, I liked the idea of creating an app with multiple sub-commands from one script.

> **Disclaimer:** This is not a knock on these tools. They're great and solve specific problems well. They just weren't quite what I was seeking.

`Argc` didn't sit right with me because it needs to be on your `$PATH` and requires a special `eval` at the end of your script.

As for `Bashly`, it's a build tool that takes a YAML config and scripts, combining them into one. It's good, but I didn't like:

1. The need for a YAML config file.
2. Defining commands in separate scripts merged at build time.

So, I envisioned a solution that took the best parts from both. Enter **Rargs**: a tool that uses script annotations to generate a compiled script with all defined functionalities. This results in a single script—or multiple—one that documents its behavior in the source and acts as a standard script when built.

I've been using **Rargs** for some time, and it's made my scripts more comprehensible, maintainable, and user-friendly. I hope it can do the same for you.
