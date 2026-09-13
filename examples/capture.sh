#!/usr/bin/env bash
# Regenerates the website fixtures in examples/ from the current CLI.
# Every case here stops before the first API call: no token, no network, no cost.
#
#   cargo build --release && examples/capture.sh
set -uo pipefail

cd "$(dirname "$0")/.."
bin="${IMGEN:-target/release/imgen}"
out=examples

# Placeholder token: lets imgen pass the token check so it reaches input validation.
# None of the cases below gets far enough to use it.
fake=offline-placeholder

capture() {
  local file=$1
  shift
  "$@" >"$out/$file" 2>&1
  local code=$?
  if [ "$code" -ne 0 ]; then echo "exit code $code" >>"$out/$file"; fi
}

capture help.txt "$bin" --help
capture version.txt "$bin" --version
capture missing-token.txt env -u REPLICATE_API_TOKEN "$bin" "A sunset over the ocean"
capture missing-prompt.txt env REPLICATE_API_TOKEN=$fake "$bin"
capture empty-batch.txt env REPLICATE_API_TOKEN=$fake "$bin" --prompt-file $out/prompt-files/empty.json
capture missing-out.txt env REPLICATE_API_TOKEN=$fake "$bin" --prompt-file $out/prompt-files/missing-out.json
