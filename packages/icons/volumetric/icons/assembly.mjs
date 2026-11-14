// Assemblies, joints and motion, and inspection.
import { sc, badge } from "../motifs.mjs";
import * as S from "../shapes.mjs";

const A = "accent";

// Two parts meeting: a lower block and an upper part.
const pair = (s, upper = A) => {
  s.box([-15, -15, -12], [15, 15, -4]);
  s.box([-8, -8, -4], [8, 8, 8], { kind: upper });
  return s;
};

export default {
  "component": () => sc({ cy: 34 }).box([-13, -13, -11], [13, 13, 1]).box([-7, -7, 1], [7, 7, 11], { kind: A }),
  "joint": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -2, -10], [15, 14, -4]);
    s.prism(S.circle(0, 0, 4), { axis: "x", from: -15, to: 15, at: [0, -2, -4], kind: A });
    return s.box([-15, -16, -6], [15, -3, -2], { kind: A });
  },
  "joint_origin": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -10], [15, 15, -4]);
    s.arrow([0, 0, -4], [16, 0, -4], { role: "accent-2", w: 2.6, head: 6 }).arrow([0, 0, -4], [0, 16, -4], { role: "face-2", w: 2.6, head: 6 }).arrow([0, 0, -4], [0, 0, 14], { role: "accent-3", w: 2.6, head: 6 });
    return s.dot([0, 0, -4], { r: 3.4, role: "paper" });
  },
  "rigid_group": () => {
    const s = sc({ cy: 36 });
    s.box([-14, -2, -10], [0, 14, 4]);
    s.box([0, -14, -10], [14, 2, 4]);
    s.line([[-7, 6, 4], [7, -6, 4]], { role: "accent-3", width: 3.4 });
    return s.dot([-7, 6, 4], { r: 3 }).dot([7, -6, 4], { r: 3 });
  },
  "motion_link": () => {
    const s = sc({ cy: 34 });
    s.prism(S.gear(0, 0, 9, 13, 9), { from: -3, to: 3, at: [-6, 6, 0] });
    s.prism(S.gear(0, 0, 6, 9, 7), { from: -3, to: 3, at: [11, -9, 0], kind: A });
    return s;
  },
  "drive": () => {
    const s = sc({ cy: 36 }).cylinder([0, 0, -12], 12, 8, { kind: A });
    s.line([[0, 0, -4], [0, 0, 8]], { role: "ink", width: 3 });
    return s.arcArrow([0, 0, 6], 12, 160, 400);
  },
  "rest": () => {
    const s = sc({ cy: 36 });
    s.box([-16, -16, -12], [16, 16, -6]);
    s.box([-8, -8, -6], [8, 8, 6], { kind: A });
    return badge(s, "pause");
  },
  "reach": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -12], [15, 15, -6]);
    s.arc([0, 0, -6], 13, { role: "accent-3", width: 2, dash: "4 3" });
    s.line([[0, 0, -6], [9, -9, -6]], { role: "accent-3", width: 2.4 });
    return s.cylinder([0, 0, -6], 3.5, 12, { kind: A });
  },
  "as_built": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -12], [15, 15, -4]);
    s.box([-8, -8, -4], [8, 8, 8], { kind: A });
    return badge(s, "check");
  },
  "contact_sets": () => {
    const s = sc({ cy: 37 });
    s.box([-15, -15, -14], [15, 15, -6]);
    s.plane([[-8, -8, -6], [8, -8, -6], [8, 8, -6], [-8, 8, -6]], { role: "accent-2", stroke: "accent-3", width: 2, layer: 0 });
    return s.box([-8, -8, 0], [8, 8, 10], { kind: A });
  },
  "contact_new": () => badge(pair(sc({ cy: 34 }), "face"), "plus"),
  "contact_all": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -12], [15, 15, -6]);
    for (const [x, y] of [[-8, -8], [8, -8], [-8, 8], [8, 8]]) s.box([x - 4, y - 4, -6], [x + 4, y + 4, 2], { kind: A });
    return s;
  },
  "contact_off": () => badge(pair(sc({ cy: 34 }), "face"), "cross"),
  "interference": () => {
    const s = sc({ cy: 34 });
    s.box([-15, -4, -10], [6, 12, 2]);
    s.box([-4, -14, -6], [14, 4, 8]);
    return s.plane([[-4, -4, 2], [6, -4, 2], [6, 4, 2], [-4, 4, 2]], { role: "accent-2", stroke: "accent-3", layer: "over" });
  },
  "align": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -12], [15, 0, -2]);
    s.box([-15, 2, -12], [15, 15, 4], { kind: A });
    return s.line([[-17, 0, -2], [17, 0, -2]], { role: "accent-3", width: 2, dash: "4 3" });
  },
  "measure": () => {
    const s = sc({ cy: 36 }).box([-14, -14, -10], [14, 14, 0]);
    s.line([[-14, 18, 0], [14, 18, 0]], { role: "accent-3", width: 2.2 });
    s.line([[-14, 14, 0], [-14, 20, 0]], { role: "accent-3", width: 1.6 });
    s.line([[14, 14, 0], [14, 20, 0]], { role: "accent-3", width: 1.6 });
    return s.dot([-14, 18, 0], { r: 2.2 }).dot([14, 18, 0], { r: 2.2 });
  },
  "section": () => {
    const s = sc({ cy: 36 });
    s.box([-14, -14, -10], [14, 0, 6]);
    s.pocket([[-10, -10, 6], [10, -10, 6], [10, 0, 6], [-10, 0, 6]], 12);
    return s.plane([[-14, 0, -10], [14, 0, -10], [14, 0, 6], [-14, 0, 6]], { role: "accent-1", stroke: "accent-3", width: 2.4, layer: "over" })
      .plane([[-10, 0, -6], [10, 0, -6], [10, 0, 6], [-10, 0, 6]], { role: "paper", stroke: "accent-3", width: 1.6, layer: "over" });
  },
  "inspect": () => {
    const s = sc({ cy: 34 }).box([-14, -14, -10], [14, 14, 4]);
    s.path("M34 30A9 9 0 1 0 52 30A9 9 0 1 0 34 30Z", { fill: "accent-1", stroke: "ink", width: 2.6 });
    return s.path("M49.5 37.5L57 45", { fill: "none", stroke: "ink", width: 4.4 });
  },
  "compare": () => {
    const s = sc({ cy: 34 });
    s.box([-14, -2, -10], [0, 12, 4]);
    s.box([0, -14, -10], [14, 0, 4], { kind: A });
    return s.line([[0, 14, -14], [0, -16, -14], [0, -16, 10], [0, 14, 10]], { role: "guide", width: 1.6, dash: "4 3", closed: true, layer: "under" });
  },
  "graph": () => {
    const s = sc({ cy: 34 }).box([-15, -15, -3], [15, 15, 0]);
    for (const [x, h] of [[-9, 6], [0, 13], [9, 9]]) s.box([x - 3, -3, 0], [x + 3, 3, h], { kind: A });
    return s;
  },
};
