---
title: Prompt file format
description: Fields, types and precedence for the JSON file passed to --prompt-file.
order: 2
---

A prompt file is either a JSON array of jobs or an object with optional `defaults` and a
required `jobs` array. Anything else is rejected before a request is sent.

## Jobs

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `prompt` | string | yes | Text prompt for the image. |
| `out` | string | yes | Output path. Missing directories are created. |
| `width` | integer | no | Width in pixels. |
| `height` | integer | no | Height in pixels. |
| `model` | string | no | Replicate model, `owner/name`. |
| `webp` | number | no | Save as WebP with this quality (0–100). |

## Defaults

Only in the object form. Same meaning as on a job.

| Field | Type |
| --- | --- |
| `model` | string |
| `width` | integer |
| `height` | integer |
| `webp` | number |

## Precedence

Per job, each value is taken from the first place that sets it:

1. the job itself,
2. `defaults`,
3. the CLI flag (`--model`, `--width`, `--height`, `--webp`),
4. the built-in default (`black-forest-labs/flux-1.1-pro`, 1024, 768, no WebP).

`--out` and the positional prompt are ignored when `--prompt-file` is given.

## Example

The repository ships this file as `prompts.example.json`:

```json
{
  "defaults": {
    "model": "black-forest-labs/flux-1.1-pro",
    "width": 1024,
    "height": 768,
    "webp": 80
  },
  "jobs": [
    {
      "prompt": "A serene mountain landscape at sunset with golden light filtering through the clouds",
      "out": "output/mountain_sunset.png"
    },
    {
      "prompt": "A futuristic cityscape with neon lights and flying cars, cyberpunk aesthetic",
      "out": "output/neon_city.png",
      "width": 1920,
      "height": 1080
    },
    {
      "prompt": "A cozy coffee shop interior with warm lighting and rain on the windows",
      "out": "output/coffee_shop.png",
      "model": "black-forest-labs/flux-schnell"
    },
    {
      "prompt": "Portrait of a calico cat sitting on a stack of old books, oil painting style",
      "out": "output/cat_portrait.png",
      "width": 768,
      "height": 1024
    },
    {
      "prompt": "Aerial view of a coral reef with crystal clear turquoise water, photorealistic",
      "out": "output/coral_reef.png"
    }
  ]
}
```

Because `webp` is set in `defaults`, all five images are saved as `.webp`.
