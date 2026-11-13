import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { MODES, SOURCE_DIR, loadTokens, rawValue, resolve, parseHex } from '@nodalia/tokens/lib';
import { ICONS_OUTPUT, OUTPUT, iconsSource, rustSource } from '../scripts/build.mjs';

const tokens = loadTokens(SOURCE_DIR);
const src = rustSource();
const colours = [...tokens.values()].filter((t) => t.type === 'color' && t.path.startsWith('color.'));

test('the committed tokens.rs matches the tokens (run `npm run build -w @nodalia/egui`)', () => {
  assert.equal(readFileSync(OUTPUT, 'utf8'), src);
});

test('generation is deterministic', () => {
  assert.equal(rustSource(), src);
});

test('has one palette constant per mode with every colour token', () => {
  for (const mode of MODES) {
    const name = mode.toUpperCase().replace('-', '_');
    const body = new RegExp(`pub const ${name}: Palette = Palette \\{\\n([\\s\\S]*?)\\n\\};`).exec(src);
    assert.ok(body, mode);
    assert.equal((body[1].match(/^ {4}\w+: Rgba::new\(/gm) ?? []).length, colours.length, mode);
  }
});

test('colours keep their channels, alpha included', () => {
  const { r, g, b } = parseHex(resolve(tokens, 'color.surface.window', 'dark'));
  const dark = /pub const DARK: Palette = Palette \{\n([\s\S]*?)\n\};/.exec(src)[1];
  assert.ok(dark.includes(`surface_window: Rgba::new(${r}, ${g}, ${b}, 255),`));
  const scrim = parseHex(resolve(tokens, 'color.scrim.default', 'light'));
  assert.ok(src.includes(`scrim_default: Rgba::new(${scrim.r}, ${scrim.g}, ${scrim.b}, ${Math.round(scrim.a * 255)}),`));
});

test('exposes densities, metrics, durations and easings', () => {
  assert.match(src, /pub const REGULAR: Density = Density \{\n {8}control_height: 32\.0,/);
  assert.match(src, /pub mod space \{[\s\S]*?pub const MD: f32 = 12\.0;/);
  assert.match(src, /pub const NORMAL: u32 = 160;/);
  assert.match(src, /pub const STANDARD: \[f32; 4\] = \[0\.3, 0\.0, 0\.2, 1\.0\];/);
  assert.match(src, /pub const STATE: Transition = Transition \{ duration_ms: 160, delay_ms: 0, /);
});

test('output is balanced, has unique items per scope and no undefined/NaN', () => {
  assert.equal((src.match(/\{/g) ?? []).length, (src.match(/\}/g) ?? []).length);
  assert.ok(!/undefined|NaN|\[object/.test(src));
  const fields = /pub struct Palette \{\n([\s\S]*?)\n\}/.exec(src)[1].match(/pub (\w+):/g);
  assert.equal(new Set(fields).size, fields.length);
});

test('the committed icons/data.rs matches the icon set (run `npm run build -w @nodalia/egui`)', () => {
  assert.equal(readFileSync(ICONS_OUTPUT, 'utf8'), iconsSource());
  const names = [...iconsSource().matchAll(/^ {8}name: "([a-z-]+)",$/gm)].map((m) => m[1]);
  assert.deepEqual(names, [...names].sort());
  assert.ok(names.includes('line') && names.includes('dim-continue'));
});

test('window-control colours preserve shared semantic aliases', () => {
  const block = /pub mod window_color \{\n([\s\S]*?)\n\}/.exec(src)[1];
  for (const token of [...tokens.values()].filter((t) => t.type === 'color' && t.path.startsWith('component.window.'))) {
    const name = token.path.split('.').at(-1).replaceAll('-', '_');
    const semantic = /^\{color\.([^}]+)\}$/.exec(rawValue(token, 'light'));
    if (semantic && MODES.every((mode) => rawValue(token, mode) === rawValue(token, 'light'))) {
      const field = semantic[1].replaceAll(/[.-]/g, '_');
      assert.ok(block.includes(`pub const fn ${name}(mode: Mode) -> Rgba {\n        mode.palette().${field}\n    }`));
    } else if (MODES.every((mode) => resolve(tokens, token.path, mode) === resolve(tokens, token.path, 'light'))) {
      const { r, g, b, a } = parseHex(resolve(tokens, token.path, 'light'));
      assert.ok(block.includes(`pub const ${name.toUpperCase()}: Rgba = Rgba::new(${r}, ${g}, ${b}, ${Math.round(a * 255)});`));
    } else {
      assert.ok(block.includes(`pub const fn ${name}(mode: Mode) -> Rgba {\n        match mode {`));
    }
  }
});
