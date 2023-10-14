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

## Table of Contents

1. [Requirements](#requirements)
2. [Your First Script](#your-first-script)
3. [Something More Complex](#something-more-complex)
4. [Rules](#rules)
5. [Conclusion](#conclusion)

## Requirements <a name="requirements"></a>

The only requirements to run `rargs` are `bash` version 4 and the `rargs` binary. You may try to run
the resulting scripts on a lower version of `bash`, but there's no guarantee that they will work.

## Your First Script <a name="your-first-script"></a>

`rargs` is heavily inspired by two other `bash` frameworks: `bashly`, and `argc`. The latter uses
comments with a `JsDoc` inspired syntax to add functionality to the scripts at runtime. This syntax,
known as a `comment decorator`, is a normal bash comment followed by an ` @` sign and a
tag. It's how the `rargs` parser identifies configuration.

To utilize this, you need to run the `argc` command and pass the output to an `eval` call somewhere
in your script. `rargs` uses the same comment decorators, and adds a few of its own, but instead of
requiring you to invoke it at runtime, you use the `rargs` binary to build your script. `rargs`
takes every decorator and transforms it into `bash`, while rearranging your functions along the way.
The result is a new `bash` script with all your logic, plus additional functionality like argument
and option parsing, `--help` documentation, and more.

The easiest script you can make is one that only outputs its `help` message:

```bash
#!/usr/bin/env bash
# @name empty
# @version 0.0.1
# @description Empty script
# @author @cloudbridgeuy
```

> You can use any `shebang` you want.

Assume our current directory looks like this, with empty.sh residing in the src directory:

```txt
📂 .
├── 📂 bin
└── 📂 src
    └── empty.sh
```

Compile this script with rargs and execute it.

```bash
# Compile the file to ./bin/empty.sh
rargs build -d ./bin src/empty.sh

# Execute it
./bin/empty -h
```

> On Unix-based systems, rargs tries to set the build file as executable.

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

### Something More Complex <a name="something-more-complex"></a>

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
# @dep curl Please install \e[32mcurl\e[0m with \e[32mbrew install curl\e[0m or \e[32mapt install curl\e[0m
# @env DEHUGGING_FACE_API_TOKEN Please set your Hugging Face API token
```

> You can decorate your help message with colors.

**Note:** You can use `$()` or inverted quotes "`" inside the description and that code will be
invoked.

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

> To get the output in pretty-printed JSON pipe it to `jq` with `| jq`, so it's not necessary to add
> this to the sub-command.

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

> We'll also add `jo` as a local dependency required when calling the `fill-mask` command.

```bash
# @cmd Fill Mask
# @help Tries to fill a hole with a missing word (token to be precise).
# @option -m --model=bert-base-uncased Model to use.
# @option -u --url="https://api-inference.huggingface.co/models" Hugging Face API base URL.
# @flag --no-use-cache Do not use the cache layer on the interface API to speedup requests.
# @flag --wait-for-model If the model is not ready, wait for it instead of receiving a 503 error.
# @arg text! Text with mask to fill
# @example Replace the value of [MASK] with the most likely word $ "The capital of France is [MASK]"
# @dep jo Please install \e[32mjo\e[0m with \e[32mbrew install jo\e[0m or \e[32mapt install jo\e[0m
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
DEBUG=1 ./huggingface.sh fill-mask fill-mask "The capital of France is [MASK]" --no-use-cache --wait-for-model
```

### Rules

Now that our script is getting more complex, it's normal for us to forget all the options available
for it, so calling for `help` is very common. By default, `rargs` will only allow you to call for
the `help` output if you pass the `-h|--help` option as the first argument of the script.

For example, if we pass the `--help` option in any other place other than the first position we'll
get an error.

```text
./huggingface.sh fill-mask "The capital of France is [MASK]" --no-use-cache --wait-for-model --help
Invalid option: --help
```

We can change this behavior by setting the `no-first-option-help` rule.

Just as with other features, we set rules at the `global` or `command` scope by appending the right
comment decorator. In this case we need to use the `@rule` decorator plus the name of the rule we
want to activate. Let's add it at the root scope.

```bash
#!/usr/bin/env bash
# @name huggingface
# @version 0.0.1
# @description Call the Huggingface API through curl
# @author @cloudbridgeuy
# @dep curl Please install \e[32mcurl\e[0m with \e[32mbrew install curl\e[0m or \e[32mapt install curl\e[0m
# @env DEHUGGING_FACE_API_TOKEN Please set your Hugging Face API token
# @rule no-first-option-help
```

If we run the command once again we'll get the expected behavior.

```text
./huggingface.sh fill-mask "The capital of France is [MASK]" --no-use-cache --wait-for-model --help
Fill Mask
Tries to fill a hole with a missing word (token to be precise).

...
```

## Conclusion <a name="conclusion"></a>

Here's the complete script:

```bash
#!/usr/bin/env bash
# @name huggingface
# @version 0.0.1
# @description Call the Huggingface API through curl
# @author @cloudbridgeuy
# @dep curl Please install \e[32mcurl\e[0m with \e[32mbrew\e[0m or \e[32mapt-get\e[0m
# @env HUGGING_FACE_API_TOKEN! Please set your Hugging Face API token
# @rule no-first-option-help

# @cmd Fill Mask
# @help Tries to fill a hole with a missing word (token to be precise).
# @option -m --model=bert-base-uncased Model to use.
# @option -u --url="https://api-inference.huggingface.co/models" Hugging Face API base URL.
# @flag --no-use-cache Do not use the cache layer on the interface API to speedup requests.
# @flag --wait-for-model If the model is not ready, wait for it instead of receiving a 503 error.
# @arg text! Text with mask to fill
# @example Replace the value of [MASK] with the most likely word $ "The capital of France is [MASK]"
# @dep jo Please install \e[32mjo\e[0m with your OS package manager
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

The previous examples showcases almost all of the most important `rargs` features. You can use it as
a starting point to create your own scripts.

You can find more information about each of these features on their corresponding page in the docs.

Happy scripting!
