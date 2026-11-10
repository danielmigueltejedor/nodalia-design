// Appearance controls and helpers shared by the catalog and the reference demos.
import { setAppearance, tooltips } from './behaviors.mjs';

const KEY = 'ndl-appearance';
const AXES = ['theme', 'contrast', 'density', 'motion'];

const load = () => { try { return JSON.parse(localStorage.getItem(KEY)) ?? {}; } catch { return {}; } };
const save = (v) => { try { localStorage.setItem(KEY, JSON.stringify(v)); } catch { /* storage unavailable: preferences simply do not persist */ } };

function apply(prefs) {
  setAppearance(Object.fromEntries(AXES.map((a) => [a, prefs[a] || null])));
  document.documentElement.style.fontSize = prefs.scale ? `${prefs.scale}%` : '';
}

/** Restores saved preferences and wires every <select data-appearance="axis"> on the page. */
export function initAppearance() {
  const prefs = load();
  apply(prefs);
  for (const el of document.querySelectorAll('[data-appearance]')) {
    el.value = prefs[el.dataset.appearance] ?? '';
    el.addEventListener('change', () => {
      prefs[el.dataset.appearance] = el.value;
      apply(prefs);
      save(prefs);
    });
  }
  tooltips(document);
}

export const cssVar = (name) => getComputedStyle(document.documentElement).getPropertyValue(name).trim();

/** Calls cb when the effective appearance may have changed (attributes or system preferences). */
export function onAppearanceChange(cb) {
  new MutationObserver(cb).observe(document.documentElement, { attributes: true });
  for (const q of ['(prefers-color-scheme: dark)', '(prefers-contrast: more)']) matchMedia(q).addEventListener('change', cb);
}

/** Markup for the appearance bar, identical on every page. */
export const APPEARANCE_BAR = `
  <div class="ndl-appearance" role="group" aria-label="Appearance">
    <label>Theme <select class="ndl-select" data-appearance="theme"><option value="">System</option><option value="light">Light</option><option value="dark">Dark</option></select></label>
    <label>Contrast <select class="ndl-select" data-appearance="contrast"><option value="">System</option><option value="more">High</option></select></label>
    <label>Density <select class="ndl-select" data-appearance="density"><option value="">Regular</option><option value="compact">Compact</option><option value="comfortable">Comfortable</option></select></label>
    <label>Scale <select class="ndl-select" data-appearance="scale"><option value="">100%</option><option value="125">125%</option><option value="150">150%</option></select></label>
    <label>Motion <select class="ndl-select" data-appearance="motion"><option value="">System</option><option value="reduced">Reduced</option></select></label>
  </div>`;
