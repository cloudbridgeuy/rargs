+++
title = "Installation"
description = "Learn how to install Rargs on your operating system"
date = 2023-09-22T08:20:00+00:00
updated = 2023-09-22T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
toc = true
top = false
+++

The best way to install `rargs` is to download the pre-built version from the project [`Release`
page.](https://github.com/cloudbridgeuy/rargs)

## Build from source

You can build your own version of `rargs` using `cargo`, Rust build tool. To do so, clone the
project into a directory, change into the cloned directory and run cargo.

```bash
git clone https://github.com/cloudbridgeuy/rargs.git
cd rargs
cargo build --bin rargs
```

You can find the build binary in the created `target/release` dirctory by the name `rargs`. Copy it
into a directory on your path, or add the `release` directory you your `PATH`, and give the `rargs`
binary executable permissions.

```bash
chmod +x rargs
```
