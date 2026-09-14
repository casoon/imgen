---
title: CLI reference
description: Arguments, defaults, environment, exit codes and time limits of the imgen binary.
order: 1
---

imgen is a command-line tool only; there is no library API. It is not published on crates.io, so
there are no docs.rs pages. The source is a single file, `src/main.rs`, in the
[repository](https://github.com/casoon/imgen).

## Usage

Captured from `imgen --help` (version 0.3.1):

```text
CLI tool to generate images via the Replicate API

Usage: imgen [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  Text prompt for image generation (ignored when --prompt-file is used)

Options:
      --prompt-file <FILE>  JSON file containing an array of jobs
      --model <MODEL>       Replicate model in "owner/name" format [default: black-forest-labs/flux-1.1-pro]
      --width <WIDTH>       Output width in pixels [default: 1024]
      --height <HEIGHT>     Output height in pixels [default: 768]
      --out <OUT>           Output file path (only used for single-prompt mode) [default: output.png]
      --webp [<WEBP>]       Convert output to WebP with given quality (0-100). Omit value for default 80
  -h, --help                Print help
  -V, --version             Print version
```

`--prompt-file` also accepts an object with `defaults` and `jobs`, see
[Prompt file format](../prompt-file/).

## Options

| Option | Default | Purpose |
| --- | --- | --- |
| `[PROMPT]` | – | Prompt for a single image. Required unless `--prompt-file` is given. |
| `--prompt-file <FILE>` | – | Run the jobs from a JSON file. |
| `--model <MODEL>` | `black-forest-labs/flux-1.1-pro` | Replicate model, `owner/name`. |
| `--width <WIDTH>` | `1024` | Requested width in pixels. |
| `--height <HEIGHT>` | `768` | Requested height in pixels. |
| `--out <OUT>` | `output.png` | Output path in single-prompt mode. |
| `--webp [<WEBP>]` | off; `80` without a value | Save as WebP with this quality. |

## Environment

| Variable | Purpose |
| --- | --- |
| `REPLICATE_API_TOKEN` | Required. Replicate API token, see [API token](../../getting-started/api-token/). |

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Every image was saved. |
| `1` | Missing token or prompt, invalid prompt file, model lookup failed, or at least one job failed. |

Without a prompt and without `--prompt-file`:

```text
$ imgen
Error: Either a prompt argument or --prompt-file is required
```

## Time limits

| Limit | Value |
| --- | --- |
| Single HTTP request | 30 seconds |
| Polling interval | 2 seconds |
| Waiting for one image | 120 seconds, then the job fails |
| Retries on HTTP 429 | 5, delay from `Retry-After`, fallback 10 seconds |
