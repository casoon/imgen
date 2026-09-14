---
title: Choosing a model
description: Pick any Replicate image model with --model, per job in a prompt file, or keep the FLUX Pro default.
order: 1
---

## Set the model

Models are addressed as `owner/name`, exactly as in the Replicate URL:

```sh
imgen "A cat in space" --model stability-ai/sdxl --out cat.png
```

Without `--model`, imgen uses `black-forest-labs/flux-1.1-pro`. In a prompt file, set `model` in
`defaults` or on a single job; see [Batch mode](../batch-mode/).

imgen always runs the model's **latest published version**. It looks the version up once per model
and run, so a batch with ten jobs on the same model makes a single lookup.

## Which models work

Any Replicate model that accepts a `prompt` input and returns an image URL (or a list whose first
entry is an image URL). The size and format inputs are detected per model, see
[Automatic schema detection](../schema-detection/).

The README lists these as tested starting points:

| Model | Notes |
| --- | --- |
| `black-forest-labs/flux-1.1-pro` | Default. Accepts exact pixel sizes, delivers WebP server-side. |
| `black-forest-labs/flux-1.1-pro-ultra` | Highest resolution of the FLUX family. |
| `black-forest-labs/flux-schnell` | Fast and cheap, good for drafts. |
| `black-forest-labs/flux-dev` | Open-source FLUX variant. |
| `ideogram-ai/ideogram-v3-quality` | The best choice when the image must contain legible text. |
| `stability-ai/sdxl` | Stable Diffusion XL. |
| `stability-ai/stable-diffusion-3` | Stable Diffusion 3, moderate text rendering. |

Prices and speed are set by Replicate and change over time; check the model page before large
batches.

## When a model fails to resolve

A typo in the name, a private model or a model without a published version stops the run before
any image is generated. The error names the model and repeats the API response, for example
`Failed to resolve model owner/name: 404 Not Found …`.
