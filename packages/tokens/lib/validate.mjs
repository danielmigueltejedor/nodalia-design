// Static validation of the token set: aliases, layering and accessibility.
import { MODES, resolve, refsIn, rawValue } from './load.mjs';
import { contrast } from './color.mjs';

/**
 * Contrast requirements. `min` applies in light and dark; `minHc` in the high-contrast modes.
 * 4.5 is WCAG 1.4.3 (text), 3 is 1.4.11 (non-text), 7 is 1.4.6 (enhanced text).
 */
export const CONTRAST_PAIRS = [
  ...['window', 'panel', 'raised', 'sunken', 'overlay'].flatMap((s) => [
    ['color.text.primary', `color.surface.${s}`, 4.5, 7],
    ['color.text.secondary', `color.surface.${s}`, 4.5, 7],
    ['color.text.link', `color.surface.${s}`, 4.5, 7],
    ['color.accent.text', `color.surface.${s}`, 4.5, 7],
    ['color.border.focus', `color.surface.${s}`, 3, 4.5],
  ]),
  ['color.text.tertiary', 'color.surface.panel', 3, 4.5, 'placeholder and supporting text, never essential copy'],
  ['color.text.on-accent', 'color.accent.default', 4.5, 7],
  ['color.text.on-accent', 'color.accent.hover', 4.5, 7],
  ['color.text.on-accent', 'color.accent.pressed', 4.5, 7],
  ['color.text.primary', 'color.control.bg', 4.5, 7],
  ['color.text.primary', 'color.control.bg-hover', 4.5, 7],
  ['color.text.primary', 'color.control.bg-pressed', 4.5, 7],
  ['color.text.primary', 'color.control.field-bg', 4.5, 7],
  ['color.text.primary', 'color.selection.bg', 4.5, 7],
  ['color.text.primary', 'color.selection.bg-inactive', 4.5, 7],
  ['color.text.primary', 'color.accent.subtle', 4.5, 7],
  ['color.accent.text', 'color.accent.subtle', 4.5, 7],
  ['color.text.inverse', 'color.surface.inverse', 4.5, 7],
  ['color.border.strong', 'color.surface.panel', 3, 4.5],
  ['color.border.strong', 'color.control.field-bg', 3, 4.5],
  ['color.control.track', 'color.surface.panel', 3, 3],
  ['color.accent.default', 'color.surface.panel', 3, 4.5],
  // Pairs the GTK adapter maps onto libadwaita's success/warning/error/destructive colours.
  ...['success', 'warning', 'danger'].map((k) => ['color.text.on-accent', `color.status.${k}.fg`, 4.5, 7]),
  ...['success', 'warning', 'danger', 'info'].flatMap((k) => [
    [`color.status.${k}.fg`, `color.status.${k}.bg`, 4.5, 7],
    [`color.status.${k}.fg`, 'color.surface.panel', 4.5, 7],
  ]),
];

/**
 * @param {Map<string, import('./load.mjs').Token>} tokens
 * @returns {string[]} human-readable problems; empty when valid
 */
export function validate(tokens) {
  const problems = [];

  // 1. Every alias resolves, in every mode, to a token of the right type.
  for (const mode of MODES) {
    for (const t of tokens.values()) {
      try {
        resolve(tokens, t.path, mode);
      } catch (e) {
        problems.push(`[${mode}] ${e.message}`);
      }
    }
  }

  // 2. Layering: primitives hold literals; components alias semantic/primitive tokens only.
  for (const t of tokens.values()) {
    const layer = t.path.split('.')[0];
    const refs = [t.value, ...Object.values(t.modes ?? {})].flatMap(refsIn);
    if (PRIMITIVE_ROOTS.has(layer) && refs.length > 0) {
      problems.push(`primitive "${t.path}" must not use aliases`);
    }
    if (layer === 'color' && refs.some((r) => !r.startsWith('palette.'))) {
      problems.push(`semantic colour "${t.path}" may only alias palette tokens`);
    }
    if (layer === 'component' && refs.some((r) => r.startsWith('component.'))) {
      problems.push(`component token "${t.path}" may not alias another component token`);
    }
  }

  // 3. Every colour is an opaque or alpha hex; modes only on colours and shadows.
  for (const t of tokens.values()) {
    if (t.modes && t.type !== 'color' && t.type !== 'shadow') {
      problems.push(`"${t.path}": per-mode values are only supported on colour and shadow tokens`);
    }
    for (const m of Object.keys(t.modes ?? {})) {
      if (!MODES.includes(m) || m === 'light') problems.push(`"${t.path}": unknown mode "${m}"`);
    }
  }

  // 4. Contrast.
  for (const mode of MODES) {
    const hc = mode.endsWith('hc');
    for (const [fg, bg, min, minHc] of CONTRAST_PAIRS) {
      const need = hc ? minHc : min;
      let ratio;
      try {
        ratio = contrast(resolve(tokens, fg, mode), resolve(tokens, bg, mode));
      } catch (e) {
        problems.push(`contrast pair ${fg} / ${bg}: ${e.message}`);
        continue;
      }
      if (ratio < need) {
        problems.push(`[${mode}] contrast ${fg} on ${bg} is ${ratio.toFixed(2)}:1, needs ${need}:1`);
      }
    }
  }

  // 5. Dark and light must differ where the semantics demand it (guards against a copy-paste dark mode).
  for (const path of ['color.surface.window', 'color.text.primary']) {
    const t = tokens.get(path);
    if (t && rawValue(t, 'light') === rawValue(t, 'dark')) problems.push(`"${path}" has the same light and dark value`);
  }

  return problems;
}

const PRIMITIVE_ROOTS = new Set(['palette', 'space', 'radius', 'border-width', 'font', 'duration', 'easing', 'breakpoint']);
