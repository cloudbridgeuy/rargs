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
# Include rargs library
source rargs.sh

# Code usage example
```

### Command Line Arguments

```bash
# Use --help to list all options
your_script.sh --help
```

## Examples :bulb:

Example use-cases can be found under the `/examples` directory.

## API Reference :book:

Documentation can be found [here](your-documentation-link).

## Contributing :heart:

See [CONTRIBUTING.md](CONTRIBUTING.md) for ways to get started.

## License :scroll:

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

---

Made with :heart: by [@cloudbridgeuy](https://github.com/cloudbridgeuy)
