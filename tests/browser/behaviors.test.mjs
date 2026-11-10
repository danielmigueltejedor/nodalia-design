import { test, before, after } from 'node:test';
import assert from 'node:assert/strict';
import { launch } from './helpers.mjs';

let h;
before(async () => { h = await launch(); });
after(async () => { await h.close(); });

async function catalog() {
  const page = await h.page({ colorScheme: 'light' });
  await page.goto(`${h.base}/index.html`);
  return page;
}
const attr = (page, sel, name) => page.$eval(sel, (e, n) => e.getAttribute(n), name);

test('tabs: arrows move selection, panels follow, Home/End jump, only one tab is in the Tab order', async () => {
  const page = await catalog();
  await page.focus('#t1');
  await page.keyboard.press('ArrowRight');
  assert.equal(await attr(page, '#t2', 'aria-selected'), 'true');
  assert.equal(await page.$eval('#p2', (e) => e.hidden), false);
  assert.equal(await page.$eval('#p1', (e) => e.hidden), true);
  await page.keyboard.press('End');
  assert.equal(await attr(page, '#t3', 'aria-selected'), 'true');
  await page.keyboard.press('ArrowRight');
  assert.equal(await attr(page, '#t1', 'aria-selected'), 'true', 'wraps around');
  const tabbable = await page.$$eval('#tabs [role="tab"]', (els) => els.filter((e) => e.tabIndex === 0).length);
  assert.equal(tabbable, 1);
});

test('toolbar: roving tabindex, arrows move focus without activating', async () => {
  const page = await catalog();
  const buttons = '#tb button';
  assert.equal(await page.$$eval(buttons, (els) => els.filter((e) => e.tabIndex === 0).length), 1);
  await page.focus('#tb button');
  await page.keyboard.press('ArrowRight');
  assert.equal(await page.evaluate(() => document.activeElement.textContent), 'I');
  assert.equal(await attr(page, '#tb button:nth-child(2)', 'aria-pressed'), 'false', 'arrow does not toggle');
  await page.keyboard.press('Home');
  assert.equal(await page.evaluate(() => document.activeElement.textContent), 'B');
});

test('tree: arrows, expand/collapse, Home/End, selection and aria-level', async () => {
  const page = await catalog();
  const levels = await page.$$eval('#tree [role="treeitem"]', (els) => els.map((e) => e.getAttribute('aria-level')));
  assert.deepEqual(levels, ['1', '2', '2', '3', '2']);
  await page.focus('#tree [role="treeitem"][aria-selected="true"]');
  await page.keyboard.press('ArrowDown');
  assert.equal(await page.evaluate(() => document.activeElement.textContent.trim().startsWith('Sketch 1')), true);
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('ArrowRight');
  await page.keyboard.press('ArrowRight');
  assert.equal(await page.evaluate(() => document.activeElement.textContent.trim().startsWith('Fillet 1')), true, 'Right on an open node enters its first child');
  await page.keyboard.press('ArrowLeft');
  await page.keyboard.press('ArrowLeft');
  assert.equal(await page.evaluate(() => document.activeElement.getAttribute('aria-expanded')), 'false', 'Left on an open node collapses it');
  assert.equal(await page.$eval('#tree [role="treeitem"] [role="treeitem"] [role="treeitem"]', (e) => e.offsetParent === null), true, 'collapsed children are not rendered');
  await page.keyboard.press('End');
  assert.equal(await page.evaluate(() => document.activeElement.textContent.trim().startsWith('Hole 1')), true, 'End skips hidden items');
  await page.keyboard.press('Enter');
  assert.equal(await page.evaluate(() => document.activeElement.getAttribute('aria-selected')), 'true');
  assert.equal(await page.$$eval('#tree [aria-selected="true"]', (e) => e.length), 1);
});

test('splitter: exposes its value, resizes with arrows and is clamped', async () => {
  const page = await catalog();
  const now = () => attr(page, '#spl', 'aria-valuenow').then(Number);
  const start = await now();
  assert.ok(start > 0 && start < 100);
  await page.focus('#spl');
  const w0 = await page.$eval('#spl', (e) => e.previousElementSibling.getBoundingClientRect().width);
  await page.keyboard.press('ArrowRight');
  const w1 = await page.$eval('#spl', (e) => e.previousElementSibling.getBoundingClientRect().width);
  assert.ok(Math.abs(w1 - w0 - 16) <= 1, `moved ${w1 - w0}px`);
  await page.keyboard.press('Home');
  const min = await page.$eval('#spl', (e) => e.previousElementSibling.getBoundingClientRect().width);
  assert.ok(min >= 120 && min < w0);
  await page.keyboard.press('End');
  const max = await page.$eval('#spl', (e) => e.nextElementSibling.getBoundingClientRect().width);
  assert.ok(max >= 119, `second pane keeps its minimum, got ${max}`);
});

