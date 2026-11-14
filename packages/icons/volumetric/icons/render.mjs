// Rendering: cameras, lights, scenes, shots and images.
import { sc, badge } from "../motifs.mjs";

const A = "accent";

// A camera body with a lens, facing the lower right.
const camera = (s, at = [0, 0, 0], kind = "face") => {
  const [x, y, z] = at;
  s.box([x - 10, y - 7, z - 6], [x + 4, y + 7, z + 6], { kind });
  s.cylinder([x + 4, y, z], 5, 7, { axis: "x", kind: A });
  return s.box([x - 7, y - 4, z + 6], [x - 1, y + 4, z + 9], { kind });
};
// A framed picture standing upright, facing the viewer's left.
const picture = (s, { kind = A, x = 0 } = {}) => {
  s.box([x - 1.2, -14, -12], [x + 1.2, 14, 12]);
  s.plane([[x + 1.25, -11, -9], [x + 1.25, 11, -9], [x + 1.25, 11, 9], [x + 1.25, -11, 9]], { role: kind === A ? "accent-1" : "face-1", stroke: null, layer: "over" });
  return s;
};
// Landscape inside a picture: a hill and a sun.
const scenery = (s, x = 1.3) => {
  s.plane([[x, -11, -9], [x, 11, -9], [x, 11, -1], [x, 3, 4], [x, -4, -2], [x, -11, 2]], { role: "accent-2", stroke: null, layer: "over" });
  return s.dot([x, -5, 5], { r: 2.6, role: "accent-3", ring: null });
};
const bulb = (s, at = [0, 0, 2], kind = A) => {
  s.sphere(at, 9, { kind });
  return s.cylinder([at[0], at[1], at[2] - 15], 4, 6);
};
const rays = (s, cx, cy, r0, r1, n = 8) => {
  let d = "";
  for (let i = 0; i < n; i++) { const t = (i / n) * Math.PI * 2; d += `M${(cx + Math.cos(t) * r0).toFixed(2)} ${(cy + Math.sin(t) * r0).toFixed(2)}L${(cx + Math.cos(t) * r1).toFixed(2)} ${(cy + Math.sin(t) * r1).toFixed(2)}`; }
  return s.path(d, { fill: "none", stroke: "accent-3", width: 2.6 });
};
const ground = (s, z = -10, h = 17, role = "face-1") => s.plane([[-h, -h, z], [h, -h, z], [h, h, z], [-h, h, z]], { role, stroke: "face-3", width: 1.8 });

