// The bold style: NDL's icons as Nodalia apps show them. A heavier stroke, and the elements an
// icon lists in `fill` drawn solid, stroked in the same ink so their corners stay round. An icon
// may give a `bold` body of its own when filling its regular shapes would not read (a disc needs
// its hole). The regular style (1.75 stroke, outlines) stays for documents and dense tools.
import { ICONS } from './icons.mjs';

export const BOLD_STROKE = 2.25;

/** The bold body of `name`: its SVG elements, with fills applied. */
export function boldBody(name) {
  const icon = ICONS[name];
  if (icon.bold) return icon.bold;
  const fill = new Set(icon.fill ?? []);
  return icon.body.map((part, i) => (fill.has(i) && !/fill=/.test(part)
    ? part.replace(/^<(\w+)/, '<$1 fill="currentColor"')
    : part));
}
