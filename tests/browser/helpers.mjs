import { chromium } from 'playwright-core';
import { serve } from '../../apps/catalog/scripts/serve.mjs';

/** Starts the catalog server and a headless Chromium. Call `close()` when done. */
export async function launch() {
  const server = await serve(0);
  const browser = await chromium.launch({ executablePath: process.env.NDL_CHROMIUM || undefined });
  return {
    browser,
    base: `http://127.0.0.1:${server.address().port}`,
    async page(opts = {}) {
      const ctx = await browser.newContext({ viewport: { width: 1280, height: 900 }, ...opts });
      const page = await ctx.newPage();
      const errors = [];
      page.on('pageerror', (e) => errors.push(e.message));
      page.on('console', (m) => { if (m.type() === 'error') errors.push(m.text()); });
      page.errors = errors;
      return page;
    },
    async close() { await browser.close(); server.close(); },
  };
}
