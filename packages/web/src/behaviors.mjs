// Keyboard and ARIA behaviours for the components that native HTML cannot provide.
// Everything is opt-in and framework-free. Each function takes an element and returns a cleanup function.

const FOCUSABLE_DISABLED = '[aria-disabled="true"], [disabled], [hidden]';

/**
 * Roving tabindex: one item in the Tab order, arrow keys move between items.
 * @param {HTMLElement} container
 * @param {{ items?: string, orientation?: 'horizontal'|'vertical'|'both', onActivate?: (el: HTMLElement) => void }} [opts]
 */
export function roving(container, { items = '> *', orientation = 'horizontal', onActivate } = {}) {
  const list = () => [...container.querySelectorAll(`:scope ${items}`)].filter((el) => !el.matches(FOCUSABLE_DISABLED));
  const setCurrent = (el) => list().forEach((i) => i.setAttribute('tabindex', i === el ? '0' : '-1'));
  const rtl = () => getComputedStyle(container).direction === 'rtl';

  const initial = list();
  setCurrent(initial.find((i) => i.getAttribute('aria-selected') === 'true' || i.getAttribute('aria-current')) ?? initial[0]);

  const onKey = (e) => {
    const all = list();
    const i = all.indexOf(document.activeElement);
    if (i < 0) return;
    const prev = orientation === 'vertical' ? 'ArrowUp' : rtl() ? 'ArrowRight' : 'ArrowLeft';
    const next = orientation === 'vertical' ? 'ArrowDown' : rtl() ? 'ArrowLeft' : 'ArrowRight';
    let target;
    if (e.key === next || (orientation === 'both' && e.key === 'ArrowDown')) target = all[(i + 1) % all.length];
    else if (e.key === prev || (orientation === 'both' && e.key === 'ArrowUp')) target = all[(i - 1 + all.length) % all.length];
    else if (e.key === 'Home') target = all[0];
    else if (e.key === 'End') target = all.at(-1);
    else if ((e.key === 'Enter' || e.key === ' ') && onActivate && !all[i].matches('button, a, input, select, textarea')) {
      e.preventDefault();
      onActivate(all[i]);
      return;
    } else return;
    e.preventDefault();
    setCurrent(target);
    target.focus();
  };
  const onFocus = (e) => { if (list().includes(e.target)) setCurrent(e.target); };
  container.addEventListener('keydown', onKey);
  container.addEventListener('focusin', onFocus);
  return () => { container.removeEventListener('keydown', onKey); container.removeEventListener('focusin', onFocus); };
}

/** Toolbar: roving focus across its controls. Expects role="toolbar". */
export function toolbar(el) {
  const vertical = el.getAttribute('aria-orientation') === 'vertical';
  return roving(el, { items: 'button, [role="button"]', orientation: vertical ? 'vertical' : 'horizontal' });
}

/** Tabs: automatic activation (selection follows focus). Expects a role="tablist" whose tabs have aria-controls. */
export function tabs(tablist) {
  const all = () => [...tablist.querySelectorAll('[role="tab"]')];
  const select = (tab) => {
    for (const t of all()) {
      const on = t === tab;
      t.setAttribute('aria-selected', String(on));
      t.setAttribute('tabindex', on ? '0' : '-1');
      const panel = document.getElementById(t.getAttribute('aria-controls'));
      if (panel) panel.hidden = !on;
    }
  };
  select(all().find((t) => t.getAttribute('aria-selected') === 'true') ?? all()[0]);
  const stop = roving(tablist, { items: '[role="tab"]' });
  const onFocus = (e) => { if (e.target.matches('[role="tab"]')) select(e.target); };
  const onClick = (e) => { const t = e.target.closest('[role="tab"]'); if (t) select(t); };
  tablist.addEventListener('focusin', onFocus);
  tablist.addEventListener('click', onClick);
  return () => { stop(); tablist.removeEventListener('focusin', onFocus); tablist.removeEventListener('click', onClick); };
}

