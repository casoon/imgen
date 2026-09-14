---
title: Batch mode
description: Run many prompts from one JSON file, with shared defaults, progress output and per-job error handling.
order: 3
---

## Two file formats

A plain array, where every job uses the CLI flags for anything it does not set:

```json
[
  { "prompt": "A mountain landscape", "out": "images/mountain.png" },
  { "prompt": "A cityscape at night", "out": "images/city.png" }
]
```

Or an object with shared `defaults`:

```json
{
  "defaults": { "model": "black-forest-labs/flux-schnell", "width": 1024, "height": 1024, "webp": 80 },
  "jobs": [
    { "prompt": "A mountain landscape at sunset", "out": "images/mountain.png" },
    { "prompt": "A futuristic cityscape with neon lights", "out": "images/city.png", "width": 1920, "height": 1080 },
    { "prompt": "A coffee shop interior", "out": "images/coffee.png", "model": "stability-ai/sdxl" }
  ]
}
```

A job value beats `defaults`, and `defaults` beat the CLI flags. All fields are listed in the
[prompt file reference](../../reference/prompt-file/).

## Run it

```sh
imgen --prompt-file prompts.json
```

## What happens

1. The file is read and validated. Nothing is sent yet.
2. Every distinct model is resolved once. If one cannot be resolved, the run stops here.
3. The jobs run one after another. Each starts with a counter such as `[2/5]`.
4. A failing job is reported with `Error: …` and the next job starts. After the last job, imgen
   exits with `Error: One or more jobs failed` and code 1 if any job failed.

## Mistakes caught before any request

Validation happens before the first API call, so a broken file costs nothing. A job without `out`:

```text
$ imgen --prompt-file examples/prompt-files/missing-out.json
Error: Failed to parse prompt file examples/prompt-files/missing-out.json

Caused by:
    0: Failed to parse jobs array
    1: missing field `out`
```

An empty `jobs` array:

```text
$ imgen --prompt-file examples/prompt-files/empty.json
Error: Prompt file contains no jobs
```

Both outputs are captured from the current binary; the files are in the repository under
`examples/prompt-files/`.

## Rate limits

Replicate limits how many requests an account may send, and the limit is lower for accounts with
little credit. On HTTP 429, imgen waits as long as the `Retry-After` header says (10 seconds if it
is missing) and retries, up to 5 times per request. Long batches on a new account simply take
longer.
