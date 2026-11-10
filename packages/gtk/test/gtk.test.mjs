import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { MODES, SOURCE_DIR, loadTokens, resolve } from '@nodalia/tokens/lib';
import { gtkCss } from '../scripts/build.mjs';

const mapping = Object.entries(JSON.parse(readFileSync(new URL('../mapping.json', import.meta.url), 'utf8'))).filter(([k]) => !k.startsWith('$'));
const tokens = loadTokens(SOURCE_DIR);

test('every mapped libadwaita colour points at an existing colour token', () => {
  for (const [name, path] of mapping) {
    assert.equal(tokens.get(path)?.type, 'color', `${name} -> ${path}`);
  }
});

test('only documented libadwaita named colours are set', () => {
  const allowed = /^(accent|destructive|success|warning|error|window|view|headerbar|sidebar|card|dialog|popover)_(bg_color|fg_color|color|border_color|backdrop_color)$/;
  for (const [name] of mapping) assert.match(name, allowed);
});

test('each mode stylesheet defines every mapped colour as @define-color and as a CSS variable', () => {
  for (const mode of MODES) {
    const css = gtkCss(mode);
    for (const [name, path] of mapping) {
      const hex = resolve(tokens, path, mode).slice(0, 7).toLowerCase();
      assert.ok(css.includes(`@define-color ${name} ${hex};`), `${mode}: ${name}`);
      assert.ok(css.includes(`--${name.replaceAll('_', '-')}: ${hex};`), `${mode}: --${name}`);
    }
  }
});

test('stylesheets contain no widget selectors, so they cannot depend on libadwaita internals', () => {
  for (const mode of MODES) {
    const selectors = gtkCss(mode).split('\n').filter((l) => /\{\s*$/.test(l) && !l.startsWith('/*') && !l.includes('*'));
    assert.deepEqual(selectors, [':root {']);
  }
});

test('dark accent is brand cyan and light and dark differ', () => {
  assert.match(gtkCss('dark'), /@define-color accent_bg_color #8adbe0;/);
  assert.notEqual(gtkCss('light'), gtkCss('dark'));
});
