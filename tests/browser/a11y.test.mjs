import { test, before, after } from 'node:test';
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { launch } from './helpers.mjs';

const axePath = createRequire(import.meta.url).resolve('axe-core/axe.min.js');
const PAGES = ['index.html', 'examples/productivity.html', 'examples/creative.html', 'examples/engineering.html'];
const APPEARANCES = [
  ['light', { colorScheme: 'light', contrast: 'no-preference' }],
  ['dark', { colorScheme: 'dark', contrast: 'no-preference' }],
  ['light high contrast', { colorScheme: 'light', contrast: 'more' }],
  ['dark high contrast', { colorScheme: 'dark', contrast: 'more' }],
];
let h;
before(async () => { h = await launch(); });
after(async () => { await h.close(); });

for (const p of PAGES) {
  for (const [name, opts] of APPEARANCES) {
    test(`axe: ${p} in ${name} has no violations`, async () => {
      const page = await h.page(opts);
      await page.goto(`${h.base}/${p}`);
      await page.waitForLoadState('networkidle');
      await page.addScriptTag({ path: axePath });
      const { violations } = await page.evaluate(() => axe.run(document, { runOnly: { type: 'tag', values: ['wcag2a', 'wcag2aa', 'wcag21aa', 'wcag22aa', 'best-practice'] } }));
      const report = violations.map((v) => `${v.id} (${v.impact}): ${v.nodes.slice(0, 3).map((n) => n.target.join(' ') + ' :: ' + (n.any[0]?.message ?? '')).join(' | ')}`);
      assert.deepEqual(report, []);
      await page.context().close();
    });
  }
}
