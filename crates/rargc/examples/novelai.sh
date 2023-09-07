#!/usr/bin/env bash
# @name novelai
# @description NovelAI CLI to call the NovelAI API
# @version 0.1.0
# @author Guzmán Monné
# @license MIT
# @option --novelai-api-key NovelAI API Key
# @option --novelai-endpoint="https://api.novelai.net/ai/generate-stream" NovelAI API Endpoint
# @env NOVELAI_API_KEY:novelai-api-key
# @env NOVELAI_ENDPOINT:novelai-endpoint

# @cmd Generate a completion stream
# @arg input="-" Input for the text generation model (reads from stdin if empty)
# @option -m --model[=kayra-v1|clio-v1|purple|green|red|blue|sigurd-2.9b-v1|cassandra|infillmodel|hypebot|krake-v2|genji-jp-6b-v2|genji-python-6b|euterpe-v2|6B-v4|2.7B] Used text generation model.
# @option -u --use-string[=true|false] If false, input and output strings should be Base64-encoded uint16 numbers representing tokens.
# @option -t --temperature=3 Temperature for the generation. [@min 0.1, @max 100]
# @option -m --min-length=1 Minimum length of the generated text. [@min 1, @max 2048]
# @option -M --max-length=40 Maximum length of the generated text. [@min 1, @max 2048]
generate-stream() {
  local input=()

  if [[ "${args["input"]}" == "-" ]]; then
    while read -r line; do
      input+=("$line")
    done
  else
    input=("${args["input"]}")
  fi

  curl -X 'POST' \
    "${args["novelai-endpoint"]}" \
    -H 'Accept: application/json' \
    -H 'Content-Type: application/json' \
    -d '{
    "input": '"$(jq -R @json <<<"${input[@]}")"',
    "model": "'"${args["model"]}"'",
    "parameters": {
      "use_string": '"${args["use-string"]}"',
      "temperature": '"${args["temperature"]}"',
      "min_length": '"${args["min-length"]}"',
      "max_length": '"${args["max-length"]}"',
      "typical_p": 0.969,
      "tail_free_sampling": 0.941,
      "repetition_penalty": 3,
      "repetition_penalty_range": 4000,
      "repetition_penalty_frequency": 0,
      "repetition_penalty_presence": 0,
      "cfg_scale": 1.48,
      "cfg_uc":"",
      "phrase_rep_pen":"medium",
      "mirostat_tau":4.95,
      "mirostat_lr":0.22,
      "bad_words_ids":[
        [3],[49356],[1431],[31715],[34387],[20765],[30702],[10691],[49333],[1266],[19438],[43145],[26523],[41471],[2936],[85,85],[49332],[7286],[1115]
      ],
      "repetition_penalty_whitelist": [ 49256, 49264, 49231, 49230, 49287, 85, 49255, 49399, 49262, 336, 333, 432, 363, 468, 492, 745, 401, 426, 623, 794, 1096, 2919, 2072, 7379, 1259, 2110, 620, 526, 487, 16562, 603, 805, 761, 2681, 942, 8917, 653, 3513, 506, 5301, 562, 5010, 614, 10942, 539, 2976, 462, 5189, 567, 2032, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 588, 803, 1040, 49209, 4, 5, 6, 7, 8, 9, 10, 11, 12 ],
      "repetition_penalty": 1,
      "repetition_penalty_presence": 0,
      "repetition_penalty_frequency": 0,
      "repetition_penalty_range": 4000,
      "generate_until_sentence":true,
      "use_cache":false,
      "return_full_text":false,
      "prefix":"special_proseaugmenter",
      "logit_bias_exp":[
        {
          "sequence":[23],
          "bias":-0.08,
          "ensure_sequence_finish":false,
          "generate_once":false
        },{
          "sequence":[21],
          "bias":-0.08,
          "ensure_sequence_finish":false,
          "generate_once":false
        }
      ],
      "num_logprobs":10,
      "order":[8,6,5,0,3]
    }
  }' -N -s \
  | while read -r line; do
    if [[ $line == data:* ]]; then
      if [[ "$line" != "data: [DONE]" ]]; then
        printf "%s" "$(yq <<<"$line" | cut -c6- | jq '.token' -r | grep -v "^null$")"
      else
        echo
      fi
    fi
  done
}
