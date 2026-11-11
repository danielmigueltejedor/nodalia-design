// Per-app accents. Each app gives one key colour; NDL derives a full accent ramp from it that has,
// step by step, the same luminance as the default ramp. Contrast depends only on luminance, so every
// app accent meets the contrast the default accent meets, whatever its hue.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { parseHex, luminance } from './color.mjs';

export const DEFAULT_ACCENT = 'nodalia';
const STEPS = (tokens) => [...tokens.keys()].filter((p) => p.startsWith('palette.accent.')).map((p) => p.slice('palette.accent.'.length));

/** Key colours by app name, as authored in src/accents.json. */
export function loadAccents(dir) {
  const raw = JSON.parse(readFileSync(join(dir, 'accents.json'), 'utf8'));
  const entries = Object.entries(raw).filter(([name]) => !name.startsWith('$'));
  for (const [name, hex] of entries) {
    if (!/^[a-z]+$/.test(name) || name === DEFAULT_ACCENT) throw new Error(`accents.json: invalid accent name "${name}"`);
    parseHex(hex);
  }
  return new Map(entries);
}

// --- OKLCH <-> sRGB (Björn Ottosson's OKLab) ---
const toLinear = (c) => (c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4);
const toGamma = (c) => (c <= 0.0031308 ? 12.92 * c : 1.055 * c ** (1 / 2.4) - 0.055);

export function toOklch(hex) {
  const { r, g, b } = parseHex(hex);
  const [lr, lg, lb] = [r, g, b].map((v) => toLinear(v / 255));
  const l = Math.cbrt(0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb);
  const m = Math.cbrt(0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb);
  const s = Math.cbrt(0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb);
  const L = 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s;
  const A = 1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s;
  const B = 0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s;
  return { l: L, c: Math.hypot(A, B), h: Math.atan2(B, A) };
}

function linearRgb({ l: L, c, h }) {
  const A = c * Math.cos(h);
  const B = c * Math.sin(h);
  const l = (L + 0.3963377774 * A + 0.2158037573 * B) ** 3;
  const m = (L - 0.1055613458 * A - 0.0638541728 * B) ** 3;
  const s = (L - 0.0894841775 * A - 1.291485548 * B) ** 3;
  return [
    4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
    -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
    -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
  ];
}

const inGamut = (rgb) => rgb.every((v) => v >= -1e-6 && v <= 1 + 1e-6);
const hexOf = (rgb) => '#' + rgb.map((v) => Math.round(Math.min(1, Math.max(0, toGamma(Math.min(1, Math.max(0, v))))) * 255).toString(16).padStart(2, '0')).join('').toUpperCase();

/** The largest chroma sRGB can show at this lightness and hue. */
function maxChroma(l, h) {
  let lo = 0;
  let hi = 0.4;
  for (let i = 0; i < 24; i += 1) {
    const mid = (lo + hi) / 2;
    if (inGamut(linearRgb({ l, c: mid, h }))) lo = mid; else hi = mid;
  }
  return lo;
}

/** How saturated a colour is for its lightness and hue: 0 is grey, 1 is the edge of sRGB. */
function saturation({ l, c, h }) {
  const limit = maxChroma(l, h);
  return limit < 1e-4 ? 0 : Math.min(1, c / limit);
}

/**
 * The accent ramp of an app: same steps as the default ramp and the hue of `keyHex`.
 * - Luminance: each step has the luminance of the default step, so contrast is the same.
 * - Saturation: as saturated as the key colour is for its own lightness, following the default
 *   ramp's profile (softer at the ends). A grey key stays grey; a deep one stays rich.
 * @param {Map<string, {value: string}>} tokens
 * @param {string} keyHex
 * @returns {Record<string, string>} step -> #RRGGBB
 */
export function accentRamp(tokens, keyHex) {
  const steps = STEPS(tokens);
  const reference = steps.map((s) => tokens.get(`palette.accent.${s}`).value);
  const profile = reference.map((hex) => saturation(toOklch(hex)));
  const peak = Math.max(...profile);
  const key = toOklch(keyHex);
  const keySaturation = saturation(key);
  const ramp = {};
  steps.forEach((step, i) => {
    const target = luminance(parseHex(reference[i]));
    const share = Math.min(1, keySaturation * (profile[i] / peak));
    const colour = (l) => linearRgb({ l, c: share * maxChroma(l, key.h), h: key.h });
    let lo = 0;
    let hi = 1;
    for (let n = 0; n < 40; n += 1) {
      const mid = (lo + hi) / 2;
      const rgb = colour(mid).map((v) => Math.min(1, Math.max(0, v)));
      const y = 0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2];
      if (y < target) lo = mid; else hi = mid;
    }
    // Rounding to 8 bits may land a hair on the wrong side. Dark steps carry light text and must
    // not get lighter than the reference; light steps carry dark text and must not get darker.
    const dark = target < 0.25;
    let l = (lo + hi) / 2;
    let hex = hexOf(colour(l));
    for (let n = 0; n < 200; n += 1) {
      const y = luminance(parseHex(hex));
      if (dark ? y <= target : y >= target) break;
      l += dark ? -0.0005 : 0.0005;
      hex = hexOf(colour(l));
    }
    ramp[step] = hex;
  });
  return ramp;
}

/** A copy of the token set whose accent ramp is `ramp`. Everything else is shared. */
export function withAccent(tokens, ramp) {
  const copy = new Map(tokens);
  for (const [step, hex] of Object.entries(ramp)) {
    const path = `palette.accent.${step}`;
    if (!copy.has(path)) throw new Error(`unknown accent step "${step}"`);
    copy.set(path, { ...copy.get(path), value: hex });
  }
  return copy;
}

/** Every accent by name, the default first: name -> { key, ramp, tokens }. */
export function accentSets(tokens, dir) {
  const steps = STEPS(tokens);
  const sets = new Map([[DEFAULT_ACCENT, { key: null, ramp: Object.fromEntries(steps.map((s) => [s, tokens.get(`palette.accent.${s}`).value])), tokens }]]);
  for (const [name, key] of loadAccents(dir)) {
    const ramp = accentRamp(tokens, key);
    sets.set(name, { key, ramp, tokens: withAccent(tokens, ramp) });
  }
  return sets;
}

/** CSS that switches the accent ramp: `<html data-accent="vortia">`. The semantic tokens alias the ramp, so they follow. */
export function accentCss(sets, cssName) {
  return [...sets].filter(([name]) => name !== DEFAULT_ACCENT).map(([name, set]) =>
    `:root[data-accent="${name}"] {\n${Object.entries(set.ramp).map(([step, hex]) => `  ${cssName(`palette.accent.${step}`)}: ${hex.toLowerCase()};`).join('\n')}\n}`).join('\n\n');
}

/** For each accent and mode, the semantic colours that differ from the default accent: path -> value. */
export function accentOverrides(sets, modes, resolve) {
  const base = sets.get(DEFAULT_ACCENT).tokens;
  const colours = [...base.values()].filter((t) => t.type === 'color' && t.path.startsWith('color.')).map((t) => t.path);
  return new Map([...sets].map(([name, set]) => [name, Object.fromEntries(modes.map((m) => [m,
    Object.fromEntries(colours.filter((p) => resolve(set.tokens, p, m) !== resolve(base, p, m)).map((p) => [p, resolve(set.tokens, p, m)]))]))]));
}
