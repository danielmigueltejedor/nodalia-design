// Surfaces and surface analysis: thin curved sheets.
import { sc, badge } from "../motifs.mjs";

const A = "accent";
const wave = (x, y) => 5.5 * Math.sin((x / 15) * Math.PI) + y * 0.08;
const saddle = (x, y) => (x * x) / 36 - 3 - y * 0.12;
const dome = (x, y) => 7 - (x * x) / 28 + y * 0.1;
const scoop = (x, y) => (y * y) / 30 - 4;

export default {
  "surface": () => sc({ cy: 34 }).sheet(wave, { kind: A, n: 14 }),
  "patch": () => {
    const s = sc({ cy: 34 });
    s.line([[-14, -14, 0], [14, -14, 0], [14, 14, 0], [-14, 14, 0]], { role: "line", width: 2.6, closed: true, layer: "under" });
    return s.sheet(dome, { kind: A, x0: -14, x1: 14, y0: -14, y1: 14, n: 12, edge: "ink" });
  },
  "ruled": () => {
    const s = sc({ cy: 34 }).sheet((x, y) => (y < 0 ? 4 * Math.sin(x / 5) : 0) * (-y / 14) + 6, { kind: A, y0: -14, y1: 14, n: 12, grid: 4 });
    return s;
  },
  "spun": () => sc({ cy: 34 }).turned({ at: [0, 0, -12], r0: 14, r1: 6, h0: 0, h1: 20, kind: A, hole: 0.01 }),
  "boundary": () => {
    const s = sc({ cy: 34 }).sheet(saddle, { kind: A, n: 12, edge: null });
    const rim = [];
    for (let i = 0; i <= 12; i++) rim.push([-14 + (28 * i) / 12, -14, saddle(-14 + (28 * i) / 12, -14)]);
    for (let i = 0; i <= 12; i++) rim.push([14, -14 + (28 * i) / 12, saddle(14, -14 + (28 * i) / 12)]);
    for (let i = 0; i <= 12; i++) rim.push([14 - (28 * i) / 12, 14, saddle(14 - (28 * i) / 12, 14)]);
    for (let i = 0; i <= 12; i++) rim.push([-14, 14 - (28 * i) / 12, saddle(-14, 14 - (28 * i) / 12)]);
    return s.line(rim, { role: "line", width: 3.4, closed: true });
  },
  "boundary_fill": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -10], [15, 15, -6]);
    s.box([-15, -15, -6], [-11, 15, 8]);
    s.box([-11, -15, -6], [15, -11, 8]);
    return s.sheet((x, y) => 2 - (x * y) / 60, { kind: A, x0: -11, x1: 15, y0: -11, y1: 15, n: 10 });
  },
  "offset_surface": () => {
    const s = sc({ cy: 36 }).sheet((x, y) => wave(x, y) - 6, { n: 12 });
    s.sheet((x, y) => wave(x, y) + 6, { kind: A, n: 12 });
    return s.arrow([0, 0, -4], [0, 0, 5], { w: 2.6, head: 5 });
  },
  "blend_surface": () => {
    const s = sc({ cy: 36 });
    s.sheet(() => -8, { x0: -15, x1: 15, y0: 3, y1: 15, n: 4 });
    s.sheet((x, y) => 8, { x0: -15, x1: 15, y0: -15, y1: -9, n: 4 });
    return s.sheet((x, y) => { const t = (y + 9) / 12; return 8 - 16 * (3 * t * t - 2 * t * t * t); }, { kind: A, x0: -15, x1: 15, y0: -9, y1: 3, n: 10, edge: null });
  },
  "stitch": () => {
    const s = sc({ cy: 34 });
    s.sheet((x, y) => wave(x, y), { x0: -15, x1: 0, n: 7 });
    s.sheet((x, y) => wave(x, y), { kind: A, x0: 0, x1: 15, n: 7 });
    const seam = Array.from({ length: 13 }, (_, i) => [0, -14 + (28 * i) / 12, wave(0, -14 + (28 * i) / 12)]);
    return s.line(seam, { role: "accent-3", width: 3.2 });
  },
  "unstitch": () => {
    const s = sc({ cy: 34 });
    s.sheet((x, y) => wave(x, y), { x0: -15, x1: -2, n: 7 });
    s.sheet((x, y) => wave(x, y), { kind: A, x0: 2, x1: 15, n: 7 });
    return s;
  },
  "untrim": () => {
    const s = sc({ cy: 34 }).sheet(wave, { kind: A, n: 12 });
    return s.arc([0, 0, 2], 6, { role: "accent-3", width: 2.2, dash: "3 3" });
  },
  "reverse_normal": () => {
    const s = sc({ cy: 38 }).sheet(wave, { n: 12 });
    s.arrow([0, 0, 1], [0, 0, 16], { role: "face-2" });
    return s.arrow([6, -6, 1], [6, -6, -14]);
  },
  "normal": () => sc({ cy: 38 }).sheet(wave, { n: 12 }).dot([0, 0, 0], { r: 2.6 }).arrow([0, 0, 0], [0, 0, 18]),
  "intersection_curve": () => {
    const s = sc({ cy: 34 }).sheet(wave, { n: 12 });
    s.plane([[-15, 0, -12], [15, 0, -12], [15, 0, 12], [-15, 0, 12]], { role: "face-1", stroke: "face-3", width: 1.6, layer: "over" });
    return s.line(Array.from({ length: 25 }, (_, i) => { const x = -14 + (28 * i) / 24; return [x, 0, wave(x, 0)]; }), { role: "accent-3", width: 3.2 });
  },
  "iso_curve": () => {
    const s = sc({ cy: 34 }).sheet(wave, { n: 12 });
    return s.line(Array.from({ length: 25 }, (_, i) => { const y = -14 + (28 * i) / 24; return [4, y, wave(4, y)]; }), { role: "accent-3", width: 3.2 });
  },
  "project_surface": () => {
    const s = sc({ cy: 36 }).sheet(wave, { n: 12 });
    const c = Array.from({ length: 33 }, (_, i) => { const t = (i / 32) * Math.PI * 2; return [Math.cos(t) * 7, Math.sin(t) * 7]; });
    s.line(c.map(([x, y]) => [x, y, 14]), { role: "guide", width: 1.8, closed: true, dash: "3 3" });
    s.line([[7, 0, 14], [7, 0, wave(7, 0)]], { role: "guide", width: 1.4, dash: "2 3" });
    return s.line(c.map(([x, y]) => [x, y, wave(x, y)]), { role: "accent-3", width: 3, closed: true });
  },
  "silhouette": () => {
    const s = sc({ cy: 32 }).sphere([0, 0, 0], 16);
    return s.path("M16 32A16 16 0 1 0 48 32A16 16 0 1 0 16 32Z", { fill: "none", stroke: "accent-3", width: 3.6 });
  },
  "curvature_comb": () => {
    const s = sc({ cy: 36 });
    const f = (x) => 6 * Math.sin((x / 16) * Math.PI);
    const pts = Array.from({ length: 17 }, (_, i) => { const x = -16 + 2 * i; return [x, -x * 0.2, f(x)]; });
    pts.forEach(([x, y, z], i) => { if (i % 2 === 0) s.line([[x, y, z], [x, y, z + 3 + 6 * Math.abs(Math.sin((x / 16) * Math.PI))]], { role: "accent-2", width: 1.6, layer: "under" }); });
    s.line(pts.map(([x, y, z], i) => [x, y, z + 3 + 6 * Math.abs(Math.sin((x / 16) * Math.PI))]), { role: "accent-3", width: 1.8, layer: "under" });
    return s.line(pts, { role: "line", width: 3 });
  },
  "curvature_map": () => sc({ cy: 34 }).sheet(saddle, { n: 12, paint: (x) => (Math.abs(x) > 10 ? "accent-3" : Math.abs(x) > 5 ? "accent-2" : "accent-1") }),
  "zebra": () => sc({ cy: 34 }).sheet(wave, { n: 16, paint: (x) => (Math.floor((x + 40) / 4.67) % 2 ? "ink" : "face-1") }),
  "draft_analysis": () => sc({ cy: 34 }).sheet(dome, { n: 14, paint: (x) => (Math.abs(x) < 5 ? "accent-1" : Math.abs(x) < 10 ? "accent-2" : "accent-3") }),
  "iso_analysis": () => {
    const s = sc({ cy: 34 }).sheet(wave, { n: 12, grid: 4 });
    return s;
  },
  "edge_continuity": () => {
    const s = sc({ cy: 34 });
    s.sheet((x, y) => wave(x, y), { x0: -15, x1: 0, n: 7 });
    s.sheet((x, y) => wave(x, y), { x0: 0, x1: 15, n: 7 });
    const seam = Array.from({ length: 13 }, (_, i) => [0, -14 + (28 * i) / 12, wave(0, -14 + (28 * i) / 12)]);
    s.line(seam, { role: "accent-3", width: 3.4 });
    return badge(s, "check");
  },
  "min_radius": () => {
    const s = sc({ cy: 34 }).sheet(scoop, { n: 12 });
    s.arc([0, 0, 4], 8, { axis: "x", a0: 200, a1: 340, role: "accent-3", width: 2.4 });
    s.line([[0, 0, 4], [0, -7.5, 1.3]], { role: "accent-3", width: 1.8 });
    return s.dot([0, 0, 4], { r: 2.4 });
  },
};
