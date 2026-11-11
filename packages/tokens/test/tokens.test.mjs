import { test } from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { MODES, SOURCE_DIR, loadTokens, resolve, buildCss, validate, contrast, hasModes } from '../lib/index.mjs';

const tokens = loadTokens(SOURCE_DIR);
const t = (obj) => new Map(Object.entries(obj).map(([path, v]) => [path, { path, ...v }]));

test('the shipped token set is valid in every mode', () => {
  assert.deepEqual(validate(tokens), []);
});

test('covers every required token category', () => {
  const roots = new Set([...tokens.keys()].map((p) => p.split('.')[0]));
  for (const r of ['palette', 'color', 'typography', 'space', 'radius', 'border-width', 'elevation', 'duration', 'easing', 'motion', 'size', 'density', 'breakpoint', 'component']) {
    assert.ok(roots.has(r), `missing category ${r}`);
  }
  for (const d of ['compact', 'regular', 'comfortable']) assert.ok(tokens.has(`density.${d}.control-height`));
});

test('dark and high-contrast modes really differ from light', () => {
  const moded = [...tokens.values()].filter(hasModes);
  assert.ok(moded.length > 30);
  for (const m of ['dark', 'light-hc', 'dark-hc']) {
    const changed = moded.filter((x) => JSON.stringify(resolve(tokens, x.path, m)) !== JSON.stringify(resolve(tokens, x.path, 'light')));
    assert.ok(changed.length > 10, `${m} changes only ${changed.length} tokens`);
  }
});

test('brand cyan is the dark accent', () => {
  assert.equal(resolve(tokens, 'color.accent.default', 'dark'), '#2CC5D3');
});

test('every text role meets WCAG AA on every surface in every mode', () => {
  for (const mode of MODES) {
    for (const s of ['window', 'panel', 'raised', 'sunken', 'overlay']) {
      for (const role of ['primary', 'secondary', 'link']) {
        const r = contrast(resolve(tokens, `color.text.${role}`, mode), resolve(tokens, `color.surface.${s}`, mode));
        assert.ok(r >= 4.5, `${mode}: text.${role} on ${s} = ${r.toFixed(2)}`);
      }
    }
  }
});

test('high-contrast modes reach AAA for primary text', () => {
  for (const mode of ['light-hc', 'dark-hc']) {
    assert.ok(contrast(resolve(tokens, 'color.text.primary', mode), resolve(tokens, 'color.surface.panel', mode)) >= 7);
  }
});

test('contrast maths matches known values', () => {
  assert.equal(contrast('#000000', '#FFFFFF').toFixed(2), '21.00');
  assert.equal(contrast('#777777', '#FFFFFF').toFixed(2), '4.48');
});

test('CSS generation is deterministic', () => {
  const hash = (s) => createHash('sha256').update(s).digest('hex');
  assert.equal(hash(buildCss(loadTokens(SOURCE_DIR))), hash(buildCss(loadTokens(SOURCE_DIR))));
});

test('CSS uses rem for scalable dimensions and px for hairlines', () => {
  const css = buildCss(tokens);
  assert.match(css, /--ndl-font-size-body: 0\.8125rem;/);
  assert.match(css, /--ndl-space-md: 0\.75rem;/);
  assert.match(css, /--ndl-border-width-thin: 1px;/);
  assert.match(css, /--ndl-radius-md: 6px;/);
});

test('component tokens keep aliases as var() so modes propagate', () => {
  assert.match(buildCss(tokens), /--ndl-component-button-font: var\(--ndl-typography-label\);/);
});

test('every mode block is complete: no token is missing from a mode selector', () => {
  const css = buildCss(tokens);
  const moded = [...tokens.values()].filter((x) => x.path.startsWith('color.') && hasModes(x));
  const blocks = css.split(/\n(?=:root|@media)/).filter((b) => b.includes('data-theme') || b.includes('data-contrast'));
  assert.ok(blocks.length >= 7);
  for (const b of blocks) {
    for (const x of moded) assert.ok(b.includes(`--ndl-${x.path.replaceAll('.', '-')}:`), `block missing ${x.path}: ${b.slice(0, 80)}`);
  }
});

test('reduced motion collapses every transition duration', () => {
  const css = buildCss(tokens);
  const block = css.slice(css.indexOf('prefers-reduced-motion'));
  const decls = block.split('\n').filter((l) => l.includes('--ndl-motion-'));
  assert.ok(decls.length >= 10);
  for (const d of decls) assert.match(d, /: 0ms /);
});

