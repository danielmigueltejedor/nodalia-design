// A small isometric scene for the NDL volumetric CAD icons.
//
// Every icon is built from solids (boxes, extruded profiles, cylinders, cones, spheres, tori) and
// overlays (sketch curves, arrows, dots) placed in model space and projected isometrically onto a
// 64 x 64 view box. Faces are shaded by their normal into three tones: the top is the palest, the
// faces turned to the left the middle one and the faces turned to the right the darkest. Geometry
// the tool creates or changes uses the accent ramp, everything else the neutral ramp, and outlines
// use the ink. The output uses only role colours (ROLES) so an app can recolour finished artwork.

export const ROLES = {
  ink: "#26303d",
  // Line art that sits on the interface background rather than on a face; an app may draw it
  // in its text colour.
  line: "#28323f",
  "face-1": "#eceef2",
  "face-2": "#c3c7d1",
  "face-3": "#9499a8",
  "accent-1": "#f3e4ac",
  "accent-2": "#d1b453",
  "accent-3": "#8f7518",
  guide: "#7a8193",
  paper: "#ffffff",
};

// Display values for a dark interface. Outlines stay dark: the shaded faces carry the contrast;
// only line art drawn on the background turns light.
export const ROLES_DARK = {
  ink: "#12141a",
  line: "#dfe2ea",
  "face-1": "#b0b5c3",
  "face-2": "#868c9f",
  "face-3": "#61677a",
  "accent-1": "#e8cf7c",
  "accent-2": "#c8a63d",
  "accent-3": "#9b7e1d",
  guide: "#959cad",
  paper: "#2b2e38",
};

export const STROKE = 2.4;
const C = Math.cos(Math.PI / 6);
const ISO_EL = (Math.asin(Math.tan(Math.PI / 6)) * 180) / Math.PI;
const ISO_K = 1 / Math.cos((ISO_EL * Math.PI) / 180);
const r2 = (n) => Math.round(n * 100) / 100;

// Model space: x runs to the lower right, y to the lower left and z up. One model unit is one view
// box unit along each axis before the 30 degree foreshortening.
// A scene can turn the camera: `az` is the azimuth in degrees (45 is isometric) and `el` the
// elevation (35.26 is isometric). Sketch icons look down more steeply so plane geometry reads.
export function project([x, y, z], s) {
  const a = ((s.az ?? 45) * Math.PI) / 180, e = ((s.el ?? ISO_EL) * Math.PI) / 180, k = s.k * ISO_K;
  const u = x * Math.cos(a) - y * Math.sin(a), w = x * Math.sin(a) + y * Math.cos(a);
  return [s.cx + u * k, s.cy + w * k * Math.sin(e) - z * k * Math.cos(e)];
}

const sub = (a, b) => a.map((v, i) => v - b[i]);
const add = (a, b) => a.map((v, i) => v + b[i]);
const mul = (a, k) => a.map((v) => v * k);
const dot = (a, b) => a.reduce((t, v, i) => t + v * b[i], 0);
const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const norm = (a) => { const l = Math.hypot(...a) || 1; return a.map((v) => v / l); };
let VIEW = norm([1, 1, 1]);
// The direction towards the camera for a scene's projection.
function viewFor(s) {
  const a = ((s.az ?? 45) * Math.PI) / 180, e = ((s.el ?? ISO_EL) * Math.PI) / 180;
  return norm([Math.sin(a) * Math.cos(e), Math.cos(a) * Math.cos(e), Math.sin(e)]);
}
const LIGHT = norm([-0.15, 0.55, 1]);

function tone(n) {
  const nn = norm(n);
  if (nn[2] > 0.55) return 1;
  return dot(nn, LIGHT) > -0.05 && nn[1] >= nn[0] - 0.05 ? 2 : 3;
}

// Curved sheets shade by how squarely a cell faces the light, in three steps.
const SHEET_LIGHT = norm([-0.5, 0.35, 1]);
function sheetTone(n) { const d = dot(norm(n), SHEET_LIGHT); return d > 0.8 ? 1 : d > 0.5 ? 2 : 3; }

function ramp(kind, t) { return (kind === "accent" ? "accent-" : "face-") + t; }

