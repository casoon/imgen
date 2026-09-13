// @ts-check
import casoonPages from '@casoon/pages-theme';
import { defineConfig } from 'astro/config';

// Project page: https://casoon.github.io/imgen/ — `base` is the GitHub Pages path.
export default defineConfig({
  site: 'https://casoon.github.io/imgen',
  base: '/imgen/',
  integrations: [
    casoonPages({
      name: 'imgen',
      description:
        'A Rust CLI that generates images with any Replicate model, from a single prompt or a JSON batch file.',
      repo: 'casoon/imgen',
      version: '0.3.1',
      license: 'MIT',
      branch: 'master',
      packages: [
        { label: 'Releases', href: 'https://github.com/casoon/imgen/releases' },
        { label: 'Homebrew tap', href: 'https://github.com/casoon/homebrew-tap' },
      ],
      docsGroups: {
        'getting-started': 'Getting started',
        guides: 'Guides',
        reference: 'Reference',
      },
    }),
  ],
});
