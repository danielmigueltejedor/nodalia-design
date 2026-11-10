import { test } from 'node:test';
import assert from 'node:assert/strict';
import { MODES, SOURCE_DIR, loadTokens, contrast, resolve } from '@nodalia/tokens/lib';
import { DENSITIES, flatTokens, placeholders, qssColor, render, template, tokenValue } from '../scripts/build.mjs';

const tokens = loadTokens(SOURCE_DIR);
const flat = flatTokens();
const source = template();

test('flat tokens carry every colour for every mode, in a form Qt accepts', () => {
  const colours = [...tokens.values()].filter((t) => t.type === 'color' && t.path.startsWith('color.'));
  for (const mode of MODES) {
    assert.equal(Object.keys(flat.color[mode]).length, colours.length, mode);
    for (const value of Object.values(flat.color[mode])) assert.match(value, /^(#[0-9A-F]{6}|rgba\(\d+, \d+, \d+, \d+\))$/);
  }
  assert.equal(flat.color.dark.surfaceWindow, resolve(tokens, 'color.surface.window', 'dark').toUpperCase());
});

test('translucent colours become rgba with alpha 0-255', () => {
  assert.equal(qssColor('#0D0F1566'), 'rgba(13, 15, 21, 102)');
  assert.equal(qssColor('#ffffff'), '#FFFFFF');
});

test('names are two-level and letters only, so one placeholder syntax covers them all', () => {
  for (const [group, values] of Object.entries(flat)) {
    if (typeof values !== 'object' || Array.isArray(values)) continue;
    const leaves = ['color', 'density'].includes(group) ? Object.values(values).flatMap((v) => Object.keys(v)) : Object.keys(values);
    for (const name of leaves) assert.match(name, /^[A-Za-z]+$/, `${group}.${name}`);
  }
});

test('every placeholder of the template resolves in every mode and density', () => {
  assert.ok(placeholders(source).length > 40);
  for (const mode of MODES) for (const density of DENSITIES) {
    const sheet = render(source, flat, mode, density);
    assert.ok(!sheet.includes('{{'), `${mode} ${density}`);
    assert.ok(!/undefined|NaN|\[object/.test(sheet));
  }
});

test('an unknown token fails the build instead of leaving a hole', () => {
  assert.throws(() => render('QWidget {color:{{color.nope}};}', flat, 'light', 'regular'), /unknown token color\.nope/);
});

test('density changes control height and nothing else changes colour', () => {
  assert.equal(tokenValue(flat, 'light', 'compact', 'density', 'controlHeight'), '24');
  assert.equal(tokenValue(flat, 'light', 'comfortable', 'density', 'controlHeight'), '40');
  assert.equal(tokenValue(flat, 'dark', 'compact', 'color', 'accentDefault'), tokenValue(flat, 'dark', 'comfortable', 'color', 'accentDefault'));
});

test('the template has no colour or pixel literal of its own', () => {
  const body = source.replace(/\/\*[\s\S]*?\*\//g, '').replace(/\{\{[A-Za-z.]+\}\}/g, 'T');
  assert.ok(!/#[0-9a-fA-F]{3,8}\b/.test(body), 'hex colour literal');
  assert.ok(!/rgba?\(/.test(body), 'rgb literal');
  // Zero and the slider thumb offset are the only numbers allowed next to px.
  const literals = [...body.matchAll(/(?<![A-Za-z])(-?\d+)px/g)].map((m) => m[1]).filter((n) => !['0', '-6'].includes(n));
  assert.deepEqual(literals, []);
});

test('the colour pairs the sheet draws keep WCAG contrast in every mode', () => {
  const pairs = [
    ['text.primary', 'control.bg', 4.5], ['text.primary', 'control.bg-hover', 4.5], ['text.on-accent', 'accent.default', 4.5],
    ['text.on-accent', 'accent.hover', 4.5], ['accent.text', 'accent.subtle', 4.5], ['text.primary', 'control.field-bg', 4.5],
    ['text.secondary', 'surface.panel', 4.5], ['accent.text', 'surface.panel', 4.5], ['text.primary', 'selection.bg', 4.5],
    ['text.inverse', 'surface.inverse', 4.5], ['text.on-accent', 'status.danger.fg', 4.5], ['border.strong', 'control.field-bg', 3],
    ['border.focus', 'surface.panel', 3],
  ];
  for (const mode of MODES) for (const [fg, bg, min] of pairs) {
    const ratio = contrast(resolve(tokens, `color.${fg}`, mode).slice(0, 7), resolve(tokens, `color.${bg}`, mode).slice(0, 7));
    assert.ok(ratio >= min, `${mode}: ${fg} on ${bg} is ${ratio.toFixed(2)}, needs ${min}`);
  }
});
