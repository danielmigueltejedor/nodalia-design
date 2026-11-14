// Construction geometry: planes, axes and points, and what each is built from.
import { sc } from "../motifs.mjs";

// A construction plane: four corners, drawn as a pale accent sheet with an accent rim.
const plane = (s, pts, { kind = "accent", layer = "over" } = {}) =>
  s.plane(pts, kind === "accent" ? { role: "accent-1", stroke: "accent-3", width: 2.2, layer } : { role: "face-1", stroke: "face-3", width: 2, layer });
const hplane = (z, h = 16, dx = 0, dy = 0) => [[-h + dx, -h + dy, z], [h + dx, -h + dy, z], [h + dx, h + dy, z], [-h + dx, h + dy, z]];

export default {
  "plane": () => plane(sc({ cy: 32 }), hplane(0, 15)),
  "origin": () => {
    const s = sc({ cy: 34 });
    s.arrow([0, 0, 0], [20, 0, 0], { role: "accent-2" }).arrow([0, 0, 0], [0, 20, 0], { role: "face-2" }).arrow([0, 0, 0], [0, 0, 20], { role: "accent-3" });
    return s.dot([0, 0, 0], { r: 3.4, role: "paper" });
  },
  "plane_midplane": () => {
    const s = sc({ cy: 34 });
    plane(s, hplane(-10, 14), { kind: "face" });
    plane(s, hplane(0, 14));
    return plane(s, hplane(10, 14), { kind: "face" });
  },
  "plane_angle": () => {
    const s = sc({ cy: 36 });
    plane(s, hplane(-6, 15), { kind: "face" });
    plane(s, [[-15, -15, -6], [-15, 15, -6], [0, 15, 14], [0, -15, 14]]);
    return s.arcArrow([-15, 15, -6], 9, 0, 55, { axis: "y" });
  },
  "plane_perpendicular": () => {
    const s = sc({ cy: 34 });
    plane(s, hplane(-8, 15), { kind: "face" });
    return plane(s, [[0, -15, -8], [0, 15, -8], [0, 15, 14], [0, -15, 14]]);
  },
  "plane_tangent": () => {
    const s = sc({ cy: 36 }).cylinder([0, 0, -12], 10, 18);
    return plane(s, [[10, -14, -14], [10, 14, -14], [10, 14, 10], [10, -14, 10]]);
  },
  "plane_three_points": () => {
    const s = sc({ cy: 32 });
    const z = (y) => -0.45 * y;
    plane(s, [[-15, -15, z(-15)], [15, -15, z(-15)], [15, 15, z(15)], [-15, 15, z(15)]]);
    for (const [x, y] of [[-8, -6], [9, -2], [-1, 9]]) s.dot([x, y, z(y)], { r: 3.2 });
    return s;
  },
  "plane_two_edges": () => {
    const s = sc({ cy: 32 });
    const z = (y) => -0.45 * y;
    plane(s, [[-15, -15, z(-15)], [15, -15, z(-15)], [15, 15, z(15)], [-15, 15, z(15)]]);
    s.line([[-17, -15, z(-15)], [17, -15, z(-15)]], { role: "line", width: 3.4 });
    return s.line([[-17, 15, z(15)], [17, 15, z(15)]], { role: "line", width: 3.4 });
  },
  "plane_axis_point": () => {
    const s = sc({ cy: 34 });
    plane(s, [[0, -14, -12], [0, 14, -12], [0, 14, 12], [0, -14, 12]]);
    s.line([[0, -18, -12], [0, 18, -12]], { role: "line", width: 3 });
    return s.dot([0, 4, 8], { r: 3 });
  },
  "plane_along_path": () => {
    const s = sc({ cy: 34 });
    const path = [];
    for (let i = 0; i <= 30; i++) { const t = i / 30; path.push([-16 + 32 * t, 10 - 20 * t * t, -6]); }
    s.line(path, { role: "line", width: 2.6, layer: "under" });
    return plane(s, [[0, 10, -16], [0, 0, -16], [0, 0, 6], [0, 10, 6]].map(([x, y, z]) => [x + 2, y - 4, z + 6]));
  },
  "axis": () => {
    const s = sc({ cy: 32 });
    s.arc([0, 0, -12], 7, { role: "guide", width: 1.6, dash: "3 3" });
    s.line([[0, 0, -20], [0, 0, 20]], { role: "accent-3", width: 3.2, dash: "8 3 2 3" });
    return s.arcArrow([0, 0, 10], 7, 200, 420);
  },
  "axis_edge": () => {
    const s = sc({ cy: 36 }).box([-12, -12, -10], [12, 12, 4]);
    return s.line([[12, 12, -18], [12, 12, 14]], { role: "accent-3", width: 3, dash: "8 3 2 3" });
  },
  "axis_normal_to_face": () => {
    const s = sc({ cy: 38 }).box([-14, -14, -10], [14, 14, -2]);
    s.plane(hplane(-2, 14), { role: "accent-1", stroke: "ink", width: 2.4 });
    s.line([[0, 0, -2], [0, 0, 22]], { role: "accent-3", width: 3, dash: "8 3 2 3" });
    return s.dot([0, 0, -2], { r: 3 });
  },
  "axis_through_cylinder": () => {
    const s = sc({ cy: 34 }).cylinder([0, 0, -10], 10, 16);
    return s.line([[0, 0, -18], [0, 0, 18]], { role: "accent-3", width: 3, dash: "8 3 2 3" });
  },
  "axis_two_planes": () => {
    const s = sc({ cy: 34 });
    plane(s, hplane(0, 14), { kind: "face" });
    plane(s, [[0, -14, -12], [0, 14, -12], [0, 14, 12], [0, -14, 12]], { kind: "face" });
    return s.line([[0, -20, 0], [0, 20, 0]], { role: "accent-3", width: 3, dash: "8 3 2 3" });
  },
  "axis_two_points": () => {
    const s = sc({ cy: 34 });
    s.line([[-14, 14, -10], [14, -14, 12]], { role: "accent-3", width: 3, dash: "8 3 2 3" });
    return s.dot([-8, 8, -5.3], { r: 3.2, role: "paper", ring: "ink" }).dot([8, -8, 7.3], { r: 3.2, role: "paper", ring: "ink" });
  },
  "point_vertex": () => sc({ cy: 36 }).box([-12, -12, -10], [12, 12, 4]).dot([12, 12, 4], { r: 4 }),
  "point_center": () => sc({ cy: 36 }).cylinder([0, 0, -10], 12, 12).dot([0, 0, 2], { r: 4 }),
  "point_edge_plane": () => {
    const s = sc({ cy: 34 });
    plane(s, [[0, -14, -12], [0, 14, -12], [0, 14, 12], [0, -14, 12]], { kind: "face" });
    s.line([[-16, 4, 2], [16, 4, 2]], { role: "line", width: 3 });
    return s.dot([0, 4, 2], { r: 4 });
  },
  "point_two_edges": () => sc({ cy: 34 }).line([[-16, 0, 0], [16, 0, 0]], { role: "line", width: 3 }).line([[0, -16, 0], [0, 16, 0]], { role: "line", width: 3 }).dot([0, 0, 0], { r: 4 }),
  "point_three_planes": () => {
    const s = sc({ cy: 34 });
    plane(s, hplane(0, 13), { kind: "face" });
    plane(s, [[0, -13, -13], [0, 13, -13], [0, 13, 13], [0, -13, 13]], { kind: "face" });
    plane(s, [[-13, 0, -13], [13, 0, -13], [13, 0, 13], [-13, 0, 13]], { kind: "face" });
    return s.dot([0, 0, 0], { r: 4 });
  },
  "point_along_path": () => {
    const s = sc({ cy: 34 });
    const path = [];
    for (let i = 0; i <= 30; i++) { const t = i / 30; path.push([-16 + 32 * t, 12 - 24 * t, 10 * Math.sin(t * Math.PI) - 4]); }
    s.line(path, { role: "line", width: 2.6 });
    return s.dot(path[18], { r: 4 });
  },
};
