// Solid modelling features.
import { sc, block, sketchPlane, badge } from "../motifs.mjs";
import * as S from "../shapes.mjs";

const A = "accent";

export default {
  "box": () => sc().box([-13, -13, -9], [13, 13, 9], { kind: A }),
  "cylinder": () => sc().cylinder([0, 0, -11], 13, 20, { kind: A }),
  "cone": () => sc({ cy: 36 }).turned({ at: [0, 0, -12], r0: 15, r1: 0, h0: 0, h1: 26, kind: A }),
  "sphere": () => sc({ cy: 32 }).sphere([0, 0, 0], 20, { kind: A }),
  "torus": () => sc({ cy: 32 }).torus([0, 0, 0], 13, 7, { kind: A }),
  "body": () => block(sc()).cylinder([4, -4, 4], 6, 7),

  "extrude": () => sc({ cy: 38 })
    .box([-14, -14, -9], [14, 14, -2])
    .box([-14, -14, -2], [14, 14, 6], { kind: A })
    .arrow([0, 0, 8], [0, 0, 26]),
  "cut": () => sc({ cy: 36 })
    .box([-15, -15, -9], [15, 15, 5])
    .pocket([[-8, -8, 5], [8, -8, 5], [8, 8, 5], [-8, 8, 5]], 8, { kind: A })
    .arrow([0, 0, 22], [0, 0, 9]),
  "boss": () => sc({ cy: 38 })
    .box([-15, -15, -9], [15, 15, -2])
    .cylinder([0, 0, -2], 8, 12, { kind: A }),
  "presspull": () => sc({ cy: 38 })
    .box([-14, -14, -9], [14, 14, 0])
    .box([-14, -14, 0], [14, 14, 4], { kind: A })
    .arrow([0, 0, 18], [0, 0, 7])
    .arrow([0, 0, 10], [0, 0, 26]),
  "revolve": () => {
    const s = sc({ cy: 37 });
    s.turned({ at: [0, 0, -12], r0: 15, r1: 15, h0: 0, h1: 7, kind: A });
    s.turned({ at: [0, 0, -5], r0: 8, r1: 8, h0: 0, h1: 9, kind: A });
    s.line([[0, 0, 4], [0, 0, 20]], { role: "guide", width: 1.8, dash: "3 3" });
    return s.arcArrow([0, 0, 15], 9, 160, 380);
  },
  "sweep": () => {
    const s = sc({ cy: 34 });
    const path = [];
    for (let i = 0; i <= 40; i++) { const t = i / 40; path.push([-14 + 28 * t, 14 - 28 * t * t, -6 + 10 * Math.sin(t * Math.PI)]); }
    s.line(path, { role: "ink", width: 11 });
    s.line(path, { role: "accent-2", width: 6.4 });
    s.line(path.map(([x, y, z]) => [x, y, z + 1.4]), { role: "accent-1", width: 2 });
    s.arc(path[0], 4.4, { axis: "x", role: "ink", width: 2 });
    return s;
  },
  "loft": () => {
    const s = sc({ cy: 34 });
    s.turned({ at: [0, 0, -12], r0: 14, r1: 7, h0: 0, h1: 20, kind: A, n: 4 });
    s.arc([0, 0, -12], 14, { role: "guide", width: 1.6, dash: "3 3", layer: "under" });
    return s;
  },
  "pipe": () => sc({ cy: 34 })
    .prism(S.circle(0, 0, 7), { axis: "y", from: -16, to: 4, at: [-6, 0, 0], kind: A })
    .bore([-6, 4, 0], 4, 3, { axis: "y" }),
  "rib": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -10], [15, 15, -4]);
    s.box([-15, -15, -4], [-9, 15, 12]);
    s.prism([[-9, -4], [8, -4], [-9, 12]], { axis: "y", from: -2, to: 2, kind: A });
    return s;
  },
  "web": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -10], [15, 15, -5]);
    for (const y of [-9, 1, 11]) s.box([-13, y - 1.5, -5], [13, y + 1.5, 6], { kind: A });
    return s;
  },
  "lip": () => {
    const s = sc({ cy: 37 });
    s.box([-15, -15, -10], [15, 15, 2]);
    s.box([-15, -15, 2], [15, -10, 9], { kind: A });
    s.box([-15, -10, 2], [-10, 15, 9], { kind: A });
    return s;
  },
  "fillet": () => sc({ cy: 34 }).prism([[-14, -8], [14, -8], ...S.arcPts(4, 2, 10, 0, 90), [-14, 12]], { axis: "x", from: -14, to: 14, sideKind: (i) => (i >= 2 && i <= 10 ? A : null) }),
  "chamfer": () => sc({ cy: 34 }).prism([[-14, -8], [14, -8], [14, 3], [5, 12], [-14, 12]], { axis: "x", from: -14, to: 14, sideKind: (i) => (i === 2 ? A : null) }),
  "draft": () => {
    const s = sc({ cy: 36 });
    s.box([-17, -17, -12], [17, 17, -6]);
    return s.turned({ at: [0, 0, -6], r0: 15, r1: 10, h0: 0, h1: 14, n: 4, rot: 45, kind: A });
  },
  "shell": () => block(sc()).pocket([[-10, -10, 4], [10, -10, 4], [10, 10, 4], [-10, 10, 4]], 10, { kind: A }),
  "hole": () => block(sc()).bore([0, 0, 4], 7, 8, { kind: A }),
  "thread": () => {
    const s = sc({ cy: 34 }).cylinder([0, 0, -14], 9, 26);
    for (let z = -10; z <= 8; z += 4.5) s.arc([0, 0, z], 9.3, { a0: -45, a1: 135, role: "accent-3", width: 2 });
    return s;
  },
  "coil": () => {
    const s = sc({ cy: 34 });
    const pts = [];
    for (let i = 0; i <= 240; i++) { const t = (i / 240) * Math.PI * 2 * 3.25; pts.push([Math.cos(t) * 12, Math.sin(t) * 12, -16 + (i / 240) * 30]); }
    s.line(pts, { role: "ink", width: 6 });
    s.line(pts, { role: "accent-2", width: 3.2 });
    return s;
  },
  "screw": () => {
    const s = sc({ cy: 30 });
    s.cylinder([0, 0, -18], 5, 24);
    for (let z = -15; z <= 3; z += 4) s.arc([0, 0, z], 5.3, { a0: -45, a1: 135, role: "ink", width: 1.8 });
    s.prism(S.ngon(0, 0, 11, 6, 0), { from: 6, to: 12, kind: A });
    return s;
  },
  "fastener": () => {
    const s = sc({ cy: 34 });
    s.box([-16, -16, -8], [16, 16, -2]);
    s.prism(S.ngon(0, 0, 9, 6, 0), { from: -2, to: 4, kind: A });
    s.cylinder([0, 0, 4], 4, 6, { kind: A });
    return s;
  },
  "snap_fit": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -10], [15, 15, -5]);
    s.prism([[-3, -5], [3, -5], [3, 9], [7, 9], [3, 15], [-3, 15]], { axis: "y", from: -6, to: 6, kind: A });
    return s;
  },
  "emboss": () => {
    const s = sc({ cy: 36 });
    s.box([-16, -16, -10], [16, 16, 0]);
    s.prism([[-9, -9], [9, -9], [9, -4], [-4, -4], [-4, 9], [-9, 9]], { from: 0, to: 5, kind: A });
    return s;
  },
  "decal": () => {
    const s = sc({ cy: 36 });
    s.box([-16, -16, -10], [16, 16, 0]);
    s.plane([[-9, -9, 0.1], [9, -9, 0.1], [9, 9, 0.1], [-9, 9, 0.1]], { role: "accent-1", stroke: "accent-3", layer: "over" });
    s.arc([-2, 2, 0.1], 4, { role: "accent-3", width: 2 });
    return s;
  },
  "thicken": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -6], [15, 15, 0], { kind: A });
    s.plane([[-15, -15, 0], [15, -15, 0], [15, 15, 0], [-15, 15, 0]], { role: "face-1", stroke: "ink", width: 2.4, layer: "over" });
    s.arrow([0, 0, 2], [0, 0, 16]);
    return s;
  },
  "scale": () => {
    const s = sc({ cy: 36 });
    s.box([-15, -15, -10], [10, 10, 10], { kind: A });
    s.box([-15, -15, -10], [-1, -1, 2]);
    return s;
  },
  "mirror": () => {
    const s = sc({ cy: 34 });
    s.box([-16, 3, -8], [10, 15, 6], { kind: A });
    s.plane([[-18, 0, -12], [12, 0, -12], [12, 0, 12], [-18, 0, 12]], { role: "face-1", stroke: "guide", dash: "4 3" });
    s.box([-16, -15, -8], [10, -3, 6]);
    return s;
  },
  "move": () => {
    const s = sc({ cy: 34 });
    s.box([-8, -8, -6], [8, 8, 6], { kind: A });
    s.arrow([8, 0, 0], [21, 0, 0]);
    s.arrow([0, 8, 0], [0, 21, 0]);
    s.arrow([0, 0, 6], [0, 0, 19]);
    return s;
  },
  "copy_body": () => {
    const s = sc({ cy: 34 });
    s.box([-14, -2, -9], [0, 12, 5]);
    s.box([0, -14, -2], [14, 0, 12], { kind: A });
    return s;
  },
  "combine": () => {
    const s = sc({ cy: 34 });
    s.box([-15, -6, -9], [6, 12, 3]);
    s.cylinder([6, -6, -9], 9, 17, { kind: A });
    return s;
  },
  "split_body": () => {
    const s = sc({ cy: 34 });
    s.box([-14, 2, -9], [14, 14, 5]);
    s.box([-14, -14, -9], [14, -2, 5], { kind: A });
    s.line([[-20, 0, -12], [20, 0, -12], [20, 0, 10], [-20, 0, 10]], { role: "guide", closed: true, width: 1.6, dash: "4 3", layer: "under" });
    return s;
  },
  "split_face": () => {
    const s = block(sc());
    s.plane([[-14, -14, 4], [0, -14, 4], [0, 14, 4], [-14, 14, 4]], { role: "accent-1", stroke: "accent-3", layer: "over" });
    return s.line([[0, -14, 4], [0, 14, 4]], { role: "accent-3", width: 2.6 });
  },
  "delete_face": () => {
    const s = sc({ cy: 34 }).box([-14, -14, -8], [14, 14, 4]);
    s.plane([[-14, -14, 4], [14, -14, 4], [14, 14, 4], [-14, 14, 4]], { role: "paper", stroke: "accent-3", dash: "4 3", layer: "over" });
    return badge(s, "cross");
  },
  "offset_face": () => {
    const s = sc({ cy: 38 });
    s.box([-14, -14, -9], [14, 14, 0]);
    s.box([-14, -14, 0], [14, 14, 5], { kind: A });
    s.line([[16, -14, 0], [16, 14, 0]], { role: "guide", width: 1.6, dash: "3 3" });
    return s.arrow([0, 0, 7], [0, 0, 21]);
  },
  "replace_face": () => {
    const s = sc({ cy: 36 }).box([-14, -14, -10], [14, 14, 0]);
    s.prism([[-14, 0], [14, 0], [14, 4, "s"], ...S.arcPts(0, -14, 21, 42, 138).slice(1, -1), [-14, 4]], { axis: "x", from: -14, to: 14, sideKind: (i) => (i > 0 && i < 13 ? A : null) });
    return s;
  },
  "bounding_solid": () => {
    const s = sc({ cy: 34 });
    s.line([[-14, -14, -10], [14, -14, -10], [14, 14, -10], [-14, 14, -10], [-14, -14, -10], [-14, -14, 10], [14, -14, 10], [14, 14, 10], [-14, 14, 10], [-14, -14, 10]], { role: "guide", width: 1.6, dash: "3 3", layer: "under" });
    s.sphere([0, 0, 0], 11, { kind: A });
    s.line([[14, -14, -10], [14, -14, 10]], { role: "guide", width: 1.6, dash: "3 3" });
    s.line([[14, 14, -10], [14, 14, 10], [-14, 14, 10], [-14, 14, -10]], { role: "accent-3", width: 2 });
    s.line([[14, 14, 10], [14, -14, 10]], { role: "accent-3", width: 2 });
    s.line([[14, 14, 10], [-14, 14, 10]], { role: "accent-3", width: 2 });
    return s;
  },
  "gear": () => sc({ cy: 34 }).prism(S.gear(0, 0, 12, 17, 9), { from: -4, to: 4, kind: A }).bore([0, 0, 4], 5, 8),
  "rack_pinion": () => {
    const s = sc({ cy: 34 });
    s.box([-18, 4, -14], [18, 12, -8]);
    for (let x = -15; x <= 15; x += 6) s.box([x - 1.5, 4, -8], [x + 1.5, 12, -5]);
    return s.prism(S.gear(0, 0, 8, 11, 8), { axis: "y", from: 4, to: 10, at: [0, -4, 6], kind: A }).bore([0, 6, 6], 3, 4, { axis: "y" });
  },
  "mesh_body": () => {
    const s = sc({ cy: 34 }).box([-13, -13, -9], [13, 13, 9], { kind: A });
    const L = (p) => s.line(p, { role: "accent-3", width: 1.3 });
    for (const t of [-4.3, 4.3]) { L([[t, -13, 9], [t, 13, 9]]); L([[-13, t, 9], [13, t, 9]]); L([[13, t, -9], [13, t, 9]]); L([[t, 13, -9], [t, 13, 9]]); }
    for (const z of [-3, 3]) { L([[13, -13, z], [13, 13, z]]); L([[-13, 13, z], [13, 13, z]]); }
    return s;
  },
  "center_of_mass": () => {
    const s = block(sc());
    s.path("M32 22A9 9 0 0 1 41 31L32 31Z", { fill: "accent-3", stroke: null });
    s.path("M32 40A9 9 0 0 1 23 31L32 31Z", { fill: "accent-3", stroke: null });
    s.path("M23 31A9 9 0 0 1 32 22M41 31A9 9 0 0 1 32 40", { fill: "none", stroke: "ink", width: 2 });
    s.path("M32 22L32 40M23 31L41 31", { fill: "none", stroke: "ink", width: 1.6 });
    return s;
  },
  "material": () => {
    const s = sc({ cy: 32 }).sphere([0, 0, 0], 19, { kind: A });
    return s;
  },
  "appearance": () => {
    const s = sc({ cy: 34 }).box([-13, -13, -9], [13, 13, 9]);
    s.plane([[13, -13, 9], [13, 13, 9], [13, 13, -9], [13, -13, -9]], { role: "accent-2", stroke: "ink", width: 2.4, layer: "over" });
    s.plane([[-13, 13, 9], [13, 13, 9], [13, 13, -9], [-13, 13, -9]], { role: "accent-1", stroke: "ink", width: 2.4, layer: "over" });
    return s;
  },
  "pattern": () => {
    const s = sc({ cy: 36 }).box([-16, -16, -10], [16, 16, -4]);
    for (const x of [-8, 8]) for (const y of [-8, 8]) s.cylinder([x, y, -4], 4.5, 9, { kind: x === -8 && y === -8 ? "face" : A });
    return s;
  },
  "pattern_rect": () => {
    const s = sc({ cy: 36 }).box([-17, -17, -10], [17, 17, -5]);
    for (const x of [-10, 0, 10]) for (const y of [-10, 0, 10]) s.box([x - 3, y - 3, -5], [x + 3, y + 3, 1], { kind: x === -10 && y === -10 ? "face" : A });
    return s;
  },
  "pattern_circ": () => {
    const s = sc({ cy: 36 }).cylinder([0, 0, -10], 17, 5);
    for (let i = 0; i < 6; i++) { const t = (i / 6) * Math.PI * 2; s.cylinder([Math.cos(t) * 11, Math.sin(t) * 11, -5], 3.2, 6, { kind: i === 0 ? "face" : A }); }
    return s.dot([0, 0, -5], { r: 2.2, role: "guide", ring: null });
  },
  "pattern_center": () => {
    const s = sc({ cy: 36 }).cylinder([0, 0, -10], 17, 5);
    for (let i = 0; i < 4; i++) { const t = (i / 4) * Math.PI * 2 + 0.6; s.cylinder([Math.cos(t) * 11, Math.sin(t) * 11, -5], 3.2, 6, { kind: A }); }
    s.line([[0, 0, -16], [0, 0, 14]], { role: "guide", width: 1.8, dash: "4 3" });
    return s.dot([0, 0, -5], { r: 3, role: "accent-3" });
  },
  "pattern_path": () => {
    const s = sc({ cy: 36 });
    const path = [];
    for (let i = 0; i <= 30; i++) { const t = i / 30; path.push([-16 + 32 * t, 12 * Math.sin(t * Math.PI) - 6, -8]); }
    s.line(path, { role: "guide", width: 1.8, dash: "4 3", layer: "under" });
    for (const t of [0.1, 0.5, 0.9]) s.box([-16 + 32 * t - 3, 12 * Math.sin(t * Math.PI) - 9, -8], [-16 + 32 * t + 3, 12 * Math.sin(t * Math.PI) - 3, -2], { kind: t === 0.1 ? "face" : A });
    return s;
  },
  "pattern_flip": () => {
    const s = sc({ cy: 34 });
    s.line([[-18, 18, -8], [18, -18, -8]], { role: "guide", width: 1.8, dash: "4 3", layer: "under" });
    s.prism([[0, 0], [12, 0], [0, 10]], { from: -8, to: 2, at: [-14, 2, 0] });
    s.prism([[0, 0], [-10, 0], [0, -12]], { from: -8, to: 2, at: [14, -2, 0], kind: A });
    return s;
  },
  "explode": () => {
    const s = sc({ cy: 36 });
    s.box([-12, -12, -14], [12, 12, -8]);
    s.box([-12, -12, -2], [12, 12, 4], { kind: A });
    s.cylinder([0, 0, 10], 6, 5, { kind: A });
    s.arrow([16, -16, -6], [16, -16, 8], { w: 2.4, head: 6 });
    return s;
  },
  "plastic_assign": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -10], [15, 15, 2], { kind: A }).pocket([[-11, -11, 2], [11, -11, 2], [11, 11, 2], [-11, 11, 2]], 9);
    return badge(s, "dot");
  },
  "plastic_rule": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -10], [15, 15, 2], { kind: A }).pocket([[-11, -11, 2], [11, -11, 2], [11, 11, 2], [-11, 11, 2]], 9);
    return badge(s, "check");
  },
  "end_cap": () => sc({ cy: 34 }).prism(S.circle(0, 0, 7), { axis: "y", from: -16, to: 8, at: [-4, 0, 0] }).cylinder([-4, 8, 0], 8, 3, { axis: "y", kind: A }),
  "end_cap_inward": () => sc({ cy: 34 }).prism(S.circle(0, 0, 7), { axis: "y", from: -16, to: 8, at: [-4, 0, 0] }).bore([-4, 8, 0], 7, 3, { axis: "y", kind: A }),
  "end_cap_outward": () => sc({ cy: 34 }).prism(S.circle(0, 0, 7), { axis: "y", from: -16, to: 4, at: [-4, 0, 0] }).turned({ at: [-4, 4, 0], axis: "y", r0: 7, r1: 3, h0: 0, h1: 8, kind: A }),
  "frame_member": () => {
    const s = sc({ cy: 34 });
    const I = [[-6, -9], [6, -9], [6, -6], [1.6, -6], [1.6, 6], [6, 6], [6, 9], [-6, 9], [-6, 6], [-1.6, 6], [-1.6, -6], [-6, -6]];
    return s.prism(I, { axis: "y", from: -15, to: 15, kind: A });
  },
};
