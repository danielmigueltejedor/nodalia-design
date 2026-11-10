// Builds dist/: the catalog page, the reference demos and the assets they share.
import { copyFileSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { SOURCE_DIR, loadTokens, cssName } from '@nodalia/tokens/lib';

const root = new URL('../', import.meta.url).pathname;
const dist = join(root, 'dist');
const src = join(root, 'src');
const web = join(root, '..', '..', 'packages', 'web', 'dist');
const tokens = loadTokens(SOURCE_DIR);
const esc = (s) => s.replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);

const swatchGroups = ['surface', 'text', 'border', 'accent', 'control', 'selection', 'status'];
function swatches() {
  return swatchGroups.map((g) => {
    const items = [...tokens.values()].filter((t) => t.type === 'color' && t.path.startsWith(`color.${g}.`));
    return `<h3>${g}</h3><div class="swatches">${items.map((t) => `<div class="swatch"><div class="swatch__chip" style="background:var(${cssName(t.path)})"></div><code>${esc(t.path)}</code><span class="swatch__hex" data-hex="${cssName(t.path)}"></span></div>`).join('')}</div>`;
  }).join('\n');
}

const STATES = [['Default', {}], ['Hover', { 'data-state': 'hover' }], ['Pressed', { 'data-state': 'active' }], ['Focus', { 'data-state': 'focus' }], ['Disabled', { disabled: '' }]];
const attrs = (o) => Object.entries(o).map(([k, v]) => (v === '' ? k : `${k}="${v}"`)).join(' ');
const row = (label, make) => `<span class="ndl-type-small">${label}</span>${STATES.map(([, a]) => `<span>${make(a)}</span>`).join('')}`;

function stateGrid() {
  const head = `<span></span>${STATES.map(([n]) => `<span class="ndl-type-small">${n}</span>`).join('')}`;
  const rows = [
    ...['primary', 'secondary', 'subtle', 'danger'].map((v) => row(`Button ${v}`, (a) => `<button class="ndl-button" ${v === 'secondary' ? '' : `data-variant="${v}"`} ${attrs(a)}>Label</button>`)),
    row('Text field', (a) => `<input class="ndl-input" value="Value" aria-label="Text field" ${attrs(a)}>`),
    row('Checkbox', (a) => `<input class="ndl-check" type="checkbox" checked aria-label="Checkbox" ${attrs(a)}>`),
    row('Switch', (a) => `<input class="ndl-switch" type="checkbox" role="switch" checked aria-label="Switch" ${attrs(a)}>`),
    row('Tab', (a) => `<span role="tablist" aria-label="Tab state example"><button class="ndl-tab" role="tab" aria-selected="false" ${attrs(a)}>Tab</button></span>`),
    row('List item', (a) => `<ul class="ndl-list" style="inline-size:100%"><li class="ndl-list__item" ${attrs(a).replace('disabled', 'aria-disabled="true"')}>Item</li></ul>`),
  ];
  return `<div class="states">${head}${rows.join('')}</div>`;
}

const space = [...tokens.values()].filter((t) => t.path.startsWith('space.'));
const spaceScale = `<div class="scale">${space.map((t) => `<div class="scale__row"><code>${esc(t.path)} · ${t.value}</code><div class="scale__bar" style="inline-size:var(${cssName(t.path)})"></div></div>`).join('')}</div>`;
const types = [...tokens.values()].filter((t) => t.path.startsWith('typography.'));
const typeSpecimens = `<div class="spec spec--column">${types.map((t) => { const n = t.path.split('.')[1]; return `<div class="ndl-type-${n}">${n} · The quick brown fox jumps over the lazy dog 0123456789</div>`; }).join('')}</div>`;
const radii = [...tokens.values()].filter((t) => t.path.startsWith('radius.'));
const radiusRow = `<div class="spec">${radii.map((t) => `<div class="elev" style="border:var(--ndl-border-width-thin) solid var(--ndl-color-border-default);border-radius:var(${cssName(t.path)})"><code>${t.path.split('.')[1]}</code></div>`).join('')}</div>`;
const motionRoles = [...tokens.values()].filter((t) => t.path.startsWith('motion.'));
const motionRows = motionRoles.map((t) => { const n = t.path.split('.')[1]; return `<div class="spec"><code style="inline-size:8em">${t.path}</code><button class="ndl-button" data-motion-play="${n}">Play</button><div class="motion-demo" style="--_t:var(${cssName(t.path)})"></div></div>`; }).join('');

mkdirSync(join(dist, 'examples'), { recursive: true });
const fill = (html) => html
  .replace('<!--@swatches-->', swatches()).replace('<!--@states-->', stateGrid()).replace('<!--@space-->', spaceScale)
  .replace('<!--@type-->', typeSpecimens).replace('<!--@radius-->', radiusRow).replace('<!--@motion-->', motionRows);
writeFileSync(join(dist, 'index.html'), fill(readFileSync(join(src, 'index.html'), 'utf8')));
for (const f of readdirSync(join(src, 'examples'))) copyFileSync(join(src, 'examples', f), join(dist, 'examples', f));
for (const f of ['shared.mjs', 'catalog.css']) copyFileSync(join(src, f), join(dist, f));
copyFileSync(join(web, 'nodalia.css'), join(dist, 'nodalia.css'));
copyFileSync(join(web, 'behaviors.mjs'), join(dist, 'behaviors.mjs'));
console.log('@nodalia/catalog: dist/index.html + 3 reference demos');
