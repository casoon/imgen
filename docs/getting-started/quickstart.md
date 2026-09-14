---
title: Quickstart
description: Generate a first image, then change size, model and output format.
order: 3
---

## One image

```sh
imgen "A sunset over the ocean" --out sunset.png
```

Without `--out`, the image is saved as `output.png` in the current directory. Missing directories
in the path are created.

While it runs, imgen prints each step to stderr: resolving the model and its schema, creating the
prediction, polling until it finishes, downloading, and the path it saved to.

## Size

```sh
imgen "A cat in space" --width 1920 --height 1080 --out cat.png
```

The default is 1024×768. Whether the model gets exactly these pixels or the closest aspect ratio
depends on the model, see [Automatic schema detection](../../guides/schema-detection/).

## Another model

```sh
imgen "A cat in space" --model stability-ai/sdxl --out cat.png
```

See [Choosing a model](../../guides/models/).

## WebP

```sh
imgen "A sunset" --out sunset.webp --webp
```

Quality defaults to 80; pass a number (`--webp 60`) to change it. See [WebP output](../../guides/webp/).

## Many images

Put the jobs in a JSON file and run `imgen --prompt-file prompts.json`. See
[Batch mode](../../guides/batch-mode/).
