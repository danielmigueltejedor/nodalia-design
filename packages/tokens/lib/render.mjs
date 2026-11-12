// Turns token values into strings for each output format.
import { isRef, refPath, MEMBER_TYPES } from './load.mjs';
import { parseHex } from './color.mjs';

const num = (n) => String(Number.parseFloat(n.toFixed(4)));

/** "13px" -> 13 */
export function px(value) {
  const m = /^(-?(?:\d+(?:\.\d+)?|\.\d+))px$/.exec(value);
  if (!m) throw new Error(`expected a px dimension, got "${value}"`);
  return Number(m[1]);
}

/** "160ms" -> 160 */
export function ms(value) {
  const m = /^((?:\d+(?:\.\d+)?|\.\d+))ms$/.exec(value);
  if (!m) throw new Error(`expected an ms duration, got "${value}"`);
  return Number(m[1]);
}

/**
 * Renders a raw (possibly alias-bearing) value as CSS.
 * @param {unknown} value
 * @param {string} type
 * @param {{ref: (path: string) => string, rem?: boolean, zeroDuration?: boolean}} ctx
 */
export function css(value, type, ctx) {
  // Reduced motion wins over aliases: a duration is 0ms whatever it points at.
  if (type === 'duration' && ctx.zeroDuration) return '0ms';
  if (isRef(value)) return ctx.ref(refPath(value));
  switch (type) {
    case 'color': return value.toLowerCase();
    case 'dimension': return ctx.rem ? rem(value) : value === '0px' ? '0' : value;
    case 'duration': return value;
    case 'cubicBezier': return `cubic-bezier(${value.map(num).join(', ')})`;
    case 'fontFamily': return value.map((f) => (/[\s]/.test(f) ? `"${f}"` : f)).join(', ');
    case 'fontWeight':
    case 'number': return String(value);
    case 'shadow': {
      const m = (k) => css(value[k], MEMBER_TYPES.shadow[k], ctx);
      return `${m('offsetX')} ${m('offsetY')} ${m('blur')} ${m('spread')} ${m('color')}`;
    }
    case 'typography': {
      const m = (k) => css(value[k], MEMBER_TYPES.typography[k], ctx);
      return `${m('fontWeight')} ${m('fontSize')}/${m('lineHeight')} ${m('fontFamily')}`;
    }
    case 'transition': {
      const m = (k) => css(value[k], MEMBER_TYPES.transition[k], ctx);
      return `${m('duration')} ${m('timingFunction')} ${m('delay')}`;
    }
    default: throw new Error(`cannot render type "${type}" as CSS`);
  }
}

function rem(value) {
  const n = px(value);
  return n === 0 ? '0' : `${num(n / 16)}rem`;
}

/** Colour as `#aarrggbb`, the order QML expects. */
export function qmlColor(hex) {
  const { r, g, b, a } = parseHex(hex);
  const h = (n) => n.toString(16).padStart(2, '0');
  return a === 1 ? `#${h(r)}${h(g)}${h(b)}`.toUpperCase() : `#${h(Math.round(a * 255))}${h(r)}${h(g)}${h(b)}`.toUpperCase();
}

/** Colour as a GTK/CSS rgba() or #rrggbb. */
export function gtkColor(hex) {
  const { r, g, b, a } = parseHex(hex);
  if (a === 1) return hex.slice(0, 7).toLowerCase();
  return `rgba(${r}, ${g}, ${b}, ${num(a)})`;
}
