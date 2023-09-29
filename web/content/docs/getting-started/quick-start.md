+++
title = "Quick Start"
description = "One page summary of how to create your first `rargs` script."
date = 2023-09-22T08:20:00+00:00
updated = 2023-09-22T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "One page summary of how to start your first `rargs` script."
toc = true
top = false
+++

## Requirements

The only requirements to run `rargs` is `bash` version 4 and the `rargs` binary. You may try to run
the resulting scripts on a lower version of `bash` but there's no guarantee that they will work.

## Your first script

`rargs` is heavily inspired by two other `bash` frameworks: `bashly`, and `argc`. The latter uses
comments with a JsDoc inspired syntax to add functionality to the scripts at runtime. To be able to
do this, you need to run the `argc` command and pass the output to an `eval` call somewhere on your
script. `rargs` uses the same comment decorators, and adds a few of its own, but instead of
requiring you to invoke it at runtime, you use the `rargs` binary to build your script. `rargs`
takes every decorator and transforms it into `bash`, while rearranging your functions along the way.
The result is a new `bash` script with all your logic, plus additional funcionality like argument
and option parsing, `--help` documentation, and more.

The easiest script you can make is one that only outputs it's `help` message:

```bash
#!/usr/bin/env bash
# @name emtpy
# @version 0.0.1
# @description Empty script
# @author @cloudbridgeuy
```

> You can use any `shebang` you want.

Assume our current directory looks like this:

```txt
 .
├──  bin
└──  src
```

Compile this script with `rargs` and execute it.

```bash
# Compile the file to ./bin/empty.sh
rargs build -d ./bin src/empty.sh

# Execute it
./bin/emtpy -h
```

> On unix bases systems `rargs` tries to set the build file as executable.

If you try to run it, you'll get an error asking you to configure a `root` command or a sub-command
function. You can use `root` commands if you don't wan to support multiple sub-commands. For
example, we can extend our previous script like this:

```bash
#!/usr/bin/env bash
# @name emtpy
# @version 0.0.1
# @description Empty script
# @author @cloudbridgeuy

root() {
    echo "Hello, World!"
}
```

After compiling and running it we'll get `Hello, World!` printed to `stdout` as expected. And if we
run it with the `-h|--help` or `-v|--version` option we'll get this:

```txt
$ ./empty.sh --help
Empty script

Usage:
  emtpy [OPTIONS]
  emtpy -h|--help
  emtpy -v|--version

Options:
  -h --help
    Print help
  -v --version
    Print version

$ ./empty.sh --version
0.0.1
```

We can see how `rargs` decorated our simple function with additional functionalities.

> If you want to see exactly which commands `rargs` added on your script, you can check the built
> script, or run it with the `DEBUG` environment variable set to `true`.

```txt
$ DEBUG=true ./empty.sh
+ set -e
+ run
+ deps=()
+ declare -A deps
+ input=()
+ declare -a input
+ normalize_input
+ local arg flags
+ [[ 0 -gt 0 ]]
+ parse_arguments
+ [[ 0 -gt 0 ]]
+ action=
+ case $action in
+ action=root
+ root
+ parse_root
+ [[ 0 -gt 0 ]]
+ echo 'Hello, World!'
Hello, World!
```

### Something more

Let's do something more complex. We'll use most of the features supported by `rargs` on the
following example. We won't get too deep on how they work on this section. You can check their
specific parts of the documentation for more information.

We want to make a script that interacts with the HuggingFace API. So let's start by creating a new
script called `huggingface.sh` and add our initial `metadata`.

```bash
#!/usr/bin/env bash
# @name huggingface
# @version 0.0.1
# @description Call the Huggingface API through curl
# @author @cloudbridgeuy
```

Our script is going to require `curl` as a tight dependency and it will require us to configure the
`HUGGING_FACE_API_TOKEN` environment variable with our account secret. `rargs` can make sure that we
have both of these values before running the script by setting the `@dep` and `@env` decorators.

> Decorators can be set at the `root` scope or per sub-command. Sub-commands will inherit `root`
> configuration by default.

Let's add these decorators to our script.

```bash
#!/usr/bin/env bash
# @name huggingface
# @version 0.0.1
# @description Call the Huggingface API through curl
# @author @cloudbridgeuy
# @dep carl Please install \e[32mcurl\e[0m with \e[32mbrew install curl\e[0m or \e[32mapt install curl\e[0m
# @env DEHUGGING_FACE_API_TOKEN Please set your Hugging Face API token
```

