// Enforces "no arbitrary visual values" in component CSS and that every token it reads exists.
import { readFileSync } from 'node:fs';
import { SOURCE_DIR, loadTokens, cssName } from '@nodalia/tokens/lib';

/** Custom properties set at runtime by behaviours or by the application, not by tokens. */
const RUNTIME_VARS = new Set(['--ndl-level', '--ndl-fill', '--ndl-value', '--ndl-split-start']);

export function lintCss(source, tokenNames) {
  const problems = [];
  const text = source.replace(/\/\*[\s\S]*?\*\//g, (m) => m.replace(/[^\n]/g, ' '));
  const lines = text.split('\n');
  lines.forEach((line, i) => {
    const at = (msg) => problems.push(`line ${i + 1}: ${msg}: ${line.trim()}`);
    if (/#[0-9a-f]{3,8}\b/i.test(line)) at('colour literal, use a colour token');
    if (/\b(rgb|rgba|hsl|hsla|oklch|oklab|color-mix)\(/i.test(line)) at('colour function, use a colour token');
    const unit = /(?<![\w.-])-?\d*\.?\d+(px|rem|em|ms|s|pt|cm|mm)\b/.exec(line);
    if (unit) at(`fixed ${unit[1]} value, use a size, space or duration token`);
    for (const [, name] of line.matchAll(/var\((--ndl-[\w-]+)/g)) {
      if (!tokenNames.has(name) && !RUNTIME_VARS.has(name)) at(`unknown token ${name}`);
    }
  });
  return problems;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const tokens = loadTokens(SOURCE_DIR);
  const names = new Set([...tokens.keys()].map(cssName));
  for (const t of tokens.keys()) {
    const m = /^density\.\w+\.(.+)$/.exec(t);
    if (m) names.add(`--ndl-density-${m[1]}`);
  }
  const file = new URL('../src/components.css', import.meta.url);
  const problems = lintCss(readFileSync(file, 'utf8'), names);
  if (problems.length) {
    console.error(`components.css: ${problems.length} problem(s)\n` + problems.map((p) => `  - ${p}`).join('\n'));
    process.exit(1);
  }
  console.log('components.css: no arbitrary values, all tokens exist');
}
