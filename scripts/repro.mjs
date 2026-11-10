// Builds twice from a clean state and fails if any generated file differs: builds must be reproducible.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, readdirSync, rmSync, statSync } from 'node:fs';
import { join } from 'node:path';

const root = new URL('../', import.meta.url).pathname;
const outputs = ['packages/tokens/dist', 'packages/gtk/dist', 'packages/qml/dist', 'packages/web/dist', 'apps/catalog/dist'];

const files = (dir) => readdirSync(dir).flatMap((f) => (statSync(join(dir, f)).isDirectory() ? files(join(dir, f)) : [join(dir, f)]));
function build() {
  for (const o of outputs) rmSync(join(root, o), { recursive: true, force: true });
  execFileSync('npm', ['run', 'build'], { cwd: root, stdio: 'ignore' });
  return Object.fromEntries(outputs.flatMap((o) => files(join(root, o))).sort().map((f) => [f.slice(root.length), createHash('sha256').update(readFileSync(f)).digest('hex')]));
}

const a = build();
const b = build();
const differing = Object.keys({ ...a, ...b }).filter((k) => a[k] !== b[k]);
if (differing.length > 0) {
  console.error(`Build is not reproducible. Differing files:\n${differing.map((f) => `  - ${f}`).join('\n')}`);
  process.exit(1);
}
console.log(`Reproducible: ${Object.keys(a).length} generated files identical across two clean builds.`);
