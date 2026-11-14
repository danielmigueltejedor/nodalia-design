// Sheet metal: thin plates, bends, flanges and flat patterns.
import { sc, badge } from "../motifs.mjs";
import * as S from "../shapes.mjs";

const A = "accent";
const T = 2.4; // plate thickness

// An L profile in the y-z plane (a base plate and an upright flange joined by a bend).
const bentL = (len, up, r = 3) => mirror([
  [-len, 0], [0 - r - T, 0], ...S.arcPts(-r - T, r + T, r + T, -90, 0).slice(1, -1).map(([u, v]) => [u, v, "s"]), [0, r + T], [0, up],
  [-T, up], [-T, r + T], ...S.arcPts(-r - T, r + T, r, 0, -90).slice(1, -1).map(([u, v]) => [u, v, "s"]), [-r - T, T], [-len, T],
]);
// The flange stands at the back so the base plate faces the viewer.
const mirror = (pts) => pts.map(([u, v, f]) => (f ? [-u, v, f] : [-u, v]));

export default {
  "sheet": () => sc({ cy: 34 }).box([-15, -15, -T / 2], [15, 15, T / 2], { kind: A }),
  "sheet_rules": () => badge(sc({ cy: 32 }).box([-15, -15, -T / 2], [15, 15, T / 2], { kind: A }), "check"),
  "convert_sheet": () => {
    const s = sc({ cy: 34 }).box([-14, -14, -8], [14, 14, 6]);
    s.pocket([[-11, -11, 6], [11, -11, 6], [11, 11, 6], [-11, 11, 6]], 11, { kind: A });
    return badge(s, "arrow-right");
  },
  "flange": () => {
    const s = sc({ cy: 38 });
    s.box([-14, -2, -10], [14, 16, -10 + T]);
    return s.prism(bentL(4, 18), { axis: "x", from: -14, to: 14, at: [0, -6, -10], kind: A });
  },
  "path_flange": () => {
    const s = sc({ cy: 38 });
    s.box([-15, -15, -10], [15, 15, -10 + T]);
    s.box([-15, 15 - T, -10 + T], [15, 15, 6], { kind: A });
    s.box([15 - T, -15, -10 + T], [15, 15 - T, 6], { kind: A });
    return s;
  },
  "hem": () => {
    const s = sc({ cy: 34 });
    const prof = [[-16, 0], [6, 0], ...S.arcPts(6, 4, 4, -90, 90).slice(1, -1).map(([u, v]) => [u, v, "s"]), [6, 8], [-6, 8], [-6, 8 - T], [6, 8 - T], ...S.arcPts(6, 4, 4 - T, 90, -90).slice(1, -1).map(([u, v]) => [u, v, "s"]), [6, T], [-16, T]];
    return s.prism(prof, { axis: "x", from: -14, to: 14, at: [0, 2, -6], sideKind: (i) => (i >= 1 ? A : null) });
  },
  "fold": () => sc({ cy: 38 }).prism(bentL(14, 16, 2), { axis: "x", from: -14, to: 14, at: [0, -12, -10], kind: A }).line([[-14, -7, -10 + T], [14, -7, -10 + T]], { role: "accent-3", width: 1.6, dash: "3 2" }),
  "unfold": () => {
    const s = sc({ cy: 34 });
    s.line([[-15, -6, -4 + T], [-15, -6, 14], [15, -6, 14], [15, -6, -4 + T]], { role: "guide", width: 1.6, dash: "3 3", layer: "under" });
    s.box([-15, -6, -4], [15, 16, -4 + T]);
    s.box([-15, -22, -4], [15, -6, -4 + T], { kind: A });
    return s.line([[-15, -6, -4 + T], [15, -6, -4 + T]], { role: "accent-3", width: 1.6, dash: "3 2" });
  },
  "refold": () => {
    const s = sc({ cy: 38 });
    s.prism(bentL(14, 16, 2), { axis: "x", from: -14, to: 14, at: [0, -12, -10], kind: A });
    return badge(s, "undo");
  },
  "flat": () => {
    const s = sc({ cy: 34 });
    s.box([-15, -11, -T / 2], [15, 11, T / 2], { kind: A });
    s.plane([[-8, -11, T / 2], [8, -11, T / 2], [8, 11, T / 2], [-8, 11, T / 2]], { role: "face-1", stroke: null, layer: "over" });
    for (const x of [-8, 8]) s.line([[x, -11, T / 2], [x, 11, T / 2]], { role: "accent-3", width: 1.8, dash: "3 2.4" });
    return s;
  },
  "bendtable": () => {
    const s = sc({ cy: 34 }).box([-15, -15, -T / 2], [15, 15, T / 2]);
    for (const y of [-7, 0, 7]) s.line([[-15, y, T / 2], [15, y, T / 2]], { role: "ink", width: 1.4 });
    s.line([[-5, -15, T / 2], [-5, 15, T / 2]], { role: "ink", width: 1.4 });
    return s.plane([[-15, -15, T / 2], [15, -15, T / 2], [15, -7, T / 2], [-15, -7, T / 2]], { role: "accent-1", stroke: "ink", width: 1.4, layer: "over" });
  },
  "perforate": () => {
    const s = sc({ cy: 34 }).box([-15, -15, -T / 2], [15, 15, T / 2]);
    for (const x of [-8, 0, 8]) for (const y of [-8, 0, 8]) s.bore([x, y, T / 2], 2.6, T, { kind: A, through: true });
    return s;
  },
  "print_face": () => {
    const s = sc({ cy: 36 }).box([-14, -14, -10], [14, 14, 2]);
    s.plane([[-14, -14, 2], [14, -14, 2], [14, 14, 2], [-14, 14, 2]], { role: "accent-1", stroke: "ink", width: 2.4, layer: "over" });
    for (const y of [-6, 0, 6]) s.line([[-8, y, 2], [8, y, 2]], { role: "accent-3", width: 1.8 });
    return s;
  },
};
