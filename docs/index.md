---
title: Overview
description: What imgen does, what it needs, and how this documentation is organised.
order: 0
---

imgen is a command-line tool that generates images through the
[Replicate API](https://replicate.com/). You pass a prompt, it creates a prediction, waits for the
result and saves the image to disk. A JSON prompt file runs many jobs in one go.

## What it does

- Works with any Replicate model that takes a `prompt` and returns an image URL. The default is
  `black-forest-labs/flux-1.1-pro`.
- Reads each model's input schema from the API and decides how to send the requested size: exact
  pixels, the closest allowed aspect ratio, or plain `width` and `height`.
- Saves PNG or WebP. Models with server-side format selection deliver WebP directly; for all others
  imgen converts locally.
- Retries rate-limited requests and gives up on a single image after 120 seconds.

## What it needs

A Replicate account and an API token in `REPLICATE_API_TOKEN`. Replicate bills each prediction to
that account. imgen talks only to the Replicate API and downloads the image from the URL the API
returns.

## How the docs are organised

- **Getting started:** [installation](getting-started/installation/),
  [API token](getting-started/api-token/) and a [quickstart](getting-started/quickstart/).
- **Guides:** [choosing a model](guides/models/), [automatic schema detection](guides/schema-detection/),
  [batch mode](guides/batch-mode/) and [WebP output](guides/webp/).
- **Reference:** [CLI options](reference/cli/) and the [prompt file format](reference/prompt-file/).
