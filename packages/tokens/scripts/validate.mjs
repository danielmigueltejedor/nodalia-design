import { SOURCE_DIR, loadTokens, validate } from '../lib/index.mjs';

const tokens = loadTokens(SOURCE_DIR);
const problems = validate(tokens);
if (problems.length > 0) {
  console.error(problems.map((p) => `  - ${p}`).join('\n'));
  process.exit(1);
}
console.log(`tokens valid: ${tokens.size} tokens`);
