// Builds dist/: tokens.css, tokens.json (authored DTCG tree) and resolved.json (values per mode).
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { MODES, SOURCE_DIR, loadTokens, resolve, buildCss, validate } from '../lib/index.mjs';

const dist = new URL('../dist/', import.meta.url).pathname;
const tokens = loadTokens(SOURCE_DIR);

const problems = validate(tokens);
if (problems.length > 0) {
  console.error(`Token validation failed (${problems.length}):\n` + problems.map((p) => `  - ${p}`).join('\n'));
  process.exit(1);
}

mkdirSync(dist, { recursive: true });
writeFileSync(join(dist, 'tokens.css'), buildCss(tokens));

const merged = {};
for (const f of readdirSync(SOURCE_DIR).filter((n) => n.endsWith('.tokens.json')).sort()) {
  Object.assign(merged, JSON.parse(readFileSync(join(SOURCE_DIR, f), 'utf8')));
}
writeFileSync(join(dist, 'tokens.json'), JSON.stringify(merged, null, 2) + '\n');

const resolved = Object.fromEntries(
  MODES.map((m) => [m, Object.fromEntries([...tokens.keys()].map((p) => [p, resolve(tokens, p, m)]))]),
);
writeFileSync(join(dist, 'resolved.json'), JSON.stringify(resolved, null, 2) + '\n');
console.log(`@nodalia/tokens: ${tokens.size} tokens, ${MODES.length} modes -> dist/`);
