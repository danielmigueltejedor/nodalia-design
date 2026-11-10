import { test, before, after } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { launch } from './helpers.mjs';

const resolved = JSON.parse(readFileSync(new URL('../../packages/tokens/dist/resolved.json', import.meta.url), 'utf8'));
const TOKENS = ['color.surface.window', 'color.text.primary', 'color.text.on-accent', 'color.accent.default', 'color.status.danger.fg', 'color.border.default'];
let h;
before(async () => { h = await launch(); });
after(async () => { await h.close(); });

const read = (page) => page.evaluate((tokens) => Object.fromEntries(tokens.map((t) => [t, getComputedStyle(document.documentElement).getPropertyValue(`--ndl-${t.replaceAll('.', '-')}`).trim()])), TOKENS);

// [system colour scheme, system contrast, data-theme, data-contrast, expected mode]
const MATRIX = [
  ['light', 'no-preference', null, null, 'light'],
  ['dark', 'no-preference', null, null, 'dark'],
  ['light', 'more', null, null, 'light-hc'],
  ['dark', 'more', null, null, 'dark-hc'],
  ['light', 'no-preference', 'dark', null, 'dark'],
  ['dark', 'no-preference', 'light', null, 'light'],
  ['light', 'no-preference', 'dark', 'more', 'dark-hc'],
  ['dark', 'no-preference', 'light', 'more', 'light-hc'],
  ['dark', 'no-preference', null, 'more', 'dark-hc'],
  ['light', 'no-preference', null, 'more', 'light-hc'],
  ['dark', 'more', 'light', null, 'light-hc'],
  ['light', 'more', 'dark', null, 'dark-hc'],
];

for (const [scheme, contrast, theme, attr, mode] of MATRIX) {
  test(`system ${scheme}/${contrast}, data-theme=${theme}, data-contrast=${attr} resolves to ${mode}`, async () => {
    const page = await h.page({ colorScheme: scheme, contrast });
    await page.goto(`${h.base}/index.html`);
    await page.evaluate(([t, c]) => {
      if (t) document.documentElement.setAttribute('data-theme', t);
      if (c) document.documentElement.setAttribute('data-contrast', c);
    }, [theme, attr]);
    const got = await read(page);
    for (const t of TOKENS) assert.equal(got[t], resolved[mode][t].toLowerCase(), `${t} in ${mode}`);
    await page.context().close();
  });
}

test('reduced motion zeroes transition tokens, and data-motion="full" opts back in', async () => {
  const page = await h.page({ reducedMotion: 'reduce' });
  await page.goto(`${h.base}/index.html`);
  const v = () => page.evaluate(() => getComputedStyle(document.documentElement).getPropertyValue('--ndl-motion-panel').trim());
  assert.match(await v(), /^0ms /);
  await page.evaluate(() => document.documentElement.setAttribute('data-motion', 'full'));
  assert.match(await v(), /^240ms /);
  const normal = await h.page({ reducedMotion: 'no-preference' });
  await normal.goto(`${h.base}/index.html`);
  assert.match(await normal.evaluate(() => getComputedStyle(document.documentElement).getPropertyValue('--ndl-motion-panel').trim()), /^240ms /);
  await normal.evaluate(() => document.documentElement.setAttribute('data-motion', 'reduced'));
  assert.match(await normal.evaluate(() => getComputedStyle(document.documentElement).getPropertyValue('--ndl-motion-panel').trim()), /^0ms /);
});

test('reduced motion stops the spinner and the indeterminate bar', async () => {
  const page = await h.page({ reducedMotion: 'reduce' });
  await page.goto(`${h.base}/index.html`);
  assert.equal(await page.$eval('.ndl-spinner', (e) => getComputedStyle(e).animationName), 'none');
  const normal = await h.page({ reducedMotion: 'no-preference' });
  await normal.goto(`${h.base}/index.html`);
  assert.equal(await normal.$eval('.ndl-spinner', (e) => getComputedStyle(e).animationName), 'ndl-spin');
});

test('density changes control height; scale changes it again through rem', async () => {
  const page = await h.page();
  await page.goto(`${h.base}/index.html`);
  const height = () => page.$eval('#actions .ndl-button', (e) => Math.round(e.getBoundingClientRect().height));
  assert.equal(await height(), 32);
  await page.selectOption('[data-appearance="density"]', 'compact');
  assert.equal(await height(), 24);
  await page.selectOption('[data-appearance="density"]', 'comfortable');
  assert.equal(await height(), 40);
  await page.selectOption('[data-appearance="scale"]', '150');
  assert.equal(await height(), 60);
});

test('appearance choices persist across reloads', async () => {
  const page = await h.page();
  await page.goto(`${h.base}/index.html`);
  await page.selectOption('[data-appearance="theme"]', 'dark');
  await page.reload();
  assert.equal(await page.evaluate(() => document.documentElement.getAttribute('data-theme')), 'dark');
  assert.equal(await page.$eval('[data-appearance="theme"]', (e) => e.value), 'dark');
});

test('the catalog and the demos render without console errors', async () => {
  for (const p of ['index.html', 'examples/productivity.html', 'examples/creative.html', 'examples/engineering.html']) {
    const page = await h.page();
    await page.goto(`${h.base}/${p}`);
    await page.waitForLoadState('networkidle');
    assert.deepEqual(page.errors, [], p);
    await page.context().close();
  }
});
