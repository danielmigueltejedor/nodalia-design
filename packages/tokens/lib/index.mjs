// Public API of @nodalia/tokens/lib: what adapter packages build on.
export { MODES, MODE_CHAIN, loadTokens, resolve, rawValue, hasModes, refsIn } from './load.mjs';
export { contrast, parseHex, luminance } from './color.mjs';
export { px, ms, gtkColor, qmlColor } from './render.mjs';
export { buildCss, cssName } from './css.mjs';
export { validate } from './validate.mjs';
export { DEFAULT_ACCENT, loadAccents, accentRamp, withAccent, accentSets, accentCss, accentOverrides, toOklch } from './accent.mjs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

/** Absolute path of the authored DTCG sources. */
export const SOURCE_DIR = join(dirname(fileURLToPath(import.meta.url)), '..', 'src');
