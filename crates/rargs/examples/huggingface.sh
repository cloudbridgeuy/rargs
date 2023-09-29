#!/usr/bin/env bash
# @name huggingface
# @version 0.0.1
# @description Call the Huggingface API through curl
# @author @cloudbridgeuy
# @dep curl,jo Please install \e[32mcurl\e[0m with \e[32mbrew\e[0m or \e[32maapt-get\e[0m
# @env HUGGING_FACE_API_TOKEN! Please set your Hugging Face API token

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

