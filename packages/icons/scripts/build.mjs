// Generates dist/:
//   svg/<name>.svg    drawn in currentColor: for the web and anything that inherits the text colour
//   mono/<name>.svg   drawn in black: for toolkits that tint by alpha (Qt icon engines, egui textures)
//   sprite.svg        every icon as a <symbol id="ndl-<name>">
//   icons.json        names and groups
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { ICONS } from '../src/icons.mjs';

export const SIZE = 24;
export const STROKE = 1.75;
const ATTRS = `fill="none" stroke-width="${STROKE}" stroke-linecap="round" stroke-linejoin="round"`;

export const names = () => Object.keys(ICONS).sort();

/** One icon as a standalone SVG document. `ink` replaces currentColor when given. */
export function svg(name, ink = null) {
  const body = ICONS[name].body.join('');
  const text = `<svg xmlns="http://www.w3.org/2000/svg" width="${SIZE}" height="${SIZE}" viewBox="0 0 ${SIZE} ${SIZE}" ${ATTRS} stroke="currentColor">${body}</svg>\n`;
  return ink ? text.replaceAll('currentColor', ink) : text;
}

export function sprite() {
  const symbols = names().map((n) => `<symbol id="ndl-${n}" viewBox="0 0 ${SIZE} ${SIZE}" ${ATTRS} stroke="currentColor">${ICONS[n].body.join('')}</symbol>`);
  return `<svg xmlns="http://www.w3.org/2000/svg" style="display:none">\n${symbols.join('\n')}\n</svg>\n`;
}

export function index() {
  return { size: SIZE, stroke: STROKE, icons: names().map((name) => ({ name, group: ICONS[name].group })) };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const out = new URL('../dist/', import.meta.url).pathname;
  for (const dir of ['svg', 'mono']) mkdirSync(join(out, dir), { recursive: true });
  for (const name of names()) {
    writeFileSync(join(out, 'svg', `${name}.svg`), svg(name));
    writeFileSync(join(out, 'mono', `${name}.svg`), svg(name, '#000'));
  }
  writeFileSync(join(out, 'sprite.svg'), sprite());
  writeFileSync(join(out, 'icons.json'), `${JSON.stringify(index(), null, 2)}\n`);
  console.log(`@nodalia/icons: ${names().length} icons -> dist/`);
}