> You can decorate your help message with colors.

With these changes the `script` won't execute unless `curl` is installed in the system and the
Huggingface API is set.

Now, let's add our first sub-command. Given that there are multiple uses for LLM, we wan't to able
to navigate to the right usage with our API. For example, let's say we wanted to use the
`bert-base-uncased` model to find a word to fill a match. Is typically used to showcase the behavior
of these models and it's fun to use.

We'll start by creating a function called `fill-mask` and decorate it with additional options and
arguments that we'll provide at runtime to override its behavior.

```bash
# @cmd Fill Mask
# @help Tries to fill a hole with a missing word (token to be precise).
# @option -m --model=bert-base-uncased Model to use.
# @option -u --url="https://api-inference.huggingface.co/models" Hugging Face API base URL.
# @arg text! Text with mask to fill
# @example Replace the value of [MASK] with the most likely word $ "The capital of France is [MASK]"
fill-mask() {
  curl -sL "$rargs_url/$rargs_model" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $HUGGING_FACE_API_TOKEN" \
    -d "{\"inputs\": \"$rargs_text\"}"
}
```

After compiling it, run it like this and get your output:

```txt
$ ./huggingface.sh fill-mask "The capital of France is [MASK]"
[
  {
    "score": 0.7153865694999695,
    "token": 1012,
    "token_str": ".",
    "sequence": "the capital of france is."
  },
  {
    "score": 0.2526995837688446,
    "token": 1025,
    "token_str": ";",
    "sequence": "the capital of france is ;"
  },
  {
    "score": 0.028098540380597115,
    "token": 1064,
    "token_str": "|",
    "sequence": "the capital of france is |"
  },
  {
    "score": 0.0020544484723359346,
    "token": 999,
    "token_str": "!",
    "sequence": "the capital of france is!"
  },
  {
    "score": 0.001290082116611302,
    "token": 1029,
    "token_str": "?",
    "sequence": "the capital of france is?"
  }
]
```

> To get the output in pretty-printed JSON pipe it to `jq` with `| jq`.

We also can get a nice `help` message using the `--help` flag.

```txt
Fill Mask
Tries to fill a hole with a missing word (token to be precise).

Usage:
  fill-mask [OPTIONS] TEXT
  fill-mask -h|--help

Examples:
  fill-mask "The capital of France is [MASK]"
    Replace the value of [MASK] with the most likely word

Arguments:
  TEXT
    Text with mask to fill
    [@required]

Options:
  -m --model [<MODEL>]
    Model to use.
    [@default bert-base-uncased]
  -u --url [<URL>]
    Hugging Face API base URL.
    [@default https://api-inference.huggingface.co/models]
  -h --help
    Print help
```

There are some additional options that this task can consume so let's add them. Also, to help us
craft the required `JSON` object we'll use a tool called `jo`.

```bash
# @cmd Fill Mask
# @help Tries to fill a hole with a missing word (token to be precise).
# @option -m --model=bert-base-uncased Model to use.
# @option -u --url="https://api-inference.huggingface.co/models" Hugging Face API base URL.
# @flag --no-use-cache Do not use the cache layer on the interface API to speedup requests.
# @flag --wait-for-model If the model is not ready, wait for it instead of receiving a 503 error.
# @arg text! Text with mask to fill
# @example Replace the value of [MASK] with the most likely word $ "The capital of France is [MASK]"
fill-mask() {
  body="$(jo inputs="$rargs_text")"

  if [[ -n "$rargs_no_use_cache" ]]; then
    body="$(jo -f <(echo -n "$body") use_cache=false)"
  fi

  if [[ -n "$rargs_wait_for_model" ]]; then
    body="$(jo -f <(echo -n "$body") wait_for_model=true)"
  fi

  curl -sL "$rargs_url/$rargs_model" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $HUGGING_FACE_API_TOKEN" \
    -d "$body"
}
```

> Since the default value of the `use_cache` option is set to `true` we use the `no-` prefix
> convention to indicate that we are actually setting its value to `false`.

If we want to see the body we are passing `curl` we can execute the command in `DEBUG` mode.

```bash
DEBUG=1 ./crates/rargs/examples/output/huggingface.sh fill-mask "The capital of France is [MASK]" --no-use-cache --wait-for-model
```
