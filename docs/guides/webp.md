---
title: WebP output
description: Save images as WebP, either delivered by the model or converted locally.
order: 4
---

## Enable WebP

```sh
imgen "A sunset" --out sunset.webp --webp      # quality 80
imgen "A sunset" --out sunset.webp --webp 60   # quality 0–100
```

In a prompt file, set `webp` to a quality value in `defaults` or on a job. See
[Batch mode](../batch-mode/).

## The file name

With WebP enabled, the saved file always ends in `.webp`. `--out sunset.png --webp` writes
`sunset.webp`.

## Where the conversion happens

- **Models with `output_format`** (for example `flux-1.1-pro`) receive `output_format: "webp"` and
  `output_quality`. The API delivers WebP and imgen saves it unchanged, without a second lossy
  encoding.
- **All other models** return their own format. imgen downloads it, detects the format from the
  file content, encodes WebP locally with the given quality and removes the original download.

Which case applies is detected per model, see [Automatic schema detection](../schema-detection/).
Without WebP, models with `output_format` are asked for `png`.