test('validate reports alias cycles, missing targets and type mismatches', () => {
  const problems = validate(t({
    'a.x': { type: 'color', value: '{a.y}' }, 'a.y': { type: 'color', value: '{a.x}' },
    'b.x': { type: 'color', value: '{nope.nothing}' },
    'c.x': { type: 'color', value: '{c.d}' }, 'c.d': { type: 'dimension', value: '4px' },
  }));
  const text = problems.join('\n');
  assert.match(text, /alias cycle/);
  assert.match(text, /unknown token "nope.nothing"/);
  assert.match(text, /expects color but "c.d" is dimension/);
});

test('validate rejects aliases inside primitives and component-to-component aliases', () => {
  const problems = validate(t({
    'palette.a': { type: 'color', value: '#000000' },
    'space.x': { type: 'dimension', value: '{space.y}' }, 'space.y': { type: 'dimension', value: '4px' },
    'component.a.r': { type: 'dimension', value: '{component.b.r}' }, 'component.b.r': { type: 'dimension', value: '4px' },
  }));
  assert.ok(problems.some((p) => p.includes('primitive "space.x"')));
  assert.ok(problems.some((p) => p.includes('may not alias another component')));
});

test('validate flags insufficient contrast', () => {
  const mk = (fg, bg) => {
    const base = Object.fromEntries([...tokens].map(([k, v]) => [k, { ...v }]));
    base['color.text.primary'] = { ...base['color.text.primary'], value: fg, modes: undefined };
    base['color.surface.panel'] = { ...base['color.surface.panel'], value: bg, modes: undefined };
    return validate(t(base));
  };
  assert.ok(mk('#777777', '#FFFFFF').some((p) => p.includes('color.text.primary on color.surface.panel')));
});

// --- App accents ---
import { accentCss, cssName as accentCssName } from '../lib/index.mjs';
import { DEFAULT_ACCENT, accentSets, accentRamp, luminance, parseHex, toOklch, validate as validateAll } from '../lib/index.mjs';

const accentTokens = loadTokens(SOURCE_DIR);
const accents = accentSets(accentTokens, SOURCE_DIR);

test('every app accent passes the full validation, contrast included, in the four modes', () => {
  assert.ok(accents.size >= 18);
  assert.equal([...accents.keys()][0], DEFAULT_ACCENT);
  for (const [name, set] of accents) assert.deepEqual(validateAll(set.tokens), [], name);
});

test('an accent ramp keeps the luminance of the default ramp, step by step', () => {
  const reference = accents.get(DEFAULT_ACCENT).ramp;
  for (const [name, set] of accents) for (const [step, hex] of Object.entries(set.ramp)) {
    const want = luminance(parseHex(reference[step]));
    const got = luminance(parseHex(hex));
    assert.ok(Math.abs(got - want) <= 0.012, `${name} ${step}: ${got.toFixed(4)} vs ${want.toFixed(4)}`);
  }
});

test('an accent ramp keeps the hue of its key colour and a grey key stays grey', () => {
  const hue = (hex) => ((toOklch(hex).h * 180) / Math.PI + 360) % 360;
  for (const [name, set] of accents) {
    if (!set.key) continue;
    const delta = Math.abs(hue(set.ramp['600']) - hue(set.key));
    assert.ok(Math.min(delta, 360 - delta) < 6, `${name}: hue drifted ${delta.toFixed(1)} degrees`);
  }
  assert.ok(toOklch(accents.get('papel').ramp['600']).c < toOklch(accents.get('vortia').ramp['600']).c / 2);
});

test('an accent only changes the accent ramp and what aliases it', () => {
  const set = accents.get('vortia');
  const changed = [...accentTokens.keys()].filter((p) => MODES.some((m) => JSON.stringify(resolve(set.tokens, p, m)) !== JSON.stringify(resolve(accentTokens, p, m))));
  assert.ok(changed.length > 11);
  for (const path of changed) assert.match(path, /^(palette\.accent\.|color\.(accent|selection\.bg$|border\.focus$|text\.link$)|component\.window\.minimize)/, path);
});

test('ramp generation is deterministic', () => {
  assert.deepEqual(accentRamp(accentTokens, '#0B55DB'), accentRamp(accentTokens, '#0B55DB'));
});

test('the accent CSS switches the ramp by attribute and leaves the default alone', () => {
  const css = accentCss(accents, accentCssName);
  assert.ok(css.includes(':root[data-accent="vortia"] {') && !css.includes('data-accent="nodalia"'));
  assert.equal((css.match(/:root\[data-accent=/g) ?? []).length, accents.size - 1);
  assert.equal((css.match(/--ndl-palette-accent-600:/g) ?? []).length, accents.size - 1);
});
