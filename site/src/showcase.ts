import { ansiToHtml } from '@casoon/pages-theme/ansi';
import type { ShowcaseExample } from '@casoon/pages-theme/showcase';
import type { ImageMetadata } from 'astro';
import gallery from '../../examples/gallery/gallery.json';

// Captured from the current binary by examples/capture.sh. Every case stops before the first
// API request, so no token and no paid generation is needed to regenerate them.
const outputs = import.meta.glob<string>('../../examples/*.txt', {
  query: '?raw',
  import: 'default',
  eager: true,
});
const promptFiles = import.meta.glob<string>('../../examples/prompt-files/*.json', {
  query: '?raw',
  import: 'default',
  eager: true,
});

const examples_: {
  slug: string;
  title: string;
  file: string;
  tags: string[];
  description: string;
  input: { code: string; lang: string };
}[] = [
  {
    slug: 'help',
    title: 'Command reference',
    file: 'help.txt',
    tags: ['options', 'defaults'],
    description: 'All options with their defaults, as printed by the current release.',
    input: { code: 'imgen --help', lang: 'shell' },
  },
  {
    slug: 'missing-token',
    title: 'Missing API token',
    file: 'missing-token.txt',
    tags: ['setup', 'error'],
    description: 'The token check runs first. Without REPLICATE_API_TOKEN nothing is sent.',
    input: { code: 'env -u REPLICATE_API_TOKEN imgen "A sunset over the ocean"', lang: 'shell' },
  },
  {
    slug: 'missing-prompt',
    title: 'No prompt given',
    file: 'missing-prompt.txt',
    tags: ['validation', 'error'],
    description: 'Either a positional prompt or --prompt-file is required.',
    input: { code: 'imgen', lang: 'shell' },
  },
  {
    slug: 'batch-missing-out',
    title: 'Batch job without output path',
    file: 'missing-out.txt',
    tags: ['batch', 'validation'],
    description:
      'imgen --prompt-file examples/prompt-files/missing-out.json: the file is rejected before any request, with the missing field named.',
    input: { code: promptFiles['../../examples/prompt-files/missing-out.json'] ?? '', lang: 'json' },
  },
  {
    slug: 'batch-empty',
    title: 'Empty batch',
    file: 'empty-batch.txt',
    tags: ['batch', 'validation'],
    description: 'imgen --prompt-file examples/prompt-files/empty.json: a prompt file needs at least one job.',
    input: { code: promptFiles['../../examples/prompt-files/empty.json'] ?? '', lang: 'json' },
  },
];

const cliExamples: ShowcaseExample[] = examples_.map(({ file, ...meta }) => ({
  ...meta,
  file: `examples/${file}`,
  output: { html: ansiToHtml(outputs[`../../examples/${file}`] ?? ''), kind: 'terminal' },
}));

// Gallery: generated once with imgen and committed, since generation needs a Replicate token.
// Prompt, model and size come unchanged from the prompt file used; see examples/gallery/gallery.json.
export const galleryImages = import.meta.glob<ImageMetadata>('../../examples/gallery/*.webp', {
  import: 'default',
  eager: true,
});

const escapeAttr = (s: string) => s.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;');

const galleryExamples: ShowcaseExample[] = gallery.images.map((img) => {
  const image = galleryImages[`../../examples/gallery/${img.file}`];
  return {
    slug: `gallery-${img.file.replace(/\.webp$/, '')}`,
    title: `Gallery: ${img.title}`,
    file: `examples/gallery/${img.file}`,
    tags: ['gallery', img.model.split('/')[1] ?? img.model],
    description: `Generated once with imgen using ${img.model} and committed as WebP, because generation needs a Replicate API token. The command passes the prompt exactly as it was used; the model returned ${img.result.width}×${img.result.height} pixels.`,
    input: { code: img.command, lang: 'shell' },
    output: {
      html: `<img src="${image?.src}" width="${image?.width}" height="${image?.height}" alt="${escapeAttr(img.alt)}" loading="lazy" decoding="async" style="display:block;width:100%;height:auto;border-radius:8px">`,
      kind: 'panel',
    },
  };
});

export const examples: ShowcaseExample[] = [...cliExamples, ...galleryExamples];
