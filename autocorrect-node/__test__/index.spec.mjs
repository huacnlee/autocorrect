import test from 'ava';

import { format, formatFor, lintFor } from '../index.js';

test('format', (t) => {
  t.assert(format('Hello你好.') === 'Hello 你好。');
});

test('formatFor', (t) => {
  t.assert(formatFor('# Hello你好.', 'md') === '# Hello 你好。');
  t.assert(formatFor('<p>Hello你好.</p>', 'html') === '<p>Hello 你好。</p>');
});

test('lintFor', (t) => {
  t.deepEqual(lintFor('<p>Hello你好.</p>', 'html'), {
    error: '',
    filepath: 'html',
    lines: [
      {
        c: 4,
        l: 1,
        new: 'Hello 你好。',
        old: 'Hello你好.',
        severity: 1,
      },
    ],
  });
});