export default {
  "camera": () => camera(sc({ cy: 33, k: 1.45 }), [-1, 0, 0]),
  "capture": () => badge(camera(sc({ cy: 30, k: 1.3 }), [-3, 0, 2]), "dot"),
  "render_lens": () => {
    const s = sc({ cy: 34 });
    s.cylinder([-10, 0, 0], 10, 20, { axis: "x", kind: A, hole: 6 });
    return s;
  },
  "render_scene": () => {
    const s = sc({ cy: 34 });
    ground(s);
    s.box([-10, -2, -10], [2, 10, 2]);
    return s.sphere([6, -8, -4], 6, { kind: A });
  },
  "render_ground": () => {
    const s = sc({ cy: 34 });
    s.plane([[-17, -17, -6], [17, -17, -6], [17, 17, -6], [-17, 17, -6]], { role: "accent-1", stroke: "accent-3", width: 2.2 });
    return s.cylinder([0, 0, -6], 7, 12);
  },
  "render_backdrop": () => {
    const s = sc({ cy: 34 });
    s.sheet((x, y) => (y < -4 ? -10 + ((y + 4) * (y + 4)) / 8 : -10), { x0: -14, x1: 14, y0: -15, y1: 14, n: 12, kind: A });
    return s.cylinder([2, 4, -10], 6, 10);
  },
  "render_environment": () => {
    const s = sc({ cy: 32 }).sphere([0, 0, 0], 18, { kind: A });
    s.path("M14 32A18 7 0 0 0 50 32", { fill: "none", stroke: "accent-3", width: 2 });
    return s.path("M32 14A7 18 0 0 0 32 50", { fill: "none", stroke: "accent-3", width: 2 });
  },
  "environment_map": () => {
    const s = sc({ cy: 32 }).sphere([0, 0, 0], 18, { kind: A });
    s.path("M14 32A18 7 0 0 0 50 32M14.6 26A18 5 0 0 0 49.4 26M14.6 38A18 5 0 0 0 49.4 38", { fill: "none", stroke: "accent-3", width: 1.6 });
    return s;
  },
  "ambient_occlusion": () => {
    const s = sc({ cy: 34 });
    ground(s, -8);
    s.plane([[-9, -9, -8], [11, -9, -8], [11, 11, -8], [-9, 11, -8]], { role: "face-3", stroke: null });
    return s.box([-8, -8, -8], [8, 8, 8], { kind: A });
  },
  "render_light_point": () => { const s = sc({ cy: 34 }); bulb(s); return rays(s, 32, 26, 13, 17); },
  "render_light_spot": () => {
    const s = sc({ cy: 34 });
    ground(s, -12);
    s.plane([[-10, -10, -12], [10, -10, -12], [10, 10, -12], [-10, 10, -12]], { role: "accent-1", stroke: null });
    return s.turned({ at: [0, 0, 2], r0: 7, r1: 3, h0: 0, h1: 9, kind: A });
  },
  "render_light_area": () => {
    const s = sc({ cy: 34 });
    ground(s, -12);
    s.box([-12, -12, 6], [12, 12, 9], { kind: A });
    for (const [x, y] of [[-8, -8], [8, -8], [-8, 8], [8, 8]]) s.line([[x, y, 5], [x, y, -6]], { role: "accent-3", width: 1.6, dash: "2 3" });
    return s;
  },
  "render_sun": () => {
    const s = sc({ cy: 32 }).sphere([0, 0, 0], 10, { kind: A });
    return rays(s, 32, 32, 14, 20, 10);
  },
  "render_light_by_highlight": () => {
    const s = sc({ cy: 34 }).sphere([0, 0, -2], 15);
    s.path("M24 25A4 4 0 1 0 32 25A4 4 0 1 0 24 25Z", { fill: "accent-1", stroke: "accent-3", width: 2.2 });
    return badge(s, "dot", { cx: 51, cy: 14, r: 8 });
  },
  "render_lighting_presets": () => {
    const s = sc({ cy: 34 });
    s.sphere([-9, 9, -6], 7, { kind: A });
    s.sphere([9, -9, -6], 7);
    return rays(s, 32, 14, 5, 9, 8).dot([0, 0, 14], { r: 3.6, role: "accent-2" });
  },
  "render_solo_light": () => { const s = sc({ cy: 34 }); bulb(s); return badge(s, "eye"); },
  "render_exposure": () => {
    const s = sc({ cy: 32 }).sphere([0, 0, 0], 17);
    return s.path("M32 15A17 17 0 0 1 32 49Z", { fill: "accent-2", stroke: "ink", width: 2.4 });
  },
  "render_image": () => scenery(picture(sc({ cy: 32 }))),
  "render_export_image": () => badge(scenery(picture(sc({ cy: 30 }))), "arrow-down"),
  "render_results": () => {
    const s = sc({ cy: 34 });
    picture(s, { kind: "face", x: -8 });
    picture(s, { x: 4 });
    return scenery(s, 5.3);
  },
  "render_aspect_frame": () => {
    const s = picture(sc({ cy: 32 }), { kind: "face" });
    return s.line([[1.3, -11, -5], [1.3, 11, -5]], { role: "accent-3", width: 2 }).line([[1.3, -11, 5], [1.3, 11, 5]], { role: "accent-3", width: 2 });
  },
  "render_frame_guides": () => {
    const s = picture(sc({ cy: 32 }), { kind: "face" });
    for (const y of [-4, 4]) s.line([[1.3, y, -9], [1.3, y, 9]], { role: "accent-3", width: 1.6 });
    for (const z of [-3, 3]) s.line([[1.3, -11, z], [1.3, 11, z]], { role: "accent-3", width: 1.6 });
    return s;
  },
  "render_output_size": () => {
    const s = picture(sc({ cy: 30 }), { kind: "face" });
    s.arrow([1.3, 9, -6], [1.3, -9, -6], { w: 2, head: 5 });
    return s.arrow([1.3, 9, -6], [1.3, 9, 7], { w: 2, head: 5 });
  },
  "render_transparent_background": () => {
    const s = picture(sc({ cy: 32 }), { kind: "face" });
    for (let i = 0; i < 4; i++) for (let j = 0; j < 3; j++) if ((i + j) % 2) s.plane([[1.3, -11 + i * 5.5, -9 + j * 6], [1.3, -5.5 + i * 5.5, -9 + j * 6], [1.3, -5.5 + i * 5.5, -3 + j * 6], [1.3, -11 + i * 5.5, -3 + j * 6]], { role: "face-2", stroke: null, layer: "over" });
    return s.sphere([3, 0, 0], 5, { kind: A });
  },
  "render_shot": () => {
    const s = sc({ cy: 34 });
    camera(s, [-6, 0, 2]);
    return s.line([[5, 0, 2], [18, -8, 10], [18, 8, 10], [5, 0, 2], [18, -8, -6], [18, 8, -6], [5, 0, 2]], { role: "accent-3", width: 1.6, dash: "3 2" });
  },
  "render_shots_strip": () => {
    const s = sc({ cy: 34 });
    for (const y of [-11, 0, 11]) s.box([-1, y - 5, -6], [1, y + 5, 4], { kind: y === 0 ? A : "face" });
    return s;
  },
  "render_all_shots": () => {
    const s = sc({ cy: 34 });
    for (const y of [-11, 0, 11]) s.box([-1, y - 5, -6], [1, y + 5, 4], { kind: A });
    return badge(s, "play");
  },
  "render_update_shot": () => badge(camera(sc({ cy: 30, k: 1.3 }), [-3, 0, 2]), "refresh"),
  "render_execute": () => badge(scenery(picture(sc({ cy: 30 }))), "play"),
  "render_cancel": () => badge(scenery(picture(sc({ cy: 30 }))), "stop"),
  "render_check": () => badge(scenery(picture(sc({ cy: 30 }))), "check"),
  "render_compare": () => {
    const s = sc({ cy: 32 });
    picture(s, { kind: "face" });
    s.plane([[1.3, 0, -9], [1.3, 11, -9], [1.3, 11, 9], [1.3, 0, 9]], { role: "accent-2", stroke: null, layer: "over" });
    return s.line([[1.3, 0, -12], [1.3, 0, 12]], { role: "ink", width: 2.4 });
  },
  "render_quality": () => {
    const s = sc({ cy: 34 });
    s.box([-15, -15, -10], [15, 15, -6]);
    for (const [x, h] of [[-8, 4], [0, 9], [8, 14]]) s.box([x - 3, -3, -6], [x + 3, 3, -6 + h], { kind: A });
    return s;
  },
  "render_fit_product": () => {
    const s = sc({ cy: 34 }).box([-7, -7, -7], [7, 7, 7], { kind: A });
    for (const [a, b] of [[[-16, -16, 13], [-11, -11, 9]], [[16, 16, -13], [11, 11, -9]]]) s.arrow(a, b, { w: 2.2, head: 5 });
    return s.line([[-16, 16, 12], [-16, 16, 16], [-12, 16, 16]], { role: "accent-3", width: 2 }).line([[16, -16, -12], [16, -16, -16], [12, -16, -16]], { role: "accent-3", width: 2 });
  },
  "render_appearance": () => sc({ cy: 32 }).sphere([0, 0, 0], 18, { kind: A }),
  "render_from_material": () => {
    const s = sc({ cy: 34 });
    s.sphere([-6, 6, -2], 10);
    return s.sphere([8, -8, 2], 9, { kind: A });
  },
  "render_library": () => {
    const s = sc({ cy: 36 });
    s.box([-16, -16, -10], [16, 16, -6]);
    for (const [x, y, k] of [[-8, -8, A], [8, -8, "face"], [-8, 8, "face"], [8, 8, A]]) s.sphere([x, y, -1], 5, { kind: k });
    return s;
  },
  "render_paint": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -10], [15, 15, -2]);
    s.plane([[-15, -15, -2], [5, -15, -2], [5, 15, -2], [-15, 15, -2]], { role: "accent-2", stroke: null, layer: "over" });
    s.box([4, -3, 2], [10, 3, 10], { kind: A });
    return s.cylinder([7, 0, 10], 1.6, 9);
  },
  "render_pick_up": () => {
    const s = sc({ cy: 36 }).sphere([-6, 6, -4], 11, { kind: A });
    s.path("M44 10L54 20L41 33L37 33L35 31L35 27Z", { fill: "face-1", stroke: "ink", width: 2.2 });
    return s.path("M35 31L29 37", { fill: "none", stroke: "ink", width: 2.6 });
  },
  "render_copy_look": () => {
    const s = sc({ cy: 34 });
    s.sphere([-8, 8, -2], 10, { kind: A });
    s.sphere([9, -9, 2], 8, { kind: A });
    return s.arrow([-2, 2, 10], [6, -6, 14], { w: 2, head: 5 });
  },
  "render_same_look": () => {
    const s = sc({ cy: 34 });
    s.box([-15, -2, -10], [-3, 12, 2], { kind: A });
    return s.sphere([8, -8, -2], 9, { kind: A });
  },
  "render_replace_all": () => {
    const s = sc({ cy: 34 });
    for (const [x, y] of [[-10, 10], [0, 0], [10, -10]]) s.sphere([x, y, -2], 6, { kind: A });
    return badge(s, "refresh");
  },
  "colored_components": () => {
    const s = sc({ cy: 34 });
    s.box([-14, -14, -10], [0, 0, 4], { kind: A });
    s.box([0, 0, -10], [14, 14, 4]);
    return s.box([-14, 0, -10], [0, 14, -2], { kind: A });
  },
};
