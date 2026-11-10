import { test } from 'node:test';
import assert from 'node:assert/strict';
import { MODES, SOURCE_DIR, loadTokens } from '@nodalia/tokens/lib';
import { qmlSource } from '../scripts/build.mjs';

const tokens = loadTokens(SOURCE_DIR);
const src = qmlSource();

test('declares a singleton QtObject with mode, density and reducedMotion controls', () => {
  assert.match(src, /^pragma Singleton$/m);
  assert.match(src, /^QtObject \{$/m);
  for (const p of ['property string mode', 'property string density', 'property bool reducedMotion']) assert.ok(src.includes(p), p);
});

test('has a palette per mode with every colour token', () => {
  const colours = [...tokens.values()].filter((x) => x.type === 'color' && x.path.startsWith('color.'));
  for (const mode of MODES) assert.ok(src.includes(`"${mode}": {`), mode);
  const entries = src.match(/"color[A-Z]\w+": "#[0-9A-F]{6,8}"/g) ?? [];
  assert.equal(entries.length, colours.length * MODES.length);
});

test('exposes one typed property per colour', () => {
  assert.match(src, /readonly property color colorSurfaceWindow: palettes\[mode\]\["colorSurfaceWindow"\]/);
});

test('translucent colours use QML #AARRGGBB order', () => {
  assert.match(src, /"colorScrimDefault": "#66/);
});

test('durations collapse under reduced motion and easings are BezierSpline arrays', () => {
  assert.match(src, /readonly property int durationNormal: reducedMotion \? 0 : 160/);
  assert.match(src, /readonly property var easingStandard: \[0\.3,0,0\.2,1,1,1\]/);
});

test('output is syntactically balanced and free of undefined/NaN', () => {
  assert.equal((src.match(/\{/g) ?? []).length, (src.match(/\}/g) ?? []).length);
  assert.equal((src.match(/\(/g) ?? []).length, (src.match(/\)/g) ?? []).length);
  assert.ok(!/undefined|NaN|\[object/.test(src));
});

test('property names are unique', () => {
  const names = [...src.matchAll(/^ {4}(?:readonly )?property \w+ (\w+):/gm)].map((m) => m[1]);
  assert.equal(new Set(names).size, names.length);
});
