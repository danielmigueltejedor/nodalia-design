// Loads DTCG token files and resolves aliases per appearance mode.
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

export const MODES = ['light', 'dark', 'light-hc', 'dark-hc'];

/** Fallback chain per mode: the first mode in the chain that defines a value wins. */
export const MODE_CHAIN = {
  light: [],
  dark: ['dark'],
  'light-hc': ['light-hc'],
  'dark-hc': ['dark-hc', 'dark'],
};

export const TYPES = new Set([
  'color', 'dimension', 'duration', 'cubicBezier', 'fontFamily', 'fontWeight',
  'shadow', 'typography', 'transition', 'number',
]);

const MODES_EXT = 'com.nodalia.modes';
const REF = /^\{([^{}]+)\}$/;

export const isRef = (v) => typeof v === 'string' && REF.test(v);
export const refPath = (v) => v.slice(1, -1);

/** @typedef {{path: string, type: string, value: unknown, modes?: Record<string, unknown>, description?: string}} Token */

/**
 * Reads every `*.tokens.json` file in `dir` (sorted by name) into one flat map.
 * @param {string} dir
 * @returns {Map<string, Token>}
 */
export function loadTokens(dir) {
  const tokens = new Map();
  const files = readdirSync(dir).filter((f) => f.endsWith('.tokens.json')).sort();
  if (files.length === 0) throw new Error(`no *.tokens.json files in ${dir}`);
  for (const file of files) {
    const tree = JSON.parse(readFileSync(join(dir, file), 'utf8'));
    walk(tree, [], undefined, tokens, file);
  }
  return tokens;
}

function walk(node, path, inheritedType, out, file) {
  for (const [key, child] of Object.entries(node)) {
    if (key.startsWith('$')) continue;
    if (child === null || typeof child !== 'object') {
      throw new Error(`${file}: "${[...path, key].join('.')}" must be a group or a token`);
    }
    const here = [...path, key];
    const type = child.$type ?? inheritedType;
    if ('$value' in child) {
      const name = here.join('.');
      if (out.has(name)) throw new Error(`${file}: duplicate token "${name}"`);
      if (!type || !TYPES.has(type)) throw new Error(`${file}: token "${name}" has missing or unknown $type "${type}"`);
      out.set(name, {
        path: name,
        type,
        value: child.$value,
        modes: child.$extensions?.[MODES_EXT],
        description: child.$description,
      });
    } else {
      walk(child, here, type, out, file);
    }
  }
}

/** Raw (alias-preserving) value of a token for a mode, following the fallback chain. */
export function rawValue(token, mode) {
  for (const m of MODE_CHAIN[mode] ?? []) {
    if (token.modes && m in token.modes) return token.modes[m];
  }
  return token.value;
}

/** True when the token defines at least one mode-specific value. */
export const hasModes = (token) => token.modes !== undefined && Object.keys(token.modes).length > 0;

/**
 * Fully resolved value of a token in a mode: every alias replaced by its target's value.
 * Throws on a missing target, a cycle, or an alias to a token of a different type.
 */
export function resolve(tokens, path, mode, stack = []) {
  const token = tokens.get(path);
  if (!token) throw new Error(`unknown token "${path}"${stack.length ? ` (referenced from "${stack.at(-1)}")` : ''}`);
  if (stack.includes(path)) throw new Error(`alias cycle: ${[...stack, path].join(' -> ')}`);
  return deref(tokens, rawValue(token, mode), token.type, mode, [...stack, path]);
}

function deref(tokens, value, type, mode, stack) {
  if (isRef(value)) {
    const target = refPath(value);
    const t = tokens.get(target);
    if (t && t.type !== type) {
      throw new Error(`"${stack.at(-1)}" expects ${type} but "${target}" is ${t.type}`);
    }
    return resolve(tokens, target, mode, stack);
  }
  if (type === 'shadow' || type === 'typography' || type === 'transition') {
    const members = MEMBER_TYPES[type];
    return Object.fromEntries(
      Object.entries(value).map(([k, v]) => [k, deref(tokens, v, members[k], mode, stack)]),
    );
  }
  return value;
}

/** Member types of composite tokens, used to type-check aliases inside them. */
export const MEMBER_TYPES = {
  shadow: { color: 'color', offsetX: 'dimension', offsetY: 'dimension', blur: 'dimension', spread: 'dimension' },
  typography: { fontFamily: 'fontFamily', fontWeight: 'fontWeight', fontSize: 'dimension', lineHeight: 'dimension' },
  transition: { duration: 'duration', delay: 'duration', timingFunction: 'cubicBezier' },
};

/** Every alias target mentioned by a raw value. */
export function refsIn(value) {
  if (isRef(value)) return [refPath(value)];
  if (value && typeof value === 'object' && !Array.isArray(value)) return Object.values(value).flatMap(refsIn);
  return [];
}