// A planar polygon in model space. `kind` picks the ramp.
function face(pts, kind, extra = {}) {
  const n = norm(cross(sub(pts[1], pts[0]), sub(pts[2], pts[0])));
  const c = mul(pts.reduce((a, p) => add(a, p), [0, 0, 0]), 1 / pts.length);
  return { pts, n, depth: dot(c, VIEW) + (extra.bias || 0), kind, ...extra };
}

// Basis for a plane named by its normal axis.
function basis(axis) {
  if (axis === "x") return [[0, 1, 0], [0, 0, 1], [1, 0, 0]];
  if (axis === "y") return [[1, 0, 0], [0, 0, 1], [0, 1, 0]];
  return [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
}

export class Scene {
  constructor({ k = 1, cx = 32, cy = 32, az = 45, el = ISO_EL } = {}) {
    this.s = { k, cx, cy, az, el };
    VIEW = viewFor(this.s);
    this.items = []; // solids: arrays of faces, drawn back to front
    this.over = []; // overlays, drawn after the solids in order
    this.under = []; // planes and guides drawn before the solids
  }

  // Overlays go "under" every solid, "over" every solid, or, given a number n, right after the
  // n-th solid added (a sketch drawn on a board, under a part floating above it).
  put(layer, o) {
    if (typeof layer === "number") (this.attached ??= []).push({ ...o, after: layer });
    else this[layer].push(o);
  }

  // ---------- solids ----------
  // Extrude a profile drawn in the plane normal to `axis` (coordinates in that plane's u,v) from
  // `from` to `to` along the axis. `at` offsets the whole solid.
  // A profile point may carry a third entry, "s", to mark a smooth vertex: the side faces meeting
  // there are one curved surface, so no edge is drawn between them except on the silhouette.
  prism(profile, { axis = "z", from = 0, to = 10, at = [0, 0, 0], kind = "face", bias = 0, sideKind = null, capKind = null } = {}) {
    const [u, v, w] = basis(axis);
    const P = (q, h) => add(at, add(add(mul(u, q[0]), mul(v, q[1])), mul(w, h)));
    const lo = Math.min(from, to), hi = Math.max(from, to);
    const ccw = area(profile) > 0;
    const prof = ccw ? profile : [...profile].reverse();
    const bottom = prof.map((q) => P(q, lo)), top = prof.map((q) => P(q, hi));
    const smooth = prof.map((q) => q[2] === "s");
    const m = prof.length;
    // sideKind(i) can pick a ramp per side face, i indexing the profile as given.
    const idx = (i) => (ccw ? i : m - 2 - i < 0 ? m - 1 : m - 2 - i);
    const sides = prof.map((_, i) => face([bottom[i], bottom[(i + 1) % m], top[(i + 1) % m], top[i]], (sideKind && sideKind(idx(i))) || kind, { bias }));
    const shown = sides.map((f) => dot(f.n, VIEW) > 1e-6);
    sides.forEach((f, i) => {
      const j = (i + 1) % m, h = (i - 1 + m) % m;
      f.edges = [[bottom[i], bottom[j]], [top[i], top[j]]];
      if (!smooth[j] || !shown[j]) f.edges.push([bottom[j], top[j]]);
      if (!smooth[i] || !shown[h]) f.edges.push([bottom[i], top[i]]);
    });
    const capTop = face([...top], capKind || kind, { bias }), capBot = face([...bottom].reverse(), capKind || kind, { bias });
    capTop.edges = top.map((p, i) => [p, top[(i + 1) % m]]);
    capBot.edges = bottom.map((p, i) => [p, bottom[(i + 1) % m]]);
    // Caps go over their side faces so a cap's outline is never cut by a side drawn later.
    capTop.depth += 0.01; capBot.depth += 0.01;
    const faces = [capTop, capBot, ...sides];
    // Order solids by the centre of their bounds: a part resting on another is drawn after it.
    const all = [...top, ...bottom], lo3 = [0, 1, 2].map((k) => Math.min(...all.map((p) => p[k]))), hi3 = [0, 1, 2].map((k) => Math.max(...all.map((p) => p[k])));
    this.items.push({ faces, depth: dot(mul(add(lo3, hi3), 0.5), VIEW) + bias, lo: lo3, hi: hi3, bias });
    return this;
  }

  box([x0, y0, z0], [x1, y1, z1], opts = {}) {
    return this.prism([[x0, y0], [x1, y0], [x1, y1], [x0, y1]], { ...opts, axis: "z", from: z0, to: z1 });
  }

  // A solid of revolution around `axis` through `at`: radius r0 at h0, r1 at h1 (cylinder, cone,
  // frustum). Drawn with banded side shading and the visible cap.
  turned({ at = [0, 0, 0], axis = "z", r0 = 10, r1 = r0, h0 = 0, h1 = 10, kind = "face", bias = 0, n = 48, hole = 0, rot = 0 } = {}) {
    const [u, v, w] = basis(axis);
    const ring = (r, h) => Array.from({ length: n }, (_, i) => {
      const t = (i / n) * Math.PI * 2 + (rot * Math.PI) / 180;
      return add(at, add(add(mul(u, Math.cos(t) * r), mul(v, Math.sin(t) * r)), mul(w, h)));
    });
    const A = ring(r0, h0), B = ring(r1, h1);
    const faces = [];
    const slope = (r0 - r1) / Math.max(1e-6, h1 - h0);
    for (let i = 0; i < n; i++) {
      const j = (i + 1) % n;
      const t = ((i + 0.5) / n) * Math.PI * 2 + (rot * Math.PI) / 180;
      const radial = add(mul(u, Math.cos(t)), mul(v, Math.sin(t)));
      const nn = norm(add(radial, mul(w, slope)));
      if (dot(nn, VIEW) <= 0) continue;
      faces.push({ pts: [A[i], A[j], B[j], B[i]], n: nn, kind, band: true });
    }
    // Merge adjacent strips of one tone into a single polygon so the side reads smooth.
    const merged = [];
    for (const f of faces) {
      const t = tone(f.n), last = merged[merged.length - 1];
      if (last && last.t === t && samePt(last.bot[last.bot.length - 1], f.pts[0])) {
        last.bot.push(f.pts[1]); last.top.push(f.pts[2]);
      } else merged.push({ t, n: f.n, bot: [f.pts[0], f.pts[1]], top: [f.pts[3], f.pts[2]] });
    }
    if (merged.length > 1) {
      const a = merged[0], z = merged[merged.length - 1];
      if (a.t === z.t && samePt(z.bot[z.bot.length - 1], a.bot[0])) {
        z.bot.push(...a.bot.slice(1)); z.top.push(...a.top.slice(1)); merged.shift();
      }
    }
    const out = merged.map((m) => ({ pts: [...m.bot, ...[...m.top].reverse()], n: m.n, kind, depth: -1e3 }));
    const capH = dot(w, VIEW) > 0 ? h1 : h0, capR = capH === h1 ? r1 : r0;
    if (capR > 0.01) {
      const cap = ring(capR, capH);
      const capFace = { pts: cap, n: capH === h1 ? w : mul(w, -1), kind, depth: 1e3 };
      if (hole > 0) capFace.hole = ring(hole, capH);
      out.push(capFace);
    }
    // The whole silhouette, outlined once on top.
    const AB = [...A, ...B];
    this.items.push({ faces: out, depth: dot(add(at, mul(w, (h0 + h1) / 2)), VIEW) + bias, outline: hull(AB, this.s), lo: [0, 1, 2].map((k) => Math.min(...AB.map((p) => p[k]))), hi: [0, 1, 2].map((k) => Math.max(...AB.map((p) => p[k]))), bias });
    return this;
  }

  cylinder(at, r, h, opts = {}) { return this.turned({ at, r0: r, r1: r, h0: 0, h1: h, ...opts }); }

  // A thin surface z = f(x, y) over a rectangle, shaded per cell and outlined at its boundary.
  // `paint(x, y, n)` may return a role to override a cell's shading (zebra stripes, curvature).
  sheet(f, { x0 = -14, x1 = 14, y0 = -14, y1 = 14, n = 10, kind = "face", paint = null, bias = 0, edge = "ink", grid = null } = {}) {
    const X = (i) => x0 + ((x1 - x0) * i) / n, Y = (j) => y0 + ((y1 - y0) * j) / n;
    const pt = (i, j) => [X(i), Y(j), f(X(i), Y(j))];
    const cells = [];
    for (let i = 0; i < n; i++) for (let j = 0; j < n; j++) {
      const q = [pt(i, j), pt(i + 1, j), pt(i + 1, j + 1), pt(i, j + 1)];
      let nn = norm(cross(sub(q[2], q[0]), sub(q[3], q[1])));
      if (dot(nn, VIEW) < 0) nn = mul(nn, -1);
      const c = mul(q.reduce((a, p) => add(a, p), [0, 0, 0]), 0.25);
      cells.push({ pts: q, n: nn, kind, depth: dot(c, VIEW), role: paint ? paint(c[0], c[1], nn, i, j) : null });
    }
    const rim = [];
    for (let i = 0; i <= n; i++) rim.push(pt(i, 0));
    for (let j = 1; j <= n; j++) rim.push(pt(n, j));
    for (let i = n - 1; i >= 0; i--) rim.push(pt(i, n));
    for (let j = n - 1; j > 0; j--) rim.push(pt(0, j));
    const lines = [];
    if (grid) for (let g = 1; g < grid; g++) {
      const k = Math.round((n * g) / grid);
      lines.push(Array.from({ length: n + 1 }, (_, j) => pt(k, j)), Array.from({ length: n + 1 }, (_, i) => pt(i, k)));
    }
    this.items.push({ sheet: { cells, rim, edge, lines }, depth: dot([(x0 + x1) / 2, (y0 + y1) / 2, f((x0 + x1) / 2, (y0 + y1) / 2)], VIEW) + bias,
      lo: [x0, y0, Math.min(...rim.map((p) => p[2]))], hi: [x1, y1, Math.max(...rim.map((p) => p[2]))], bias });
    return this;
  }

  sphere(at, r, { kind = "face", bias = 0 } = {}) {
    this.items.push({ sphere: { at, r }, kind, depth: dot(at, VIEW) + bias, lo: at.map((v) => v - r), hi: at.map((v) => v + r), bias });
    return this;
  }

  torus(at, R, r, { kind = "face", bias = 0 } = {}) {
    this.items.push({ torus: { at, R, r }, kind, depth: dot(at, VIEW) + bias, lo: [at[0] - R - r, at[1] - R - r, at[2] - r], hi: [at[0] + R + r, at[1] + R + r, at[2] + r], bias });
    return this;
  }

  // A round hole cut into a face: the opening, the far wall inside it and the floor's edge. Drawn as
  // an overlay on the solid it cuts, so call it after the solid.
  bore(at, r, depth, { axis = "z", kind = "face", through = false } = {}) {
    this.over.push({ bore: { at, r, depth, axis, kind, through } });
    return this;
  }

  // A pocket cut down from a face: a convex opening (model points on the face), its visible inner
  // walls and its floor, `depth` deep along -normal.
  pocket(opening, depth, { normal = [0, 0, 1], kind = "face", through = false } = {}) {
    this.over.push({ pocket: { opening, depth, normal, kind, through } });
    return this;
  }

  // ---------- overlays ----------
  // A polyline or closed loop in model space.
  line(pts, { role = "accent-3", width = STROKE, closed = false, dash = null, layer = "over" } = {}) {
    this.put(layer, { line: pts, role, width, closed, dash });
    return this;
  }

  // A circle or arc lying in the plane normal to `axis`.
  arc(at, r, { axis = "z", a0 = 0, a1 = 360, ...o } = {}) {
    const [u, v] = basis(axis);
    const n = Math.max(8, Math.round(Math.abs(a1 - a0) / 7.5));
    const pts = Array.from({ length: n + 1 }, (_, i) => {
      const t = ((a0 + ((a1 - a0) * i) / n) * Math.PI) / 180;
      return add(at, add(mul(u, Math.cos(t) * r), mul(v, Math.sin(t) * r)));
    });
    return this.line(pts, { closed: Math.abs(a1 - a0) >= 360, ...o });
  }

  // A filled flat shape lying in a plane (a sketch region, a plane, a sheet).
  plane(pts, { role = "face-1", stroke = "guide", width = STROKE * 0.75, layer = "under", dash = null } = {}) {
    this.put(layer, { poly: pts, role, stroke, width, dash });
    return this;
  }

  dot(p, { r = 3, role = "accent-3", ring = "ink", layer = "over" } = {}) {
    this.put(layer, { dot: p, r, role, ring });
    return this;
  }

  // A block arrow drawn in screen space from model point a to b.
  arrow(a, b, { role = "accent-3", w = 3.2, head = 7.5, layer = "over" } = {}) {
    this.put(layer, { arrow: [a, b], role, w, head });
    return this;
  }

  // A curved arrow along an arc (degrees) in the plane normal to `axis`, head at a1.
  arcArrow(at, r, a0, a1, { axis = "z", role = "accent-3", width = 2.8, head = 6 } = {}) {
    this.over.push({ arcArrow: { at, r, a0, a1, axis }, role, width, head });
    return this;
  }

  // Raw screen-space overlay in view box units: a path `d` (absolute M L C Q A Z only).
  path(d, { fill = "none", stroke = "ink", width = STROKE, layer = "over" } = {}) {
    this.put(layer, { raw: d, fill, stroke, width });
    return this;
  }

  // ---------- output ----------
  svg(title) {
    const s = this.s, out = [];
    VIEW = viewFor(s);
    const P = (p) => project(p, s);
    const pts = (arr) => arr.map((p) => P(p)).map(([x, y]) => `${r2(x)} ${r2(y)}`);
    const poly = (arr) => "M" + pts(arr).join("L") + "Z";
    const col = (r) => ROLES[r] ?? r;
    const strokeAttr = (r, w) => `stroke="${col(r)}" stroke-width="${r2(w)}" stroke-linejoin="round" stroke-linecap="round"`;
    const emitOverlay = (o) => {
      if (o.line) {
        const d = "M" + pts(o.line).join("L") + (o.closed ? "Z" : "");
        out.push(`<path d="${d}" fill="none" ${strokeAttr(o.role, o.width)}${o.dash ? ` stroke-dasharray="${o.dash}"` : ""}/>`);
      } else if (o.poly) {
        out.push(`<path d="${poly(o.poly)}" fill="${col(o.role)}"${o.stroke ? " " + strokeAttr(o.stroke, o.width) : ""}${o.dash ? ` stroke-dasharray="${o.dash}"` : ""}/>`);
      } else if (o.dot) {
        const [x, y] = P(o.dot);
        out.push(`<circle cx="${r2(x)}" cy="${r2(y)}" r="${r2(o.r)}" fill="${col(o.role)}"${o.ring ? " " + strokeAttr(o.ring, 1.6) : ""}/>`);
      } else if (o.arrow) {
        out.push(`<path d="${blockArrow(P(o.arrow[0]), P(o.arrow[1]), o.w, o.head)}" fill="${col(o.role)}" ${strokeAttr("ink", 1.6)}/>`);
      } else if (o.bore) {
        const { at, r, depth, axis, kind, through } = o.bore;
        const [u, v, w] = basis(axis);
        const ring = (h) => Array.from({ length: 48 }, (_, i) => { const t = (i / 48) * Math.PI * 2; return add(at, add(add(mul(u, Math.cos(t) * r), mul(v, Math.sin(t) * r)), mul(w, -h))); });
        const lip = ring(0);
        out.push(`<path d="${poly(lip)}" fill="${col(through ? "paper" : ramp(kind, 3))}"/>`);
        // The floor seen through the opening: the floor ellipse clipped to the opening.
        if (!through) {
          const seen = clipConvex(ring(depth).map(P), lip.map(P));
          if (seen.length > 2) out.push(`<path d="M${seen.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}Z" fill="${col(ramp(kind, 2))}"/>`);
        }
        out.push(`<path d="${poly(lip)}" fill="none" ${strokeAttr("ink", STROKE)}/>`);
      } else if (o.pocket) {
        const { opening, depth, normal, kind, through } = o.pocket;
        const down = mul(norm(normal), -depth);
        const floor = opening.map((p) => add(p, down));
        const openS = opening.map(P);
        const fillS = (arr, role) => { if (arr.length > 2) out.push(`<path d="M${arr.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}Z" fill="${col(role)}" ${strokeAttr(role, 0.6)}/>`); };
        fillS(openS, through ? "paper" : ramp(kind, 3));
        const c = mul(opening.reduce((a, p) => add(a, p), [0, 0, 0]), 1 / opening.length);
        const walls = [];
        for (let i = 0; i < opening.length; i++) {
          const a = opening[i], b = opening[(i + 1) % opening.length];
          const mid = mul(add(a, b), 0.5);
          const inward = norm(sub(c, mid));
          const nIn = norm(sub(inward, mul(norm(normal), dot(inward, norm(normal)))));
          if (dot(nIn, VIEW) > 1e-6) walls.push({ q: [a, b, add(b, down), add(a, down)], n: nIn });
        }
        for (const wl of walls) fillS(clipConvex(wl.q.map(P), openS), ramp(kind, tone(wl.n)));
        if (!through) fillS(clipConvex(floor.map(P), openS), ramp(kind, tone(normal)));
        out.push(`<path d="M${openS.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}Z" fill="none" ${strokeAttr("ink", STROKE)}/>`);
      } else if (o.arcArrow) {
        const { at, r, a0, a1, axis } = o.arcArrow;
        const [u, v] = basis(axis);
        const n = 40, pt = (deg) => P(add(at, add(mul(u, Math.cos((deg * Math.PI) / 180) * r), mul(v, Math.sin((deg * Math.PI) / 180) * r))));
        const sp = Array.from({ length: n + 1 }, (_, i) => pt(a0 + ((a1 - a0) * i) / n));
        const tip = sp[n], back = sp[n - 3];
        const dx = tip[0] - back[0], dy = tip[1] - back[1], L = Math.hypot(dx, dy) || 1, ux = dx / L, uy = dy / L;
        const h = o.head, base = [tip[0] - ux * h, tip[1] - uy * h];
        let cut = n; while (cut > 1 && Math.hypot(sp[cut][0] - tip[0], sp[cut][1] - tip[1]) < h * 0.7) cut--;
        const body = sp.slice(0, cut + 1);
        out.push(`<path d="M${body.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}" fill="none" ${strokeAttr("ink", o.width + 2.6)}/>`);
        out.push(`<path d="M${body.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}" fill="none" ${strokeAttr(o.role, o.width)}/>`);
        const hw = h * 0.62;
        out.push(`<path d="M${r2(tip[0])} ${r2(tip[1])}L${r2(base[0] - uy * hw)} ${r2(base[1] + ux * hw)}L${r2(base[0] + uy * hw)} ${r2(base[1] - ux * hw)}Z" fill="${col(o.role)}" ${strokeAttr("ink", 1.6)}/>`);
      } else if (o.raw) {
        out.push(`<path d="${o.raw}" fill="${col(o.fill)}"${o.stroke ? " " + strokeAttr(o.stroke, o.width) : ""}/>`);
      }
    };
    this.under.forEach(emitOverlay);
    const items = paintOrder(this.items);
    const attachedTo = (it) => (this.attached || []).filter((o) => this.items[o.after] === it);
    for (const it of items) {
      drawItem(it);
      attachedTo(it).forEach(emitOverlay);
    }
    this.over.forEach(emitOverlay);
    return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">\n  <title>${title}</title>\n  ${out.join("\n  ")}\n</svg>\n`;
    function drawItem(it) {
      if (it.sphere) { out.push(...sphereSvg(it, s, col)); return; }
      if (it.torus) { out.push(...torusSvg(it, s, col)); return; }
      if (it.sheet) {
        for (const c of [...it.sheet.cells].sort((a, b) => a.depth - b.depth)) {
          const fill = col(c.role || ramp(c.kind, sheetTone(c.n)));
          out.push(`<path d="${poly(c.pts)}" fill="${fill}" ${strokeAttr(fill, 0.6)}/>`);
        }
        for (const l of it.sheet.lines) out.push(`<path d="M${pts(l).join("L")}" fill="none" ${strokeAttr("ink", 1.1)}/>`);
        if (it.sheet.edge) out.push(`<path d="${poly(it.sheet.rim)}" fill="none" ${strokeAttr(it.sheet.edge, STROKE)}/>`);
        return;
      }
      const vis = it.faces.filter((f) => f.band || f.hole || dot(f.n, VIEW) > 1e-6).sort((a, b) => a.depth - b.depth);
      for (const f of vis) {
        const c = col(ramp(f.kind, tone(f.n)));
        if (f.hole) out.push(`<path d="${poly(f.pts)}${poly([...f.hole].reverse())}" fill="${c}" fill-rule="evenodd" ${strokeAttr("ink", STROKE)}/>`);
        else if (f.edges) {
          out.push(`<path d="${poly(f.pts)}" fill="${c}" ${strokeAttr(c, 0.6)}/>`);
          out.push(`<path d="${f.edges.map((e) => "M" + pts(e).join("L")).join("")}" fill="none" ${strokeAttr("ink", STROKE)}/>`);
        } else out.push(`<path d="${poly(f.pts)}" fill="${c}" ${it.outline ? strokeAttr(c, 0.6) : strokeAttr("ink", STROKE)}/>`);
      }
      if (it.outline) {
        out.push(`<path d="M${it.outline.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}Z" fill="none" ${strokeAttr("ink", STROKE)}/>`);
        const cap = vis.find((f) => f.depth === 1e3);
        if (cap) out.push(`<path d="${poly(cap.pts)}" fill="none" ${strokeAttr("ink", STROKE)}/>`);
      }
    }
  }
}

// Painter's order for solids. Where two solids' bounds are separated along an axis the one on the
// far side is drawn first (that settles parts resting on or beside each other); otherwise, and
// for an explicit bias, the centre depth decides.
function paintOrder(items) {
  const before = (a, b) => {
    if (a.bias !== b.bias) return a.bias < b.bias;
    for (let k = 0; k < 3; k++) {
      if (a.hi[k] <= b.lo[k] + 1e-6) return true;
      if (b.hi[k] <= a.lo[k] + 1e-6) return false;
    }
    return a.depth < b.depth;
  };
  const rest = [...items], out = [];
  while (rest.length) {
    // Take a solid nothing remaining must precede.
    let pick = rest.findIndex((a) => rest.every((b) => b === a || !before(b, a)));
    if (pick < 0) pick = rest.reduce((m, a, i) => (a.depth < rest[m].depth ? i : m), 0);
    out.push(rest.splice(pick, 1)[0]);
  }
  return out;
}

// Sutherland-Hodgman: the part of convex polygon `subject` inside convex polygon `clip` (screen space).
function clipConvex(subject, clip) {
  const sign = Math.sign(area(clip.map(([x, y]) => [x, y]))) || 1;
  const inside = (p, a, b) => sign * ((b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])) >= 0;
  const cut = (p, q, a, b) => {
    const [x1, y1] = p, [x2, y2] = q, [x3, y3] = a, [x4, y4] = b;
    const d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4) || 1e-9;
    const t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / d;
    return [x1 + t * (x2 - x1), y1 + t * (y2 - y1)];
  };
  let out = subject;
  for (let i = 0; i < clip.length && out.length; i++) {
    const a = clip[i], b = clip[(i + 1) % clip.length], inp = out; out = [];
    for (let j = 0; j < inp.length; j++) {
      const p = inp[j], q = inp[(j + 1) % inp.length];
      if (inside(q, a, b)) { if (!inside(p, a, b)) out.push(cut(p, q, a, b)); out.push(q); }
      else if (inside(p, a, b)) out.push(cut(p, q, a, b));
    }
  }
  return out;
}

