// Sketch tools: geometry drawn on a sketch plane. The plane is the neutral face, the geometry the
// tool creates is the accent, existing geometry is ink and construction references are dashed.
import { Scene } from "../scene.mjs";
import { badge } from "../motifs.mjs";

const H = 17;
const W = 2.8;
const P = (pts) => pts.map(([x, y]) => [x, y, 0]);

// A fresh sketch: the plane, and helpers that draw on it in plane coordinates.
export const SKETCH_VIEW = { az: 22, el: 58, k: 1.02 };

export function sk({ plane = true, cy = 30 } = {}) {
  const s = new Scene({ cy, ...SKETCH_VIEW });
  if (plane) s.box([-H, -H, -2.6], [H, H, 0]);
  // Everything drawn on the board is attached to it, so a part floating above still covers it.
  const on = plane ? 0 : "over";
  const g = {
    s,
    // New geometry.
    line: (pts, o = {}) => (s.line(P(pts), { role: "accent-3", width: W, layer: on, ...o }), g),
    loop: (pts, o = {}) => (s.line(P(pts), { role: "accent-3", width: W, closed: true, layer: on, ...o }), g),
    // Existing geometry the tool works from.
    ref: (pts, o = {}) => (s.line(P(pts), { role: "ink", width: 2.2, layer: on, ...o }), g),
    guide: (pts, o = {}) => (s.line(P(pts), { role: "guide", width: 1.7, dash: "3 3", layer: on, ...o }), g),
    circle: (cx, cy, r, o = {}) => (s.arc([cx, cy, 0], r, { role: "accent-3", width: W, layer: on, ...o }), g),
    arc: (cx, cy, r, a0, a1, o = {}) => (s.arc([cx, cy, 0], r, { a0, a1, role: "accent-3", width: W, layer: on, ...o }), g),
    refCircle: (cx, cy, r, o = {}) => (s.arc([cx, cy, 0], r, { role: "ink", width: 2.2, layer: on, ...o }), g),
    guideCircle: (cx, cy, r, o = {}) => (s.arc([cx, cy, 0], r, { role: "guide", width: 1.7, dash: "3 3", layer: on, ...o }), g),
    pt: (x, y, o = {}) => (s.dot([x, y, 0], { r: 2.7, role: "accent-3", layer: on, ...o }), g),
    refPt: (x, y, o = {}) => (s.dot([x, y, 0], { r: 2.4, role: "paper", ring: "ink", layer: on, ...o }), g),
    fill: (pts, o = {}) => (s.plane(P(pts), { role: "accent-1", stroke: null, layer: on, ...o }), g),
    badge: (glyph, o) => (badge(s, glyph, o), g),
  };
  return g;
}

export const arcPts = (cx, cy, r, a0, a1, n = 24) => Array.from({ length: n + 1 }, (_, i) => { const t = ((a0 + ((a1 - a0) * i) / n) * Math.PI) / 180; return [cx + Math.cos(t) * r, cy + Math.sin(t) * r]; });
export const ngon = (cx, cy, r, n, rot = 0) => Array.from({ length: n }, (_, i) => { const t = ((rot + (360 * i) / n) * Math.PI) / 180; return [cx + Math.cos(t) * r, cy + Math.sin(t) * r]; });
const slotPts = (x0, y0, x1, y1, r) => {
  const a = (Math.atan2(y1 - y0, x1 - x0) * 180) / Math.PI;
  return [...arcPts(x1, y1, r, a - 90, a + 90, 16), ...arcPts(x0, y0, r, a + 90, a + 270, 16)];
};
export const bez = (p0, p1, p2, p3, n = 28) => Array.from({ length: n + 1 }, (_, i) => {
  const t = i / n, u = 1 - t;
  return [0, 1].map((k) => u * u * u * p0[k] + 3 * u * u * t * p1[k] + 3 * u * t * t * p2[k] + t * t * t * p3[k]);
});