/**
 * Tree: ARIA treeview keyboard model (arrows, Home/End, Enter/Space select, Left/Right collapse/expand).
 * Sets aria-level and --ndl-level from nesting depth. Expects role="tree" with nested role="treeitem" / role="group".
 */
export function tree(root, { onSelect } = {}) {
  const items = () => [...root.querySelectorAll('[role="treeitem"]')];
  const visible = () => items().filter((el) => !el.parentElement.closest('[role="treeitem"][aria-expanded="false"]'));
  const levelOf = (el) => { let n = 0; for (let p = el.parentElement.closest('[role="treeitem"]'); p; p = p.parentElement.closest('[role="treeitem"]')) n++; return n; };
  for (const el of items()) {
    const lvl = levelOf(el);
    el.setAttribute('aria-level', String(lvl + 1));
    el.style.setProperty('--ndl-level', String(lvl));
    el.setAttribute('tabindex', '-1');
  }
  const focusItem = (el) => { items().forEach((i) => i.setAttribute('tabindex', i === el ? '0' : '-1')); el.focus(); };
  const select = (el) => {
    items().forEach((i) => i.setAttribute('aria-selected', String(i === el)));
    onSelect?.(el);
  };
  (items().find((i) => i.getAttribute('aria-selected') === 'true') ?? items()[0])?.setAttribute('tabindex', '0');

  const onKey = (e) => {
    const el = e.target.closest('[role="treeitem"]');
    if (!el || e.target.closest('input, select, textarea')) return;
    const vis = visible();
    const i = vis.indexOf(el);
    const expandable = el.hasAttribute('aria-expanded');
    const expanded = el.getAttribute('aria-expanded') === 'true';
    const rtl = getComputedStyle(root).direction === 'rtl';
    const key = rtl && e.key === 'ArrowLeft' ? 'ArrowRight' : rtl && e.key === 'ArrowRight' ? 'ArrowLeft' : e.key;
    switch (key) {
      case 'ArrowDown': focusItem(vis[Math.min(i + 1, vis.length - 1)]); break;
      case 'ArrowUp': focusItem(vis[Math.max(i - 1, 0)]); break;
      case 'Home': focusItem(vis[0]); break;
      case 'End': focusItem(vis.at(-1)); break;
      case 'ArrowRight':
        if (expandable && !expanded) el.setAttribute('aria-expanded', 'true');
        else if (expandable) focusItem(el.querySelector('[role="treeitem"]') ?? el);
        break;
      case 'ArrowLeft': {
        if (expandable && expanded) el.setAttribute('aria-expanded', 'false');
        else { const parent = el.parentElement.closest('[role="treeitem"]'); if (parent) focusItem(parent); }
        break;
      }
      case 'Enter': case ' ': select(el); break;
      default: return;
    }
    e.preventDefault();
  };
  const onClick = (e) => {
    const row = e.target.closest('.ndl-tree__row');
    const el = row?.closest('[role="treeitem"]');
    if (!el) return;
    focusItem(el);
    select(el);
    if (el.hasAttribute('aria-expanded') && e.detail === 2) el.setAttribute('aria-expanded', String(el.getAttribute('aria-expanded') !== 'true'));
  };
  root.addEventListener('keydown', onKey);
  root.addEventListener('click', onClick);
  return () => { root.removeEventListener('keydown', onKey); root.removeEventListener('click', onClick); };
}

/**
 * Splitter: drag or use arrow keys to resize the first pane of a .ndl-split.
 * Expects <div class="ndl-splitter" role="separator" tabindex="0"> between the two panes.
 */