function area(p) { let a = 0; for (let i = 0; i < p.length; i++) { const j = (i + 1) % p.length; a += p[i][0] * p[j][1] - p[j][0] * p[i][1]; } return a / 2; }
function samePt(a, b) { return Math.abs(a[0] - b[0]) + Math.abs(a[1] - b[1]) + Math.abs(a[2] - b[2]) < 1e-6; }

// Convex hull (screen space) of projected points: the silhouette of a turned solid.
function hull(points, s) {
  const p = points.map((q) => project(q, s)).sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cr = (o, a, b) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const lo = [], up = [];
  for (const q of p) { while (lo.length >= 2 && cr(lo[lo.length - 2], lo[lo.length - 1], q) <= 0) lo.pop(); lo.push(q); }
  for (const q of [...p].reverse()) { while (up.length >= 2 && cr(up[up.length - 2], up[up.length - 1], q) <= 0) up.pop(); up.push(q); }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
}

function blockArrow([x0, y0], [x1, y1], w, head) {
  const dx = x1 - x0, dy = y1 - y0, L = Math.hypot(dx, dy) || 1, ux = dx / L, uy = dy / L, nx = -uy, ny = ux;
  const hb = Math.max(0, L - head), hw = w * 1.15 + 1.6;
  const P = (a, b) => `${r2(x0 + ux * a + nx * b)} ${r2(y0 + uy * a + ny * b)}`;
  return `M${P(0, -w / 2)}L${P(hb, -w / 2)}L${P(hb, -hw)}L${P(L, 0)}L${P(hb, hw)}L${P(hb, w / 2)}L${P(0, w / 2)}Z`;
}

