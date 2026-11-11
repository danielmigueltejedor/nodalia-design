// Generates, for Qt Widgets apps:
//   dist/nodalia-qt.json            tokens flattened to `group.name` (colours per mode, metrics per density)
//   dist/nodalia.qss                the style-sheet template, with {{group.name}} placeholders
//   dist/nodalia-<mode>-<density>.qss   the template rendered, for apps without a template step
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { MODES, SOURCE_DIR, accentSets, accentOverrides, loadTokens, resolve, parseHex, px, ms } from '@nodalia/tokens/lib';

const tokens = loadTokens(SOURCE_DIR);
const all = [...tokens.values()];
const camel = (text) => text.split(/[.-]/).map((p, i) => (i === 0 ? p : p[0].toUpperCase() + p.slice(1))).join('');
const light = (t) => resolve(tokens, t.path, 'light');
const group = (prefix, type = 'dimension') => all.filter((t) => t.path.startsWith(`${prefix}.`) && t.type === type);
const dims = (prefix) => Object.fromEntries(group(prefix).map((t) => [camel(t.path.slice(prefix.length + 1)), px(light(t))]));

/** Qt style sheets take `#rrggbb` or `rgba(r, g, b, a)` with alpha 0-255. */
export function qssColor(hex) {
  const { r, g, b, a } = parseHex(hex);
  return a === 1 ? hex.slice(0, 7).toUpperCase() : `rgba(${r}, ${g}, ${b}, ${Math.round(a * 255)})`;
}

export const DENSITIES = [...new Set(all.filter((t) => t.path.startsWith('density.')).map((t) => t.path.split('.')[1]))];

export function flatTokens() {
  const colors = all.filter((t) => t.type === 'color' && t.path.startsWith('color.'));
  const componentColors = all.filter((t) => t.type === 'color' && t.path.startsWith('component.'));
  const version = JSON.parse(readFileSync(new URL('../package.json', import.meta.url), 'utf8')).version;
  return {
    name: 'Nodalia Design Language',
    version,
    modes: MODES,
    densities: DENSITIES,
    // Looked up under the active mode: color.<name>
    // Component colours (the window's close) join them under their component name: color.windowClose.
    color: Object.fromEntries(MODES.map((m) => [m, Object.fromEntries([
      ...colors.map((t) => [camel(t.path.slice('color.'.length)), qssColor(resolve(tokens, t.path, m))]),
      ...componentColors.map((t) => [camel(t.path.slice('component.'.length)), qssColor(resolve(tokens, t.path, m))]),
    ])])),
    // Per app and mode, the colours that replace those of `color` (empty for the default accent).
    accent: Object.fromEntries([...accentOverrides(accentSets(tokens, SOURCE_DIR), MODES, resolve)].map(([name, modes]) => [name,
      Object.fromEntries(Object.entries(modes).map(([m, values]) => [m, Object.fromEntries(Object.entries(values).map(([path, hex]) => [camel(path.slice('color.'.length)), qssColor(hex)]))]))])),
    // Looked up under the active density: density.<name>
    // Qt draws no corner radius past half the side, so "full" (9999) squares a control off. Each
    // density gets the pill radius its control height allows.
    density: Object.fromEntries(DENSITIES.map((d) => {
      const own = dims(`density.${d}`);
      return [d, { ...own, pillRadius: Math.floor(own.controlHeight / 2) }];
    })),
    space: dims('space'),
    radius: dims('radius'),
    border: dims('border-width'),
    font: dims('font.size'),
    line: dims('font.line-height'),
    size: dims('size'),
    component: dims('component'),
    weight: Object.fromEntries(group('font.weight', 'fontWeight').map((t) => [camel(t.path.slice('font.weight.'.length)), light(t)])),
    duration: Object.fromEntries(group('duration', 'duration').map((t) => [camel(t.path.slice('duration.'.length)), ms(light(t))])),
  };
}

const PLACEHOLDER = /\{\{([A-Za-z]+)\.([A-Za-z]+)\}\}/g;

/** Value of `group.name` for a mode, density and app accent, or undefined. */
export function tokenValue(flat, mode, density, groupName, name, accent = 'nodalia') {
  let source = flat[groupName];
  if (groupName === 'color') source = { ...source?.[mode], ...flat.accent?.[accent]?.[mode] };
  if (groupName === 'density') source = source?.[density];
  const value = source?.[name];
  return value === undefined || typeof value === 'object' ? undefined : String(value);
}

/** Fills the template. Unknown references throw: a style sheet with a hole in it must not ship. */
export function render(template, flat, mode, density, accent = 'nodalia') {
  if (!(accent in flat.accent)) throw new Error(`unknown accent "${accent}"`);
  return template.replace(PLACEHOLDER, (whole, groupName, name) => {
    const value = tokenValue(flat, mode, density, groupName, name, accent);
    if (value === undefined) throw new Error(`unknown token ${groupName}.${name} in the style-sheet template`);
    return value;
  });
}

export const template = () => readFileSync(new URL('../src/nodalia.qss', import.meta.url), 'utf8');
export const placeholders = (text) => [...new Set([...text.matchAll(PLACEHOLDER)].map((m) => `${m[1]}.${m[2]}`))];

if (import.meta.url === `file://${process.argv[1]}`) {
  const out = new URL('../dist/', import.meta.url).pathname;
  mkdirSync(out, { recursive: true });
  const flat = flatTokens();
  const source = template();
  writeFileSync(join(out, 'nodalia-qt.json'), `${JSON.stringify(flat, null, 2)}\n`);
  writeFileSync(join(out, 'nodalia.qss'), source);
  for (const mode of MODES) for (const density of DENSITIES) writeFileSync(join(out, `nodalia-${mode}-${density}.qss`), render(source, flat, mode, density));
  console.log(`@nodalia/qtwidgets: nodalia-qt.json, nodalia.qss and ${MODES.length * DENSITIES.length} rendered sheets -> dist/`);
}
