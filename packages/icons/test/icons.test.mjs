import { test } from 'node:test';
import assert from 'node:assert/strict';
import { ICONS } from '../src/icons.mjs';
import { SIZE, index, names, sprite, svg } from '../scripts/build.mjs';

const numbers = (text) => [...text.matchAll(/-?\d*\.?\d+/g)].map((m) => Number(m[0]));

test('names are kebab-case, unique and grouped', () => {
  assert.ok(names().length >= 55);
  for (const name of names()) {
    assert.match(name, /^[a-z]+(-[a-z]+)*$/);
    assert.match(ICONS[name].group, /^[a-z]+$/);
    assert.ok(ICONS[name].body.length > 0, name);
  }
});

test('icons only use plain shapes and never set a colour of their own', () => {
  for (const name of names()) for (const part of ICONS[name].body) {
    assert.match(part, /^<(path|circle|rect) [^<>]*\/>$/, `${name}: ${part}`);
    assert.ok(!/#[0-9a-fA-F]{3,8}|rgb|style=|class=|transform=/.test(part), `${name}: ${part}`);
    for (const colour of part.matchAll(/(?:fill|stroke)="([^"]+)"/g)) assert.ok(['currentColor', 'none'].includes(colour[1]), `${name}: ${colour[0]}`);
  }
});

test('circles and rectangles stay inside the view box', () => {
  for (const name of names()) for (const part of ICONS[name].body) {
    const get = (attr) => Number(new RegExp(` ${attr}="([^"]+)"`).exec(part)?.[1]);
    if (part.startsWith('<circle')) {
      for (const centre of [get('cx'), get('cy')]) assert.ok(centre - get('r') >= 1 && centre + get('r') <= SIZE - 1, `${name}: ${part}`);
    } else if (part.startsWith('<rect')) {
      assert.ok(get('x') >= 1 && get('y') >= 1 && get('x') + get('width') <= SIZE - 1 && get('y') + get('height') <= SIZE - 1, `${name}: ${part}`);
    }
  }
});

test('paths start inside the view box and stay on a quarter-pixel grid', () => {
  for (const name of names()) for (const part of ICONS[name].body.filter((p) => p.startsWith('<path'))) {
    const d = / d="([^"]+)"/.exec(part)[1];
    for (const start of d.matchAll(/M\s*(-?\d*\.?\d+)[ ,]?(-?\d*\.?\d+)/g)) {
      for (const value of [Number(start[1]), Number(start[2])]) assert.ok(value >= 1 && value <= SIZE - 1, `${name}: M ${start[1]} ${start[2]}`);
    }
    for (const value of numbers(d)) assert.ok(Number.isInteger(Math.round(value * 100)) && Math.abs(value * 100 - Math.round(value * 100)) < 1e-9, `${name}: ${value}`);
  }
});

test('standalone files are complete documents in the requested ink', () => {
  const current = svg('search');
  assert.match(current, /^<svg xmlns="http:\/\/www\.w3\.org\/2000\/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke-width="1\.75"/);
  assert.ok(current.includes('stroke="currentColor"') && current.endsWith('</svg>\n'));
  const mono = svg('geometry', '#000');
  assert.ok(!mono.includes('currentColor') && mono.includes('fill="#000"'));
});

test('the sprite and the index list every icon once', () => {
  const ids = [...sprite().matchAll(/<symbol id="ndl-([a-z-]+)"/g)].map((m) => m[1]);
  assert.deepEqual(ids, names());
  assert.deepEqual(index().icons.map((i) => i.name), names());
  assert.equal(new Set(ids).size, ids.length);
});