function sphereSvg(it, s, col) {
  const [x, y] = project(it.sphere.at, s), r = it.sphere.r * s.k;
  const k = it.kind;
  return [
    `<circle cx="${r2(x)}" cy="${r2(y)}" r="${r2(r)}" fill="${col(ramp(k, 3))}"/>`,
    `<circle cx="${r2(x - r * 0.12)}" cy="${r2(y - r * 0.12)}" r="${r2(r * 0.86)}" fill="${col(ramp(k, 2))}"/>`,
    `<circle cx="${r2(x - r * 0.3)}" cy="${r2(y - r * 0.3)}" r="${r2(r * 0.48)}" fill="${col(ramp(k, 1))}"/>`,
    `<circle cx="${r2(x)}" cy="${r2(y)}" r="${r2(r)}" fill="none" stroke="${col("ink")}" stroke-width="${STROKE}" stroke-linejoin="round" stroke-linecap="round"/>`,
  ];
}

function torusSvg(it, s, col) {
  // The tube as a ring of overlapping shaded discs, far ones first, then the outer silhouette and
  // the visible rim of the hole in ink.
  const { at, R, r } = it.torus, k = it.kind, n = 72;
  const centres = Array.from({ length: n }, (_, i) => { const t = (i / n) * Math.PI * 2; return add(at, [Math.cos(t) * R, Math.sin(t) * R, 0]); })
    .sort((a, b) => dot(a, VIEW) - dot(b, VIEW));
  const out = [];
  const rr = r * s.k;
  for (const c of centres) {
    const [x, y] = project(c, s);
    out.push(`<circle cx="${r2(x)}" cy="${r2(y)}" r="${r2(rr)}" fill="${col(ramp(k, 3))}"/>`);
  }
  for (const c of centres) {
    const [x, y] = project(c, s);
    out.push(`<circle cx="${r2(x - rr * 0.1)}" cy="${r2(y - rr * 0.18)}" r="${r2(rr * 0.82)}" fill="${col(ramp(k, 2))}"/>`);
  }
  for (const c of centres) {
    const [x, y] = project(c, s);
    out.push(`<circle cx="${r2(x - rr * 0.15)}" cy="${r2(y - rr * 0.4)}" r="${r2(rr * 0.45)}" fill="${col(ramp(k, 1))}"/>`);
  }
  const ring = (rad, dz) => Array.from({ length: 96 }, (_, i) => { const t = (i / 96) * Math.PI * 2; return project(add(at, [Math.cos(t) * rad, Math.sin(t) * rad, dz]), s); });
  const silhouette = [];
  for (const c of centres) { const [x, y] = project(c, s); for (let j = 0; j < 24; j++) { const t = (j / 24) * Math.PI * 2; silhouette.push([x + Math.cos(t) * rr, y + Math.sin(t) * rr, 0]); } }
  const outline = hull2(silhouette);
  const st = `stroke="${col("ink")}" stroke-width="${STROKE}" stroke-linejoin="round" stroke-linecap="round"`;
  out.push(`<path d="M${outline.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}Z" fill="none" ${st}/>`);
  // The hole: the inner equator seen from above, nudged down by the tube's projected radius.
  const hole = ring(R - r, 0).map(([x, y]) => [x, y + rr * 0.35]);
  out.push(`<path d="M${hole.map(([x, y]) => `${r2(x)} ${r2(y)}`).join("L")}Z" fill="none" ${st}/>`);
  return out;
}

function hull2(p) {
  p = [...p].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cr = (o, a, b) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const lo = [], up = [];
  for (const q of p) { while (lo.length >= 2 && cr(lo[lo.length - 2], lo[lo.length - 1], q) <= 0) lo.pop(); lo.push(q); }
  for (const q of [...p].reverse()) { while (up.length >= 2 && cr(up[up.length - 2], up[up.length - 1], q) <= 0) up.pop(); up.push(q); }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
}
