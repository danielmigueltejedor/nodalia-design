import { test } from 'node:test';
import assert from 'node:assert/strict';
import { render, dark } from '../volumetric/build.mjs';
import { ROLES, ROLES_DARK } from '../volumetric/scene.mjs';

const icons = await render();

test('volumetric icons are kebab-case, unique and grouped', () => {
  assert.ok(icons.length >= 300);
  assert.equal(new Set(icons.map((i) => i.name)).size, icons.length);
  for (const { name, group } of icons) {
    assert.match(name, /^[a-z0-9]+(-[a-z0-9]+)*$/);
    assert.match(group, /^[a-z]+$/);
  }
});

test('volumetric icons use only role colours, so an app can recolour them', () => {
  const roles = new Set(Object.values(ROLES));
  assert.equal(roles.size, Object.keys(ROLES).length, 'every role has its own colour');
  assert.deepEqual(Object.keys(ROLES_DARK).sort(), Object.keys(ROLES).sort());
  for (const { name, svg } of icons) {
    assert.match(svg, /^<svg xmlns="http:\/\/www\.w3\.org\/2000\/svg" viewBox="0 0 64 64">/, name);
    assert.ok(!/style=|class=|transform=|<image|<text/.test(svg), name);
    for (const [, colour] of svg.matchAll(/(?:fill|stroke)="([^"]+)"/g)) {
      assert.ok(colour === 'none' || roles.has(colour), `${name}: ${colour}`);
    }
  }
});

test('volumetric path data is absolute M L C Q A Z with finite numbers', () => {
  for (const { name, svg } of icons) {
    for (const [, d] of svg.matchAll(/ d="([^"]+)"/g)) {
      assert.match(d, /^[MLCQAZ0-9 .\-]+$/, name);
      assert.ok(!/NaN|Infinity/.test(d), name);
    }
  }
});

test('the dark rendering swaps every role colour', () => {
  for (const { name, svg } of icons) {
    const d = dark(svg);
    for (const hex of Object.values(ROLES)) assert.ok(!d.includes(`"${hex}"`), `${name}: ${hex}`);
  }
});