test('splitter: pointer drag resizes', async () => {
  const page = await catalog();
  await page.locator('#spl').scrollIntoViewIfNeeded();
  const box = await (await page.$('#spl')).boundingBox();
  const w0 = await page.$eval('#spl', (e) => e.previousElementSibling.getBoundingClientRect().width);
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.mouse.down(); await page.mouse.move(box.x + 60, box.y + box.height / 2, { steps: 5 }); await page.mouse.up();
  const w1 = await page.$eval('#spl', (e) => e.previousElementSibling.getBoundingClientRect().width);
  assert.ok(w1 > w0 + 40, `${w0} -> ${w1}`);
});

test('context menu: opens from the keyboard, arrows wrap, Escape closes and restores focus', async () => {
  const page = await catalog();
  await page.focus('#ctx-btn');
  await page.keyboard.press('Shift+F10');
  assert.equal(await page.$eval('#ctx', (e) => e.hidden), false);
  assert.equal(await page.evaluate(() => document.activeElement.textContent.startsWith('Rename')), true);
  await page.keyboard.press('ArrowUp');
  assert.equal(await page.evaluate(() => document.activeElement.textContent.startsWith('Delete')), true, 'wraps to last');
  await page.keyboard.press('Tab');
  assert.equal(await page.$eval('#ctx', (e) => e.hidden), false, 'Tab does not leave an open menu');
  await page.keyboard.press('Escape');
  assert.equal(await page.$eval('#ctx', (e) => e.hidden), true);
  assert.equal(await page.evaluate(() => document.activeElement.id), 'ctx-btn');
});

test('context menu: right click opens at the pointer, outside click closes', async () => {
  const page = await catalog();
  await page.click('#ctx-btn', { button: 'right' });
  assert.equal(await page.$eval('#ctx', (e) => e.hidden), false);
  await page.mouse.click(5, 5);
  assert.equal(await page.$eval('#ctx', (e) => e.hidden), true);
});

test('dialog: modal, labelled, focuses the safe action, Escape closes', async () => {
  const page = await catalog();
  await page.click('#dlg-btn');
  assert.equal(await page.$eval('#dlg', (e) => e.open), true);
  assert.equal(await page.evaluate(() => document.activeElement.textContent), 'Cancel');
  assert.equal(await attr(page, '#dlg', 'aria-labelledby'), 'dlg-t');
  await page.keyboard.press('Escape');
  assert.equal(await page.$eval('#dlg', (e) => e.open), false);
  assert.equal(await page.evaluate(() => document.activeElement.id), 'dlg-btn', 'focus returns to the opener');
});

test('tooltip: appears on keyboard focus, is described by aria-describedby, Escape dismisses', async () => {
  const page = await catalog();
  await page.keyboard.press('Tab');
  await page.focus('[data-ndl-tooltip="Add item"]');
  await page.keyboard.press('Shift+Tab'); await page.keyboard.press('Tab');
  const tip = page.locator('.ndl-tooltip');
  await tip.waitFor({ state: 'visible', timeout: 2000 });
  assert.equal(await tip.textContent(), 'Add item');
  const id = await tip.getAttribute('id');
  assert.equal(await attr(page, '[data-ndl-tooltip="Add item"]', 'aria-describedby'), id);
  await page.keyboard.press('Escape');
  assert.equal(await tip.isHidden(), true);
});

test('slider fill follows the value', async () => {
  const page = await catalog();
  await page.fill('#sl', '25');
  assert.equal(await page.$eval('#sl', (e) => e.style.getPropertyValue('--ndl-fill')), '25%');
});

test('toast: appears in a live region and can be dismissed', async () => {
  const page = await catalog();
  assert.equal(await attr(page, '#toasts', 'aria-live'), 'polite');
  await page.click('#toast-btn');
  assert.equal(await page.locator('#toasts .ndl-toast').count(), 1);
  await page.click('#toasts .ndl-toast button');
  assert.equal(await page.locator('#toasts .ndl-toast').count(), 0);
});

test('every interactive control in the catalog is reachable and shows a visible focus indicator', async () => {
  const page = await catalog();
  const bad = await page.evaluate(() => {
    const out = [];
    for (const el of document.querySelectorAll('.ndl-button, .ndl-input, .ndl-select, .ndl-switch, .ndl-check, .ndl-slider')) {
      if (el.closest('.states') || el.disabled || !el.checkVisibility()) continue;
      el.focus({ focusVisible: true });
      if (document.activeElement !== el) { out.push(`${el.className} cannot take focus`); continue; }
      const c = getComputedStyle(el);
      const hasRing = (c.outlineStyle !== 'none' && parseFloat(c.outlineWidth) >= 2) || c.boxShadow !== 'none';
      if (!hasRing) out.push(`${el.className} (${el.textContent.trim().slice(0, 12)}) has no focus indicator`);
    }
    return out;
  });
  assert.deepEqual(bad, []);
});
