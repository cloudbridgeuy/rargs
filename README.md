# Rargs :rocket:

Introducing Rargs, a Bash framework incorporating elements from [Bashly](https://bashly.dannyb.co/) and [Argc](https://github.com/sigoden/argc) for high-efficiency script deployment.

---

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Examples](#examples)
5. [API Reference](#api-reference)
6. [Contributing](#contributing)
7. [License](#license)

---

## Features :star2:

- Good help messages.
- Good argument, flag, and option parsing.
- Good handling of environment variables.
- Good handling of script dependencies.
- Good suport for script sub-commands.
- Inline documentation.

## Installation :hammer_and_wrench:

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

## Usage :computer:

Here is a quick example:

```bash
rargc -- build -d ./bin ./src/script.sh
```

### Command Line Arguments

```txt
A bash framework for managing your bash scripts

Usage: rargs [COMMAND]

Commands:
  tree   Output a tree of all the commands available based on the script root
  build  Build the script
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Examples :bulb:

Example use-cases can be found under the `/crates/rargs/examples` directory.

## API Reference :book:

Documentation can be found [here](rargs.cloudbridge.uy).

## Contributing :heart:

See [CONTRIBUTING.md](CONTRIBUTING.md) for ways to get started.

## License :scroll:

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

---

Made with :heart: by [@cloudbridgeuy](https://github.com/cloudbridgeuy)
