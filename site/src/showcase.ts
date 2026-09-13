import { ansiToHtml } from '@casoon/pages-theme/ansi';
import type { ShowcaseExample } from '@casoon/pages-theme/showcase';

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

export const examples: ShowcaseExample[] = examples_.map(({ file, ...meta }) => ({
  ...meta,
  file: `examples/${file}`,
  output: { html: ansiToHtml(outputs[`../../examples/${file}`] ?? ''), kind: 'terminal' },
}));
