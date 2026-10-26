#!/usr/bin/env bash
# Regenerates the terminal capture on the start page: a real run of the release binary
# against jobs.json, a prompt file whose second job has no `out` field. imgen rejects the
# file before the first API request, so no network and no real token are needed.
#   cargo build --release && site/captures/capture.sh
# The placeholder token only gets imgen past its token check; it is never sent.
# Rebuild the scene after a capture:
#   cd site/captures && npx -y -p @casoon/castwright@0.5.0 -p shiki castwright build batch.terminal.yaml --format svg -o .
set -uo pipefail

dir="$(cd "$(dirname "$0")" && pwd)"
# The release binary from Cargo's target directory (which may be set via CARGO_TARGET_DIR).
target="$(cd "$dir/../.." && cargo metadata --format-version 1 --no-deps | sed -E 's/.*"target_directory":"([^"]+)".*/\1/')"
bin="${IMGEN:-$target/release/imgen}"
cd "$dir"

"$bin" --version > version.txt
status=0
export REPLICATE_API_TOKEN=placeholder
"$bin" --prompt-file jobs.json > run.txt 2>&1 || status=$?
echo "$status" > exit.txt
