+++
title = "FAQ"
description = "Answers to frequently asked questions."
date = 2021-05-01T19:30:00+00:00
updated = 2021-05-01T19:30:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Answers to frequently asked questions."
toc = true
top = false
+++

## Why does it fail when I use a `}` inside a multiline string?

There's a known bug in **Rargs** that I haven't been able to fix revolving
the usage of `}` symbols without any leading whitespace. The current parser looks for the `}` symbol to close the current function body. Nonetheless, it's not uncommon in Bash to do somethong like this:

```bash
print_json() {
  cat <<-EOF | tee /tmp/example.json
{
  "something": "awesome"
}
EOF
}
```

This perfectly balanced Bash code will produce an erroneous built script, since it will consider the `}` used in the JSON object as the end of the function, thus creating a slew of issues.

There is a workaround, if you add a `space` or any other whitespace character (other than a new line) before the `}` then nothing will break. You could also try building your JSON object with some other tool, like [`jo`](https://github.com/jpmens/jo).

## How do I contact the creator?

Send _CloudBridgeUY_ an E-mail:

- <admin@cloudbridge.com.uy>

```

```
