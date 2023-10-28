#!/usr/bin/env bash
# @name json
# @version 0.0.1
# @description Script that tests if rargs can handle JSON serialized strings in multi-line mode.
# @author @cloudbridgeuy

## There is a known bug with `rargs` that happens if you provide a single `}` as part of a
## multi-line string, for example, when creating a JSON object as below. `rargs` thinks the function
## definition has ended and the script breaks.
##
## No proper solution has been implemented for this yet. Still, there are a couple of workarounds
## that can be used to avoid this issue.
##
## 1. Use [`jo`](https://github.com/jpmens/jo) to create the `JSON` object: `jo something=awesome`.
## 2. Add a `
# @dep jo,jq Install jo with brew install jo
root() {
  # Workaround #1
  jo something=awesome | jq

  # Workaround #2
  cat <<-EOF | jq
{
  "something": "awesome"
 }
EOF
}

