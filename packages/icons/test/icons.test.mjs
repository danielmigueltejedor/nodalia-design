import { test } from 'node:test';
import assert from 'node:assert/strict';
import { ICONS } from '../src/icons.mjs';
import { SIZE, index, names, preview, sprite, svg } from '../scripts/build.mjs';

const numbers = (text) => [...text.matchAll(/-?\d*\.?\d+/g)].map((m) => Number(m[0]));

test('names are kebab-case, unique and grouped', () => {
  assert.ok(names().length >= 240);
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

// Every absolute point a path passes through (end points and Bézier controls; arcs by their ends).
function points(d) {
  const tokens = d.match(/[a-zA-Z]|-?(?:\d+\.?\d*|\.\d+)/g);
  const out = [];
  let [i, x, y, sx, sy, cmd, c2] = [0, 0, 0, 0, 0, '', null];
  const next = () => Number(tokens[i++]);
  while (i < tokens.length) {
    if (/[a-zA-Z]/.test(tokens[i])) cmd = tokens[i++];
    else if (cmd === '' || cmd.toUpperCase() === 'Z') throw new Error(`a number without a command: ${d}`);
    const [ox, oy] = cmd === cmd.toLowerCase() ? [x, y] : [0, 0];
    const kind = cmd.toUpperCase();
    // An S reflects the previous C or S's second control point about the current point.
    const reflected = c2 ? [2 * x - c2[0], 2 * y - c2[1]] : [x, y];
    c2 = null;
    if (kind === 'Z') [x, y] = [sx, sy];
    else if (kind === 'H') x = ox + next();
    else if (kind === 'V') y = oy + next();
    else {
      if (kind === 'A') for (let k = 0; k < 5; k++) next();
      if (kind === 'S') out.push(reflected);
      if (kind === 'C') out.push([ox + next(), oy + next()]);
      if (kind === 'C' || kind === 'S') out.push((c2 = [ox + next(), oy + next()]));
      [x, y] = [ox + next(), oy + next()];
      if (kind === 'M') [sx, sy, cmd] = [x, y, cmd === 'm' ? 'l' : 'L'];
    }
    out.push([x, y]);
  }
  return out;
}

test('paths keep two pixels clear of every edge, so no stroke is clipped', () => {
  for (const name of names()) for (const part of ICONS[name].body.filter((p) => p.startsWith('<path'))) {
    for (const [x, y] of points(/ d="([^"]+)"/.exec(part)[1])) {
      assert.ok(Math.min(x, y) >= 2 - 1e-9 && Math.max(x, y) <= SIZE - 2 + 1e-9, `${name}: ${x}, ${y}`);
    }
  }
});

test('the path reader rejects stray numbers and follows reflected controls', () => {
  assert.throws(() => points('M2 2Z5'));
  assert.ok(points('M3 12C3 12 3 12 21 12S21 12 21 18').some(([x]) => x === 39));
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

test('the preview page shows every icon and loads nothing from outside', () => {
  const page = preview();
  assert.equal((page.match(/<figure>/g) ?? []).length, names().length);
  assert.ok(!/https?:\/\/(?!www\.w3\.org)/.test(page) && !/<script|<link|<img/.test(page));
});

test('the bold style fills only elements that exist, and its own bodies follow the same rules', async () => {
  const { boldBody } = await import('../src/bold.mjs');
  let solid = 0;
  for (const name of names()) {
    const icon = ICONS[name];
    for (const i of icon.fill ?? []) assert.ok(Number.isInteger(i) && i >= 0 && i < icon.body.length, `${name}: fill ${i}`);
    for (const part of boldBody(name)) {
      assert.match(part, /^<(path|circle|rect) [^<>]*\/>$/, `${name}: ${part}`);
      assert.ok(!/#[0-9a-fA-F]{3,8}|rgb|style=|class=|transform=/.test(part), `${name}: ${part}`);
    }
    if (boldBody(name).some((p) => /fill="currentColor"/.test(p) && !/stroke="none"/.test(p))) solid += 1;
  }
  // The style's character: the media controls and the library's own icons are solid.
  assert.ok(solid >= 25, `${solid} solid icons`);
});

test('accents name elements that exist, and the CAD set is complete', () => {
  for (const name of names()) for (const i of ICONS[name].accent ?? []) assert.ok(Number.isInteger(i) && i >= 0 && i < ICONS[name].body.length, `${name}: accent ${i}`);
  const cad = names().filter((n) => ICONS[n].group === 'cad');
  assert.ok(cad.length >= 93, `${cad.length} CAD icons`);
  for (const shared of ['line', 'circle', 'rectangle', 'fillet', 'chamfer', 'dimension']) assert.ok(ICONS[shared].accent, shared);
});

// A name given twice in icons.mjs silently keeps only the later drawing.
test('no icon name is defined twice', async () => {
  const { readFile } = await import('node:fs/promises');
  const source = await readFile(new URL('../src/icons.mjs', import.meta.url), 'utf8');
  const seen = new Set();
  for (const [, name] of source.matchAll(/^ {2}"([a-z-]+)": \{/gm)) {
    assert.ok(!seen.has(name), `${name} is defined twice`);
    seen.add(name);
  }
});
