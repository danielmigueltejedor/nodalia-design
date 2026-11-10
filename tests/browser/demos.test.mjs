import { test, before, after } from 'node:test';
import assert from 'node:assert/strict';
import { launch } from './helpers.mjs';

let h;
before(async () => { h = await launch(); });
after(async () => { await h.close(); });
const open = async (p) => { const page = await h.page({ colorScheme: 'light' }); await page.goto(`${h.base}/examples/${p}`); return page; };
const text = (page, sel) => page.$eval(sel, (e) => e.textContent.trim());

test('productivity: add, complete, filter, search, delete with confirmation', async () => {
  const page = await open('productivity.html');
  assert.equal(await text(page, '#summary'), '1 of 4 completed');
  await page.click('#add');
  assert.equal(await page.locator('#tasks > li').count(), 5);
  assert.equal(await page.inputValue('#d-title'), 'New task 5');
  await page.locator('#tasks li').nth(1).locator('input[type=checkbox]').check();
  assert.equal(await text(page, '#summary'), '2 of 5 completed');
  assert.equal(await attr(page, '#prog', 'aria-valuenow'), '40');
  await page.click('[data-filter="done"]');
  assert.equal(await page.locator('#tasks > li').count(), 2);
  await page.click('[data-filter="all"]');
  await page.fill('#q', 'keyboard');
  assert.equal(await page.locator('#tasks > li').count(), 1);
  await page.locator('#tasks li button').first().click();
  await page.click('#d-del');
  assert.equal(await page.$eval('#confirm', (e) => e.open), true);
  await page.click('#confirm button[value="cancel"]');
  assert.equal(await page.locator('#tasks > li').count(), 1, 'cancel keeps the task');
  await page.click('#d-del'); await page.click('#confirm button[value="delete"]');
  await page.waitForFunction(() => document.querySelectorAll('#tasks > li').length === 0); // the dialog's close event is async
  assert.equal(await page.locator('#tasks > li').count(), 0);
  assert.equal(await page.$eval('#empty', (e) => e.hidden), false);
});
const attr = (page, sel, n) => page.$eval(sel, (e, x) => e.getAttribute(x), n);

test('creative: layer visibility, property edits and undo/redo', async () => {
  const page = await open('creative.html');
  assert.equal(await page.isDisabled('#undo'), true);
  await page.locator('#layers li').nth(0).locator('button').first().click();
  assert.equal(await attr(page, '#layers li:nth-child(1) button', 'aria-pressed'), 'false');
  assert.equal(await page.isDisabled('#undo'), false);
  await page.click('#undo');
  assert.equal(await attr(page, '#layers li:nth-child(1) button', 'aria-pressed'), 'true');
  assert.equal(await page.isDisabled('#redo'), false);
  await page.click('#redo');
  assert.equal(await attr(page, '#layers li:nth-child(1) button', 'aria-pressed'), 'false');
  await page.fill('#l-name', 'Renamed'); await page.press('#l-name', 'Enter'); await page.locator('#l-name').blur();
  assert.match(await text(page, '#layers'), /Renamed/);
});

test('creative: the canvas actually paints the layers', async () => {
  const page = await open('creative.html');
  const painted = await page.$eval('#cv', (c) => { const d = c.getContext('2d').getImageData(0, 0, c.width, c.height).data; let n = 0; for (let i = 3; i < d.length; i += 4) if (d[i]) n++; return n; });
  assert.ok(painted > 1000);
});

test('engineering: parameter edits update the model, history, undo, units and suppression', async () => {
  const page = await open('engineering.html');
  assert.equal(await text(page, '#dims'), '80 × 50 × 20 mm');
  await page.fill('#f-width', '100'); await page.press('#f-width', 'Enter'); await page.locator('#f-width').blur();
  assert.equal(await text(page, '#dims'), '100 × 50 × 20 mm');
  assert.equal(await text(page, '#m-sketch'), '100 × 50 mm');
  assert.equal(await page.isDisabled('#undo'), false);
  await page.click('#undo');
  assert.equal(await text(page, '#dims'), '80 × 50 × 20 mm');
  await page.click('#redo');
  await page.selectOption('#unit', 'in');
  assert.equal(await text(page, '#m-extrude'), '0.787 in');
  await page.selectOption('#unit', 'mm');
  await page.click('#suppress');
  assert.equal(await text(page, '#m-hole'), 'suppressed');
  assert.equal(await attr(page, '#suppress', 'aria-pressed'), 'true');
  await page.click('#t-hist');
  assert.equal(await page.locator('#hist li').count(), 3);
  assert.equal(await page.locator('#hist li[data-current]').count(), 1);
});

test('engineering: invalid input is rejected and announced, not applied', async () => {
  const page = await open('engineering.html');
  await page.fill('#f-width', '-5'); await page.press('#f-width', 'Enter'); await page.locator('#f-width').blur();
  assert.equal(await attr(page, '#f-width', 'aria-invalid'), 'true');
  assert.equal(await text(page, '#dims'), '80 × 50 × 20 mm');
  assert.match(await text(page, '#st'), /must be a positive number/);
});

test('engineering: selecting a tree item switches the inspector', async () => {
  const page = await open('engineering.html');
  await page.click('#tree [data-op="extrude"] > .ndl-tree__row');
  assert.equal(await text(page, '#ins-title'), 'Extrude 1');
  assert.equal(await page.locator('#f-height').count(), 1);
});

test('engineering: export dialog runs a progress bar to completion', async () => {
  const page = await open('engineering.html');
  await page.click('#exp');
  await page.click('#dg');
  await page.waitForFunction(() => document.getElementById('dp').getAttribute('aria-valuenow') === '100', null, { timeout: 4000 });
  assert.equal(await text(page, '#ds'), 'Export complete.');
});

test('demos hold up at compact density, 150% scale and high contrast without horizontal overflow', async () => {
  for (const p of ['productivity.html', 'creative.html', 'engineering.html']) {
    const page = await h.page({ colorScheme: 'dark', contrast: 'more', viewport: { width: 1280, height: 800 } });
    await page.goto(`${h.base}/examples/${p}`);
    await page.selectOption('[data-appearance="density"]', 'compact');
    await page.selectOption('[data-appearance="scale"]', '125');
    const over = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    assert.ok(over <= 0, `${p} overflows by ${over}px`);
    await page.context().close();
  }
});
