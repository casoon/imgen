# imgen

CLI tool to generate images via the [Replicate API](https://replicate.com/). Works with any image generation model on Replicate.

Default model: [black-forest-labs/flux-1.1-pro](https://replicate.com/black-forest-labs/flux-1.1-pro)

## Supported models

Any Replicate model that takes a `prompt` and outputs an image URL works. The model version and input schema are resolved automatically via the API.

| Model | Description | Speed | Text in images |
|-------|-------------|-------|----------------|
| `black-forest-labs/flux-1.1-pro` | FLUX Pro -- best quality (default) | ~4s | poor |
| `black-forest-labs/flux-1.1-pro-ultra` | FLUX Ultra -- highest resolution | ~6s | poor |
| `black-forest-labs/flux-schnell` | FLUX Schnell -- fast & cheap | ~1s | poor |
| `black-forest-labs/flux-dev` | FLUX Dev -- open-source variant | ~10s | poor |
| `ideogram-ai/ideogram-v3-quality` | Ideogram v3 -- best for text in images | ~10s | excellent |
| `stability-ai/sdxl` | Stable Diffusion XL | ~5s | poor |
| `stability-ai/stable-diffusion-3` | Stable Diffusion 3 | ~5s | moderate |

Models that use `aspect_ratio` are handled transparently -- the tool reads each model's schema from the API and picks the best matching ratio for your `--width` and `--height`. Models that support `custom` aspect ratios get exact pixel dimensions; models with fixed ratios (e.g. `16:9`, `3:4`) get the closest match automatically.

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
```

## Setup

Set your Replicate API token:

```bash
export REPLICATE_API_TOKEN="r8_your_token_here"
```

## Usage

### Single image

```bash
imgen "A sunset over the ocean" --out sunset.png
```

With custom dimensions:

```bash
imgen "A cat in space" --width 1920 --height 1080 --out cat.png
```

### Different model

```bash
imgen "A cat in space" --model stability-ai/sdxl --out cat.png
```

### Batch mode

The prompt file supports two formats:

**Simple array** (all defaults from CLI):

```json
[
  { "prompt": "A mountain landscape", "out": "images/mountain.png" },
  { "prompt": "A cityscape at night", "out": "images/city.png" }
]
```

**Object with defaults** (shared settings for all jobs):

```json
{
  "defaults": {
    "model": "black-forest-labs/flux-schnell",
    "width": 1024,
    "height": 1024
  },
  "jobs": [
    {
      "prompt": "A mountain landscape at sunset",
      "out": "images/mountain.png"
    },
    {
      "prompt": "A futuristic cityscape with neon lights",
      "out": "images/city.png",
      "width": 1920,
      "height": 1080
    },
    {
      "prompt": "A coffee shop interior",
      "out": "images/coffee.png",
      "model": "stability-ai/sdxl"
    }
  ]
}
```

Values in individual jobs override the defaults. Defaults override CLI flags.

Run:

```bash
imgen --prompt-file prompts.json
```

### All options

```
Usage: imgen [OPTIONS] [PROMPT]

Arguments:
  [PROMPT]  Text prompt for image generation

Options:
      --prompt-file <FILE>  JSON file with jobs
      --model <MODEL>       Replicate model (owner/name) [default: black-forest-labs/flux-1.1-pro]
      --width <WIDTH>       Output width in pixels [default: 1024]
      --height <HEIGHT>     Output height in pixels [default: 768]
      --out <OUT>           Output file path [default: output.png]
  -h, --help                Print help
  -V, --version             Print version
```

## JSON schema

### Defaults (optional)

| Field    | Type   | Description                                       |
|----------|--------|---------------------------------------------------|
| `model`  | string | Replicate model for all jobs                      |
| `width`  | u32    | Default width for all jobs                        |
| `height` | u32    | Default height for all jobs                       |

### Jobs

| Field    | Type   | Required | Description                                       |
|----------|--------|----------|---------------------------------------------------|
| `prompt` | string | yes      | Text prompt for image generation                  |
| `out`    | string | yes      | Output file path                                  |
| `width`  | u32    | no       | Image width (overrides default)                   |
| `height` | u32    | no       | Image height (overrides default)                  |
| `model`  | string | no       | Replicate model (overrides default)               |

### Priority

Per-job value > `defaults` section > CLI flags (`--model`, `--width`, `--height`)

## License

MIT
