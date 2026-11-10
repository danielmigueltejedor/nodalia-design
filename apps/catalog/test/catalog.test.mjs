import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { SOURCE_DIR, loadTokens } from '@nodalia/tokens/lib';

const dist = new URL('../dist/', import.meta.url).pathname;
const pages = ['index.html', ...readdirSync(join(dist, 'examples')).map((f) => `examples/${f}`)];

test('builds the catalog and three reference demos', () => {
  assert.deepEqual(pages.sort(), ['examples/creative.html', 'examples/engineering.html', 'examples/productivity.html', 'index.html']);
});

test('every page is a complete document with language, title and viewport', () => {
  for (const p of pages) {
    const html = readFileSync(join(dist, p), 'utf8');
    assert.match(html, /^<!doctype html>/i, p);
    assert.match(html, /<html lang="en">/, p);
    assert.match(html, /<title>[^<]+<\/title>/, p);
    assert.match(html, /name="viewport"/, p);
  }
});

test('pages load only local files: no CDN, no network at runtime', () => {
  for (const p of pages) {
    const html = readFileSync(join(dist, p), 'utf8');
    assert.ok(!/(src|href)="https?:/.test(html), `${p} references an external URL`);
    for (const [, ref] of html.matchAll(/(?:src|href)="([^"#:]+?)(?:#[^"]*)?"/g)) {
      assert.ok(existsSync(join(dist, dirname(p), ref)), `${p}: missing ${ref}`);
    }
    for (const [, ref] of html.matchAll(/from '(\.[^']+)'/g)) assert.ok(existsSync(join(dist, dirname(p), ref)), `${p}: missing module ${ref}`);
  }
});

test('the catalog shows every semantic colour token', () => {
  const html = readFileSync(join(dist, 'index.html'), 'utf8');
  const shown = [...loadTokens(SOURCE_DIR).keys()].filter((p) => /^color\.(surface|text|border|accent|control|selection|status)\./.test(p));
  assert.ok(shown.length > 30);
  for (const p of shown) assert.ok(html.includes(`<code>${p}</code>`), p);
});

test('the catalog lists forced interaction states for the core controls', () => {
  const html = readFileSync(join(dist, 'index.html'), 'utf8');
  for (const s of ['hover', 'active', 'focus']) assert.ok(html.includes(`data-state="${s}"`), s);
  assert.ok(html.includes('disabled'));
});

test('no template placeholder survives the build', () => {
  assert.ok(!/<!--@/.test(readFileSync(join(dist, 'index.html'), 'utf8')));
});
