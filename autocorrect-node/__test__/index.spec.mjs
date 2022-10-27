import test from 'ava';

import { format, formatFor, Ignorer, lintFor, loadConfig } from '../index.js';

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

test('loadConfig', (t) => {
  loadConfig("{ textRules: { '你好hello': 0 } }");
  t.assert(format('Hello你好.') === 'Hello 你好。');
  t.assert(format('你好hello.') === '你好hello.');
});

test('Ignorer', (t) => {
  const ignorer = new Ignorer('./');
  t.assert(ignorer.isIgnored('node_modules/foo/bar') === true);
  t.assert(ignorer.isIgnored('src/lib.rs') === false);
  t.assert(ignorer.isIgnored('Cagro.toml') === false);
  t.assert(ignorer.isIgnored('__test__/foo/bar.js') === false);
});
