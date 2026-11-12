// Colour parsing and WCAG 2.x contrast.

/** @returns {{r:number,g:number,b:number,a:number}} channels 0-255, alpha 0-1 */
export function parseHex(hex) {
  const m = /^#([0-9a-f]{6})([0-9a-f]{2})?$/i.exec(hex);
  if (!m) throw new Error(`unsupported colour "${hex}" (expected #RRGGBB or #RRGGBBAA)`);
  const n = Number.parseInt(m[1], 16);
  return {
    r: (n >> 16) & 255,
    g: (n >> 8) & 255,
    b: n & 255,
    a: m[2] ? Number.parseInt(m[2], 16) / 255 : 1,
  };
}

const lin = (c) => {
  const s = c / 255;
  return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
};

export function luminance({ r, g, b }) {
  return 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
}

/** WCAG contrast ratio between two opaque hex colours. */
export function contrast(fgHex, bgHex) {
  const a = luminance(parseHex(fgHex));
  const b = luminance(parseHex(bgHex));
  const [hi, lo] = a > b ? [a, b] : [b, a];
  return (hi + 0.05) / (lo + 0.05);
}
