// Generates dist/:
//   svg/<name>.svg    drawn in currentColor: for the web and anything that inherits the text colour
//   mono/<name>.svg   drawn in black: for toolkits that tint by alpha (Qt icon engines, egui textures)
//   sprite.svg        every icon as a <symbol id="ndl-<name>">
//   icons.json        names and groups
//   preview.html      every icon by group, to look the set over in a browser
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

/** A self-contained page showing every icon by group, in light and dark. */
export function preview() {
  const groups = new Map();
  for (const name of names()) groups.set(ICONS[name].group, [...(groups.get(ICONS[name].group) ?? []), name]);
  const section = ([group, list]) => `<h2>${group} <small>${list.length}</small></h2><div class="grid">${list.map((n) => `<figure><svg viewBox="0 0 ${SIZE} ${SIZE}" ${ATTRS} stroke="currentColor">${ICONS[n].body.join('')}</svg><figcaption>${n}</figcaption></figure>`).join('')}</div>`;
  return `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>NDL icons</title>
<style>
:root { color-scheme: light dark; --bg: #F3F4F8; --panel: #FFFFFF; --ink: #11131B; --soft: #444A5C; --line: #E8EAF0; }
@media (prefers-color-scheme: dark) { :root { --bg: #07080C; --panel: #11131B; --ink: #F3F4F8; --soft: #B4B8C6; --line: #1F2330; } }
body { margin: 0; padding: 24px 16px; background: var(--bg); color: var(--ink); font: 13px/20px system-ui, sans-serif; }
h1 { font-size: 24px; line-height: 32px; margin: 0 0 4px; } p { margin: 0 0 16px; color: var(--soft); }
h2 { font-size: 15px; margin: 24px 0 8px; text-transform: capitalize; } small { color: var(--soft); font-weight: 400; }
.grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(112px, 1fr)); gap: 8px; }
figure { margin: 0; padding: 12px 4px 8px; text-align: center; background: var(--panel); border: 1px solid var(--line); border-radius: 10px; }
svg { width: 24px; height: 24px; } figcaption { margin-top: 6px; font-size: 11px; color: var(--soft); overflow-wrap: anywhere; }
</style></head><body>
<h1>NDL icons</h1><p>${names().length} icons on a ${SIZE} px grid with a ${STROKE} px stroke, shown at their real size.</p>
${[...groups.entries()].map(section).join('\n')}
</body></html>
`;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const out = new URL('../dist/', import.meta.url).pathname;
  for (const dir of ['svg', 'mono']) mkdirSync(join(out, dir), { recursive: true });
  for (const name of names()) {
    writeFileSync(join(out, 'svg', `${name}.svg`), svg(name));
    writeFileSync(join(out, 'mono', `${name}.svg`), svg(name, '#000'));
  }
  writeFileSync(join(out, 'sprite.svg'), sprite());
  writeFileSync(join(out, 'preview.html'), preview());
  writeFileSync(join(out, 'icons.json'), `${JSON.stringify(index(), null, 2)}\n`);
  console.log(`@nodalia/icons: ${names().length} icons -> dist/`);
}
