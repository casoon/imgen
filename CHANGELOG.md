# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.3.1] - 2026-02-24

### Fixed

- With `--webp` and a model that delivers WebP server-side (e.g. `flux-1.1-pro`), the file was
  saved with the original extension. The output path now always ends in `.webp`.

## [0.3.0] - 2026-02-24

### Changed

- When a model supports `output_format`, imgen requests PNG or WebP (with `output_quality`)
  directly from the API instead of re-encoding locally.

### Fixed

- `aspect_ratio` values referenced via `$ref` (inside `allOf`) are now resolved. Before, FLUX Pro
  images came out 1:1 regardless of the requested dimensions.

## [0.2.2] - 2026-02-24

### Fixed

- An output path ending in `.webp` no longer overwrites and deletes the downloaded image; the
  download goes to a temporary `.png` first.
- The downloaded image format is detected from its content instead of the file extension.

## [0.2.1] - 2026-02-24

### Fixed

- HTTP 429 responses are retried up to 5 times, waiting for the `Retry-After` header
  (fallback: 10 seconds).

## [0.2.0] - 2026-02-23

### Added

- `webp` quality in prompt files, both in `defaults` and per job. Priority: job, then
  `defaults`, then the `--webp` flag.

## [0.1.0] - 2026-02-23

### Added

- CLI for generating images with any Replicate model; default `black-forest-labs/flux-1.1-pro`.
- Automatic schema detection for `aspect_ratio` handling.
- Single prompts and batch mode via JSON prompt files with shared defaults.
- `--webp` flag with optional quality (default 80).
- Prebuilt release binaries for macOS and Linux (x86_64 and arm64) and a Homebrew formula.

### Changed

- TLS via rustls instead of OpenSSL, so the Linux arm64 build cross-compiles.