export function splitter(el, { step = 16, min = 120 } = {}) {
  const split = el.parentElement;
  const vertical = split.dataset.orientation === 'vertical';
  el.setAttribute('aria-orientation', vertical ? 'horizontal' : 'vertical');
  const first = () => el.previousElementSibling;
  const size = () => (vertical ? first().getBoundingClientRect().height : first().getBoundingClientRect().width);
  const total = () => (vertical ? split.getBoundingClientRect().height : split.getBoundingClientRect().width);
  const expose = () => {
    el.setAttribute('aria-valuenow', String(Math.round((size() / total()) * 100)));
    el.setAttribute('aria-valuemin', '0');
    el.setAttribute('aria-valuemax', '100');
  };
  const set = (px) => {
    const grip = vertical ? el.getBoundingClientRect().height : el.getBoundingClientRect().width;
    const max = total() - grip - min;
    const v = Math.round(Math.min(Math.max(px, min), Math.max(min, max)));
    split.style.setProperty('--ndl-split-start', `${v}px`);
    expose();
  };
  expose();
  const onKey = (e) => {
    const dec = vertical ? 'ArrowUp' : 'ArrowLeft';
    const inc = vertical ? 'ArrowDown' : 'ArrowRight';
    if (e.key === dec) set(size() - step);
    else if (e.key === inc) set(size() + step);
    else if (e.key === 'Home') set(min);
    else if (e.key === 'End') set(total());
    else return;
    e.preventDefault();
  };
  const onDown = (e) => {
    el.setPointerCapture(e.pointerId);
    el.dataset.dragging = '';
    const origin = vertical ? e.clientY : e.clientX;
    const start = size();
    const move = (m) => set(start + (vertical ? m.clientY : m.clientX) - origin);
    const up = () => { delete el.dataset.dragging; el.removeEventListener('pointermove', move); el.removeEventListener('pointerup', up); };
    el.addEventListener('pointermove', move);
    el.addEventListener('pointerup', up);
  };
  el.addEventListener('keydown', onKey);
  el.addEventListener('pointerdown', onDown);
  return () => { el.removeEventListener('keydown', onKey); el.removeEventListener('pointerdown', onDown); };
}

/** Keeps --ndl-fill in step with a range input so the filled part of the track follows the value. */
export function slider(input) {
  const update = () => {
    const min = Number(input.min || 0), max = Number(input.max || 100);
    input.style.setProperty('--ndl-fill', `${((input.value - min) / (max - min)) * 100}%`);
  };
  update();
  input.addEventListener('input', update);
  return () => input.removeEventListener('input', update);
}

/**
 * Context menu: opens on right click, Shift+F10 or the Menu key; arrow keys move; Escape or an outside click closes
 * and returns focus to the trigger. The menu element needs role="menu" and items with role="menuitem".
 */
export function contextMenu(trigger, menu, { onSelect } = {}) {
  const items = () => [...menu.querySelectorAll('[role="menuitem"]')].filter((i) => i.getAttribute('aria-disabled') !== 'true');
  const close = () => { menu.hidden = true; trigger.focus?.(); };
  const open = (x, y) => {
    menu.hidden = false;
    const r = menu.getBoundingClientRect();
    menu.style.left = `${Math.max(0, Math.min(x, innerWidth - r.width))}px`;
    menu.style.top = `${Math.max(0, Math.min(y, innerHeight - r.height))}px`;
    items().forEach((i, n) => i.setAttribute('tabindex', n === 0 ? '0' : '-1'));
    items()[0]?.focus();
  };
  const onContext = (e) => { e.preventDefault(); open(e.clientX, e.clientY); };
  const onTriggerKey = (e) => {
    if ((e.key === 'F10' && e.shiftKey) || e.key === 'ContextMenu') {
      e.preventDefault();
      const r = trigger.getBoundingClientRect();
      open(r.left + r.width / 2, r.top + r.height / 2);
    }
  };
  const onMenuKey = (e) => {
    const all = items();
    const i = all.indexOf(document.activeElement);
    if (e.key === 'Escape') { e.preventDefault(); close(); return; }
    if (e.key === 'Tab') { e.preventDefault(); return; }
    let t;
    if (e.key === 'ArrowDown') t = all[(i + 1) % all.length];
    else if (e.key === 'ArrowUp') t = all[(i - 1 + all.length) % all.length];
    else if (e.key === 'Home') t = all[0];
    else if (e.key === 'End') t = all.at(-1);
    else if ((e.key === 'Enter' || e.key === ' ') && i >= 0) { e.preventDefault(); onSelect?.(all[i]); close(); return; }
    else return;
    e.preventDefault();
    all.forEach((a) => a.setAttribute('tabindex', a === t ? '0' : '-1'));
    t.focus();
  };
  const onMenuClick = (e) => { const it = e.target.closest('[role="menuitem"]'); if (it && it.getAttribute('aria-disabled') !== 'true') { onSelect?.(it); close(); } };
  const onOutside = (e) => { if (!menu.hidden && !menu.contains(e.target)) menu.hidden = true; };
  trigger.addEventListener('contextmenu', onContext);
  trigger.addEventListener('keydown', onTriggerKey);
  menu.addEventListener('keydown', onMenuKey);
  menu.addEventListener('click', onMenuClick);
  document.addEventListener('pointerdown', onOutside);
  return () => {
    trigger.removeEventListener('contextmenu', onContext); trigger.removeEventListener('keydown', onTriggerKey);
    menu.removeEventListener('keydown', onMenuKey); menu.removeEventListener('click', onMenuClick);
    document.removeEventListener('pointerdown', onOutside);
  };
}

