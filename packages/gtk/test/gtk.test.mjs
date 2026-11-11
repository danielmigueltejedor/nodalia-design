import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { MODES, SOURCE_DIR, loadTokens, resolve } from '@nodalia/tokens/lib';
import { gtkCss, metricsCss, DENSITIES, allSheets, CRATE_CSS, WIDGETS } from '../scripts/build.mjs';

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

const selectorsOf = (css) => css.split('\n').filter((l) => /\{\s*$/.test(l) && !l.startsWith('/*') && !l.includes('*'));

test('stylesheets contain no widget selectors, so they cannot depend on libadwaita internals', () => {
  for (const mode of MODES) assert.deepEqual(selectorsOf(gtkCss(mode)), [':root {']);
  for (const density of DENSITIES) assert.deepEqual(selectorsOf(metricsCss(density)), [':root {']);
});

test('metric sheets exist for every density and differ only in the density block', () => {
  assert.deepEqual(DENSITIES, ['compact', 'regular', 'comfortable']);
  const strip = (css) => css.split('\n').filter((l) => !l.includes('--ndl-density-') && !l.includes('density.')).join('\n');
  assert.equal(strip(metricsCss('compact')), strip(metricsCss('comfortable')));
  assert.match(metricsCss('regular'), /--ndl-density-control-height: 32px;/);
  assert.match(metricsCss('compact'), /--ndl-density-control-height: 24px;/);
});

test('metric sheets carry no colours and use only units GTK understands', () => {
  for (const density of DENSITIES) {
    const css = metricsCss(density);
    assert.doesNotMatch(css, /--ndl-color-|#[0-9a-f]{3,8}\b/i);
    assert.doesNotMatch(css, /\brem\b|system-ui|-apple-system/);
    for (const name of ['radius-md', 'space-md', 'font-size-body', 'font-weight-semibold', 'component-button-radius', 'font-family-ui']) {
      assert.match(css, new RegExp(`--ndl-${name}: [^;]+;`), `${density}: ${name}`);
    }
  }
});

test('dark accent is brand cyan and light and dark differ', () => {
  assert.match(gtkCss('dark'), /@define-color accent_bg_color #2cc5d3;/);
  assert.notEqual(gtkCss('light'), gtkCss('dark'));
});

test('each mode sheet carries both elevations as box-shadow values', () => {
  for (const mode of MODES) {
    for (const level of ['raised', 'overlay']) {
      assert.match(gtkCss(mode), new RegExp(`--ndl-elevation-${level}: \\S+ \\S+ \\S+ \\S+ (#[0-9a-f]{6}|rgba\\([^)]+\\));`), `${mode}: ${level}`);
    }
  }
});

test('the crate embeds exactly what the generator writes', () => {
  for (const [name, css] of allSheets()) {
    const committed = readFileSync(join(CRATE_CSS, name), 'utf8');
    assert.equal(committed, css, `crate/src/css/${name} is stale: run npm run build -w @nodalia/gtk`);
  }
});

test('the widget sheet takes every value from a variable the other sheets define', () => {
  const defined = new Set();
  for (const [name, css] of allSheets()) {
    if (name === 'nodalia-gtk.css') continue;
    for (const m of css.matchAll(/^\s*(--ndl-[a-z0-9-]+):/gm)) defined.add(m[1]);
  }
  const used = [...WIDGETS.matchAll(/var\((--ndl-[a-z0-9-]+)\)/g)].map((m) => m[1]);
  assert.ok(used.length > 50);
  const missing = [...new Set(used)].filter((v) => !defined.has(v));
  assert.deepEqual(missing, []);
});

test('the widget sheet hard-codes no colour', () => {
  const stripped = WIDGETS.replace(/\/\*[\s\S]*?\*\//g, '');
  const literals = stripped.match(/#[0-9a-fA-F]{3,8}\b|\brgba?\(|\b(white|black)\b/g) ?? [];
  assert.deepEqual(literals, []);
});