export default {
  // Each entry builds a sketch on a board seen from above.
  "sketch": () => {
    const g = sk();
    g.loop([[-10, -10], [8, -10], [8, 2]]).circle(2, 6, 6);
    return g.s;
  },
  "canvas": () => {
    const s = new Scene({ cy: 32 });
    s.plane([[-15, -15, 0], [15, -15, 0], [15, 15, 0], [-15, 15, 0]], { role: "accent-1", stroke: "ink", width: 2.2 });
    s.line([[-10, 6, 0], [-3, -2, 0], [3, 3, 0], [10, -6, 0]], { role: "accent-3", width: 2.4 });
    s.dot([5, -8, 0], { r: 2.6, role: "accent-2" });
    return s;
  },
  "finish": () => sk().loop([[-10, -10], [10, -10], [10, 10], [-10, 10]]).badge("check").s,
  "finish_edit": () => sk().loop([[-10, -10], [10, -10], [10, 10], [-10, 10]]).badge("arrow-right").s,
  "line": () => sk().line([[-12, 10], [10, -10]]).pt(-12, 10).pt(10, -10).s,
  "line_mid": () => sk().line([[-12, 10], [10, -10]]).pt(-1, 0, { r: 3.2 }).refPt(-12, 10).refPt(10, -10).s,
  "chain": () => sk().line([[-13, 6], [-4, -8], [4, 8], [13, -6]]).pt(-13, 6).pt(-4, -8).pt(4, 8).pt(13, -6).s,
  "centerline": () => sk().line([[-14, 12], [14, -12]], { dash: "7 3 2 3" }).s,
  "infinite": () => sk().guide([[-19, 15], [-12, 9]]).line([[-12, 9], [12, -9]]).guide([[12, -9], [19, -15]]).pt(-12, 9).pt(12, -9).s,
  "point": () => sk().pt(0, 0, { r: 3.6 }).guide([[-6, 0], [6, 0]]).guide([[0, -6], [0, 6]]).s,
  "dots": () => { const g = sk(); for (const x of [-8, 0, 8]) for (const y of [-8, 0, 8]) g.pt(x, y, { r: 2.2 }); return g.s; },
  "snap": () => sk().ref([[-14, 10], [14, -10]]).refCircle(-4, -4, 6, { a0: 0, a1: 360 }).pt(0, 0, { r: 3.4 }).s,
  "construction": () => sk().line([[-13, 11], [13, -11]], { dash: "4 3" }).refCircle(0, 0, 7, { dash: "4 3", role: "accent-3" }).s,
  "construction_line": () => sk().line([[-18, 14], [18, -14]], { dash: "5 3" }).pt(0, 0).s,
  "construction_line_horizontal": () => sk().line([[-18, 0], [18, 0]], { dash: "5 3" }).pt(0, 0).s,
  "construction_line_vertical": () => sk().line([[0, -18], [0, 18]], { dash: "5 3" }).pt(0, 0).s,
  "construction_line_angle": () => sk().ref([[-14, 8], [14, 8]]).line([[-12, 16], [12, -16]], { dash: "5 3" }).arc(3, 8, 7, 230, 300, { role: "ink", width: 1.8 }).s,
  "construction_line_parallel": () => sk().ref([[-16, 4], [4, -16]]).line([[-4, 16], [16, -4]], { dash: "5 3" }).s,
  "construction_line_perpendicular": () => sk().ref([[-16, 6], [16, 6]]).line([[0, 18], [0, -18]], { dash: "5 3" }).ref([[0, 2], [4, 2], [4, 6]], { width: 1.6 }).s,
  "construction_line_bisector": () => sk().ref([[-14, 12], [8, -6]]).ref([[-14, 12], [14, 8]]).line([[-14, 12], [18, -2]], { dash: "5 3" }).s,
  "construction_line_tangent_at": () => sk().refCircle(-2, 2, 9).line([[-16, -7.4], [12.6, 14.4]], { dash: "5 3" }).pt(3.8, 7.8 - 3.2).s,
  "construction_line_tangent_external": () => sk().refCircle(-7, 5, 6).refCircle(8, -6, 5).line([[-17, 0], [15, -12]], { dash: "5 3" }).s,
  "construction_line_tangent_two": () => sk().refCircle(-7, 5, 6).refCircle(8, -6, 5).line([[-14, 0], [16, 10]], { dash: "5 3" }).s,
  "circle": () => sk().circle(0, 0, 11).pt(0, 0).s,
  "circle2": () => sk().circle(0, 0, 11).pt(-11, 0).pt(11, 0).guide([[-11, 0], [11, 0]]).s,
  "circle3": () => { const g = sk().circle(0, 0, 11); for (const a of [100, 220, 340]) g.pt(Math.cos((a * Math.PI) / 180) * 11, Math.sin((a * Math.PI) / 180) * 11); return g.s; },
  "circle_tt": () => sk().ref([[-16, -12], [-16, 14]]).ref([[-16, 14], [14, 14]]).circle(-7, 5, 9).s,
  "circle_ttt": () => sk().ref([[-16, -14], [-16, 14]]).ref([[-16, 14], [14, 14]]).ref([[-16, -14], [14, 14]]).circle(-7.2, 5.2, 7.6).s,
  "ellipse": () => sk().line(Array.from({ length: 49 }, (_, i) => { const t = (i / 48) * Math.PI * 2; return [Math.cos(t) * 14 * 0.7 + Math.sin(t) * 6 * 0.7, -Math.cos(t) * 14 * 0.7 + Math.sin(t) * 6 * 0.7]; })).pt(0, 0).s,
  "conic": () => sk().ref([[-14, 10], [4, -14]], { width: 1.6, dash: "3 3" }).ref([[4, -14], [14, 12]], { width: 1.6, dash: "3 3" }).line(bez([-14, 10], [-2, -6], [6, -6], [14, 12])).pt(-14, 10).pt(14, 12).s,
  "arc3": () => sk().arc(0, 4, 13, 200, 340).pt(-12.2, -0.4).pt(0, -9).pt(12.2, -0.4).s,
  "arc_center": () => sk().arc(0, 0, 12, 180, 340).guide([[0, 0], [-12, 0]]).guide([[0, 0], [11.3, -4.1]]).pt(0, 0).s,
  "arc_tangent": () => sk().ref([[-16, 8], [-2, 8]]).arc(-2, -3, 11, 90, -60).pt(-2, 8).s,
  "arc_slot": () => sk().loop([...arcPts(0, 10, 16, 210, 330), ...arcPts(10.39, 4, 4, 330, 510, 10), ...arcPts(0, 10, 8, 330, 210), ...arcPts(-10.39, 4, 4, 30, 210, 10)]).s,
  "arc_slot_center": () => sk().loop([...arcPts(0, 10, 16, 210, 330), ...arcPts(10.39, 4, 4, 330, 510, 10), ...arcPts(0, 10, 8, 330, 210), ...arcPts(-10.39, 4, 4, 30, 210, 10)]).pt(0, 10).guide([[0, 10], [-10.39, 4]]).s,
  "slot": () => sk().loop(slotPts(-8, 6, 8, -6, 5.5)).s,
  "slot_center": () => sk().loop(slotPts(-8, 6, 8, -6, 5.5)).guide([[-8, 6], [8, -6]]).pt(-8, 6).pt(8, -6).s,
  "slot_overall": () => sk().loop(slotPts(-8, 6, 8, -6, 5.5)).ref([[-14.4, 10.8], [14.4, -10.8]], { width: 1.6 }).pt(-12.4, 9.3).pt(12.4, -9.3).s,
  "rect": () => sk().loop([[-12, -9], [12, -9], [12, 9], [-12, 9]]).pt(-12, 9).pt(12, -9).s,
  "rect_center": () => sk().loop([[-12, -9], [12, -9], [12, 9], [-12, 9]]).pt(0, 0).guide([[0, 0], [12, -9]]).s,
  "rect3": () => sk().loop([[-14, 2], [0, -12], [12, 0], [-2, 14]]).pt(-14, 2).pt(0, -12).pt(12, 0).s,
  "rect_rounded": () => sk().loop([...arcPts(8, -5, 4, -90, 0, 8), ...arcPts(8, 5, 4, 0, 90, 8), ...arcPts(-8, 5, 4, 90, 180, 8), ...arcPts(-8, -5, 4, 180, 270, 8)]).s,
  "polygon": () => sk().loop(ngon(0, 0, 12, 6)).pt(0, 0).s,
  "polygon_circumscribed": () => sk().guideCircle(0, 0, 10.4).loop(ngon(0, 0, 12, 6, 30)).pt(0, 0).s,
  "polygon_edge": () => sk().loop(ngon(0, 0, 12, 6)).ref([[6, 10.4], [-6, 10.4]], { width: 3 }).pt(6, 10.4).pt(-6, 10.4).s,
  "spline": () => sk().line(bez([-14, 8], [-6, -16], [4, 16], [14, -8])).pt(-14, 8).pt(14, -8).pt(0, 0).s,
  "spline_cv": () => sk().guide([[-14, 8], [-6, -14], [4, 14], [14, -8]]).line(bez([-14, 8], [-6, -14], [4, 14], [14, -8])).refPt(-6, -14).refPt(4, 14).pt(-14, 8).pt(14, -8).s,
  "blend": () => sk().ref([[-16, 10], [-6, 10]]).ref([[6, -10], [6, -16]]).line(bez([-6, 10], [2, 10], [6, 0], [6, -10])).s,
  "equation_curve": () => sk().guide([[-16, 0], [16, 0]]).line(Array.from({ length: 41 }, (_, i) => { const x = -15 + (30 * i) / 40; return [x, -8 * Math.sin((x / 15) * Math.PI * 1.5)]; })).s,
  "fit_section": () => sk().line(bez([-14, 6], [-6, -12], [6, 12], [14, -6])).pt(-12, 0.5, { role: "paper", ring: "ink" }).pt(-5, -3.2, { role: "paper", ring: "ink" }).pt(3, 2.6, { role: "paper", ring: "ink" }).pt(11, -3, { role: "paper", ring: "ink" }).s,
  "offset": () => sk().ref([[-14, 4], [-2, -8], [12, -8]]).line([[-14, 12], [1, -3], [12, -3]]).s,
  "offset_symmetric": () => sk().ref([[-14, 0], [14, 0]]).line([[-14, 6], [14, 6]]).line([[-14, -6], [14, -6]]).s,
  "offset_two_sides": () => sk().ref([[-14, 0], [14, 0]]).line([[-14, 8], [14, 8]]).line([[-14, -4], [14, -4]]).s,
  "sketch_fillet": () => sk().ref([[-14, 12], [-14, -2]]).ref([[0, -14], [14, -14]]).line(arcPts(0, -2, 14, 180, 270, 16)).s,
  "sketch_chamfer": () => sk().ref([[-14, 12], [-14, -4]]).ref([[2, -14], [14, -14]]).line([[-14, -4], [2, -14]]).s,
  "sketch_chamfer_angle": () => sk().ref([[-14, 12], [-14, -4]]).ref([[2, -14], [14, -14]]).line([[-14, -4], [2, -14]]).arc(2, -14, 6, 150, 180, { role: "ink", width: 1.8 }).s,
  "sketch_chamfer_two_distances": () => sk().ref([[-14, 12], [-14, -4]]).ref([[2, -14], [14, -14]]).line([[-14, -4], [2, -14]]).pt(-14, -4).pt(2, -14).s,
  "sketch_mirror": () => sk().guide([[-16, 16], [16, -16]], { dash: "6 3 2 3" }).ref([[-14, -2], [-6, -12], [-2, -4]]).line([[2, 14], [12, 6], [4, 2]]).s,
  "trim": () => sk().ref([[-14, -14], [14, 14]]).line([[-14, 6], [0, 0]]).guide([[0, 0], [14, -6]]).badge("cross", { cx: 50, cy: 48, r: 8 }).s,
  "extend": () => sk().ref([[8, -16], [8, 16]]).line([[-14, 4], [-2, 2]]).line([[-2, 2], [8, 0.4]], { dash: "4 3" }).pt(8, 0.4).s,
  "break": () => sk().line([[-14, 8], [-3, 2]]).line([[3, -2], [14, -8]]).pt(-3, 2).pt(3, -2).s,
  "intersect": () => sk().ref([[-14, -12], [14, 12]]).ref([[-14, 12], [14, -12]]).pt(0, 0, { r: 3.6 }).s,
  "project": () => {
    const g = sk();
    g.s.box([-6, -6, 10], [6, 6, 16], { kind: "face" });
    g.loop([[-6, -6], [6, -6], [6, 6], [-6, 6]]);
    g.s.line([[6, 6, 10], [6, 6, 0]], { role: "guide", width: 1.6, dash: "2 3", layer: 0 });
    g.s.line([[-6, 6, 10], [-6, 6, 0]], { role: "guide", width: 1.6, dash: "2 3", layer: 0 });
    g.s.line([[6, -6, 10], [6, -6, 0]], { role: "guide", width: 1.6, dash: "2 3", layer: 0 });
    return g.s;
  },
  "include": () => {
    const g = sk();
    g.s.box([-12, -12, -10], [12, 12, -0.5]);
    g.loop([[-12, -12], [12, -12], [12, 12], [-12, 12]]);
    return g.s;
  },
  "trace": () => sk().fill([[-12, -10], [10, -12], [12, 10], [-10, 12]], { role: "face-2" }).line([[-12, -10], [10, -12], [12, 10], [-10, 12]], { closed: true }).s,
  "text": () => sk().line([[-12, -12], [12, -12]]).line([[0, -12], [0, 14]]).s,
  "svg": () => sk().line(bez([-14, 10], [-14, -14], [14, 14], [14, -10])).guide([[-14, 10], [-14, -14]]).guide([[14, -10], [14, 14]]).refPt(-14, -14).refPt(14, 14).pt(-14, 10).pt(14, -10).s,
  "dxf": () => sk().ref([[-12, -12], [12, -12], [12, 12], [-12, 12]], { closed: true }).line([[-8, 6], [-2, -6], [8, -2]]).circle(4, 6, 3).s,
  "dimension": () => sk().ref([[-12, 6], [12, -6]]).line([[-15, 0], [9, -12]], { width: 1.8 }).line([[-12, 6], [-15, 0]], { width: 1.4 }).line([[12, -6], [9, -12]], { width: 1.4 }).pt(-15, 0, { r: 2 }).pt(9, -12, { r: 2 }).s,
  "constrainer": () => sk().ref([[-14, 8], [6, 8]]).ref([[6, 8], [6, -14]]).ref([[2, 8], [2, 4], [6, 4]], { width: 1.6 }).badge("check").s,
  "auto_constrain": () => sk().ref([[-14, 8], [6, 8]]).ref([[6, 8], [6, -14]]).ref([[2, 8], [2, 4], [6, 4]], { width: 1.6 }).badge("refresh").s,
};