/**
 * Tooltips for every [data-ndl-tooltip] below root. Shown after a short delay on hover and immediately on keyboard focus;
 * Escape dismisses. The text is also exposed through aria-describedby.
 */
export function tooltips(root = document, { delay = 500 } = {}) {
  const tip = document.createElement('div');
  tip.className = 'ndl-tooltip';
  tip.setAttribute('role', 'tooltip');
  tip.id = `ndl-tooltip-${Math.random().toString(36).slice(2, 8)}`;
  tip.hidden = true;
  document.body.append(tip);
  let timer;
  const hide = () => { clearTimeout(timer); tip.hidden = true; };
  const show = (el) => {
    tip.textContent = el.dataset.ndlTooltip;
    el.setAttribute('aria-describedby', tip.id);
    tip.hidden = false;
    const r = el.getBoundingClientRect();
    const t = tip.getBoundingClientRect();
    tip.style.left = `${Math.max(0, Math.min(r.left + r.width / 2 - t.width / 2, innerWidth - t.width))}px`;
    tip.style.top = `${r.bottom + 6 + t.height > innerHeight ? r.top - t.height - 6 : r.bottom + 6}px`;
  };
  const target = (e) => e.target.closest?.('[data-ndl-tooltip]');
  const over = (e) => { const el = target(e); if (el) { clearTimeout(timer); timer = setTimeout(() => show(el), delay); } };
  const focus = (e) => { const el = target(e); if (el && el.matches(':focus-visible')) show(el); };
  const key = (e) => { if (e.key === 'Escape') hide(); };
  root.addEventListener('pointerover', over);
  root.addEventListener('pointerout', hide);
  root.addEventListener('focusin', focus);
  root.addEventListener('focusout', hide);
  document.addEventListener('keydown', key);
  return () => {
    hide(); tip.remove();
    root.removeEventListener('pointerover', over); root.removeEventListener('pointerout', hide);
    root.removeEventListener('focusin', focus); root.removeEventListener('focusout', hide);
    document.removeEventListener('keydown', key);
  };
}

/** Applies appearance preferences to <html>. Pass null to return an axis to the system preference. */
export function setAppearance({ theme, contrast, density, motion } = {}, root = document.documentElement) {
  const apply = (name, value) => (value == null ? root.removeAttribute(name) : root.setAttribute(name, value));
  if (theme !== undefined) apply('data-theme', theme);
  if (contrast !== undefined) apply('data-contrast', contrast);
  if (density !== undefined) apply('data-density', density);
  if (motion !== undefined) apply('data-motion', motion);
}
