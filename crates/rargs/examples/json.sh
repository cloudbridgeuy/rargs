#!/usr/bin/env bash
# @name json
# @version 0.0.1
# @description Script that tests if rargs can handle JSON serialized strings in multi-line mode.
# @author @cloudbridgeuy

root() {
  cat <<-EOF | jq
{
  "something": "awesome"
}
EOF
}

