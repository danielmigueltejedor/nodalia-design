// Builds dist/nodalia.css (tokens + components in cascade layers) and dist/behaviors.mjs.
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const dist = new URL('../dist/', import.meta.url).pathname;
const src = new URL('../src/', import.meta.url).pathname;
const tokens = readFileSync(new URL('../../tokens/dist/tokens.css', import.meta.url), 'utf8');
const components = readFileSync(join(src, 'components.css'), 'utf8');

mkdirSync(dist, { recursive: true });
// Layers let an application override NDL with ordinary, un-layered rules.
writeFileSync(join(dist, 'nodalia.css'), `@layer ndl.tokens, ndl.components;\n\n@layer ndl.tokens {\n${tokens}}\n\n@layer ndl.components {\n${components}}\n`);
copyFileSync(join(src, 'behaviors.mjs'), join(dist, 'behaviors.mjs'));
console.log('@nodalia/web: nodalia.css, behaviors.mjs -> dist/');
