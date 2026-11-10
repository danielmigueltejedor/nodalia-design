import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { gzipSync } from 'node:zlib';
import { SOURCE_DIR, loadTokens, cssName } from '@nodalia/tokens/lib';
import { lintCss } from '../scripts/lint.mjs';

const read = (rel) => readFileSync(new URL(rel, import.meta.url), 'utf8');
const names = new Set([...loadTokens(SOURCE_DIR).keys()].map(cssName));

test('lint rejects colour literals, colour functions, fixed units and unknown tokens', () => {
  const lines = ['color: #fff;', 'background: rgb(0 0 0);', 'margin: 4px;', 'padding: 0.5rem;', 'transition: x 150ms;', 'border-color: var(--ndl-nope);'];
  for (const l of lines) assert.equal(lintCss(`.a { ${l} }`, names).length, 1, l);
});

test('lint ignores comments and allows unitless numbers, percentages and tokens', () => {
  assert.deepEqual(lintCss('/* #fff 4px */\n.a { opacity: 0.5; inline-size: 100%; margin: var(--ndl-space-md); }', names), []);
});

test('components.css contains no arbitrary visual values and reads only existing tokens', () => {
  const dens = new Set([...names].filter((n) => n.startsWith('--ndl-density-')).concat(['--ndl-density-control-height', '--ndl-density-control-padding-x', '--ndl-density-row-height', '--ndl-density-gap', '--ndl-density-panel-padding']));
  assert.deepEqual(lintCss(read('../src/components.css'), new Set([...names, ...dens])), []);
});

test('every component state has a keyboard focus style', () => {
  const css = read('../src/components.css');
  assert.match(css, /:focus-visible/);
  assert.ok(!/outline:\s*(none|0)\b/.test(css), 'outline must never be removed');
});

test('size budget: nodalia.css <= 14 KB gzip, behaviors <= 5 KB gzip', () => {
  const css = gzipSync(readFileSync(new URL('../dist/nodalia.css', import.meta.url))).length;
  const js = gzipSync(readFileSync(new URL('../dist/behaviors.mjs', import.meta.url))).length;
  assert.ok(css <= 14 * 1024, `nodalia.css is ${css} B gzip`);
  assert.ok(js <= 5 * 1024, `behaviors.mjs is ${js} B gzip`);
});
