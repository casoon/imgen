---
title: Automatic schema detection
description: How imgen reads a model's OpenAPI schema and turns --width and --height into the inputs that model understands.
order: 2
---

Replicate models disagree on how they take an image size. Some accept pixels, some only a fixed
list of aspect ratios, some both. imgen does not keep a list of models for this. It asks the API.

## What imgen reads

Before the first prediction, imgen fetches `GET /v1/models/<owner>/<name>` and looks at the
OpenAPI schema of the model's latest version:

1. **`aspect_ratio`** under `components/schemas/Input/properties`. The allowed values come from
   an `enum` on the property itself or, if the property points to a shared definition via `$ref`
   (also inside `allOf`), from that definition.
2. **`output_format`** in the same place. If it exists, the model can deliver PNG or WebP itself.

The result is printed once per model:

```text
Resolving model black-forest-labs/flux-1.1-pro…
  version: <first 12 characters of the version id>…  aspect_ratio: <allowed values or none>  output_format: <yes|no>
```

## How the size is sent

| The schema has … | imgen sends |
| --- | --- |
| `aspect_ratio` including `custom` | `aspect_ratio: "custom"` plus your exact `width` and `height` |
| `aspect_ratio` with fixed values only | the allowed ratio closest to `width / height`, no pixel size |
| no `aspect_ratio` | `width` and `height` as given |

"Closest" compares the numeric ratios. With the default 1024×768 (1.33) and a model that allows
`1:1`, `16:9`, `3:2` and `4:3`, imgen picks `4:3`. For 1920×1080 it picks `16:9`. If none of the
allowed values has the form `W:H`, it falls back to `1:1`.

With fixed ratios, the model decides the final pixel size. Your `--width` and `--height` only pick
the shape.

## How the format is sent

If the schema has `output_format`, imgen requests `png`, or `webp` with `output_quality` when WebP
output is enabled. Otherwise it downloads what the model returns and converts locally if needed.
Details in [WebP output](../webp/).

## Why this matters

Before version 0.3.0, imgen only read a direct `enum`. FLUX Pro defines its ratios through `$ref`,
so every FLUX Pro image came out square. Following the reference fixed that without any
model-specific code.
