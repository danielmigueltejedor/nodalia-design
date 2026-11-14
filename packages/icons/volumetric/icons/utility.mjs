// Files, history, organisation, status and workspaces, as small objects.
import { sc, badge, token, Scene } from "../motifs.mjs";
import * as S from "../shapes.mjs";

const A = "accent";

// A sheet of paper lying flat, seen from above like a sketch, with lines of text.
const desk = (o = {}) => new Scene({ cy: 32, az: 22, el: 58, k: 1.02, ...o });
const page = (s, { kind = "face", x = 0, y = 0, z = 0 } = {}) => {
  s.box([x - 10, y - 13, z - 1.6], [x + 10, y + 13, z], { kind });
  for (const dy of [-7, -2, 3]) s.line([[x - 6, y + dy, z], [x + 6, y + dy, z]], { role: kind === A ? "accent-3" : "face-3", width: 2, layer: s.items.length - 1 });
  return s;
};
const folder = (s, kind = A) => {
  s.box([-14, -14, -10], [14, 14, -7], { kind });
  s.box([-14, -14, -7], [14, -11, 8], { kind });
  s.box([-14, -11, -7], [-11, 14, 8], { kind });
  return s.box([-14, -14, 8], [-11, -2, 11], { kind });
};
const slab = (s, z, kind = "face", h = 3) => s.box([-14, -14, z], [14, 14, z + h], { kind });
const tray = (s) => s.box([-16, -16, -12], [16, 16, -8]);

export default {
  "new": () => badge(page(desk(), { kind: A }), "plus"),
  "open": () => {
    const s = folder(sc({ cy: 36 }), "face");
    return s.prism([[-11, -11], [14, -11], [14, 14], [-11, 14]], { from: -7, to: -3, kind: A });
  },
  "folder": () => folder(sc({ cy: 36 })),
  "import": () => {
    const s = sc({ cy: 38 });
    tray(s);
    s.box([-7, -7, -8], [7, 7, 0], { kind: A });
    return s.arrow([0, 0, 22], [0, 0, 3]);
  },
  "export": () => {
    const s = sc({ cy: 38 });
    tray(s);
    s.box([-7, -7, -8], [7, 7, 0], { kind: A });
    return s.arrow([0, 0, 3], [0, 0, 22]);
  },
  "copy": () => { const s = desk(); page(s, { x: -4, y: -4, z: 0 }); return page(s, { kind: A, x: 4, y: 4, z: 4 }); },
  "paste": () => {
    const s = desk({ cy: 30 });
    s.box([-12, -15, -2], [12, 15, 0]);
    s.box([-5, -17, 0], [5, -12, 2.5]);
    return page(s, { kind: A, x: 2, y: 3, z: 3 });
  },
  "delete": () => {
    const s = sc({ cy: 34 });
    s.turned({ at: [0, 0, -14], r0: 9, r1: 11, h0: 0, h1: 22, kind: A });
    s.turned({ at: [0, 0, 8], r0: 13, r1: 13, h0: 0, h1: 3 });
    return s.box([-3, -3, 11], [3, 3, 14]);
  },
  "edit": () => token(sc(), "M14 50L18 38L44 12L52 20L26 46Z", { top: "accent-2" }).path("M18 38L26 46M40 16L48 24", { fill: "none", stroke: "ink", width: 2.2 }),
  "rename": () => {
    const s = sc({ cy: 34 }).box([-6, -16, -7], [6, 16, 3]);
    s.plane([[6.05, -12, -4], [6.05, 6, -4], [6.05, 6, 0], [6.05, -12, 0]], { role: "accent-1", stroke: null, layer: "over" });
    return s.line([[6.1, 9, -6], [6.1, 9, 6]], { role: "accent-3", width: 2.6 });
  },
  "select": () => {
    const s = sc({ cy: 32 }).box([-8, -8, -10], [8, 8, 2]);
    s.path("M30 18L30 44L36.5 38L41 48L45 46L40.5 36L49 36Z", { fill: "accent-2", stroke: "ink", width: 2.2 });
    return s;
  },
  "filter": () => sc({ cy: 34 }).turned({ at: [0, 0, -14], r0: 3, r1: 15, h0: 6, h1: 22, kind: A }).cylinder([0, 0, -14], 3, 6, { kind: A }),
  "list": () => { const s = sc({ cy: 38 }); for (const z of [-12, -5, 2]) slab(s, z, z === 2 ? A : "face"); return s; },
  "reorder": () => {
    const s = sc({ cy: 38 });
    slab(s, -12); slab(s, -5, A); slab(s, 2);
    return s.arcArrow([0, 0, -3], 18, 120, 230, { axis: "x" });
  },
  "library": () => {
    const s = sc({ cy: 36 });
    s.box([-12, -10, -12], [-4, 10, 12]);
    s.box([-3, -10, -12], [5, 10, 8], { kind: A });
    return s.box([6, -10, -12], [14, 10, 4]);
  },
  "params": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -8], [15, 15, -4]);
    for (const [y, x] of [[-8, 4], [0, -6], [8, 8]]) {
      s.line([[-12, y, -4], [12, y, -4]], { role: "ink", width: 2 });
      s.cylinder([x, y, -4], 3, 4, { kind: A });
    }
    return s;
  },
  "configuration_table": () => {
    const s = sc({ cy: 34 }).box([-15, -15, -4], [15, 15, 0]);
    for (const t of [-5, 5]) { s.line([[t, -15, 0], [t, 15, 0]], { role: "ink", width: 1.6 }); s.line([[-15, t, 0], [15, t, 0]], { role: "ink", width: 1.6 }); }
    return s.plane([[-15, -15, 0], [-5, -15, 0], [-5, 15, 0], [-15, 15, 0]], { role: "accent-1", stroke: null, layer: 0 });
  },
  "feature_group": () => folder(sc({ cy: 36 })).box([-6, -6, -7], [6, 6, 3], { kind: "face" }),
  "checkpoint": () => {
    const s = sc({ cy: 34 }).cylinder([0, 0, -14], 7, 3);
    s.cylinder([0, 0, -11], 1.4, 26);
    return s.prism([[0, 0], [14, 4], [0, 8]], { axis: "y", from: -1, to: 1, at: [0, 0, 6], kind: A });
  },
  "rollback": () => {
    const s = sc({ cy: 32 }).box([-16, -4, -4], [16, 4, 0]);
    for (const x of [-10, -2, 6]) s.box([x - 2.5, -5, 0], [x + 2.5, 5, 4], { kind: x === -10 ? A : "face" });
    return badge(s, "undo");
  },
  "revert": () => badge(sc({ cy: 32 }).box([-11, -11, -10], [11, 11, 8], { kind: A }), "undo"),
  "restore": () => badge(folder(sc({ cy: 34 }), "face"), "undo"),
  "update": () => badge(sc({ cy: 32 }).box([-11, -11, -10], [11, 11, 8], { kind: A }), "refresh"),
  "activate": () => badge(sc({ cy: 32 }).box([-11, -11, -10], [11, 11, 8], { kind: A }), "check"),
  "placeholder": () => {
    const s = sc({ cy: 34 });
    const c = [[-11, -11, -10], [11, -11, -10], [11, 11, -10], [-11, 11, -10]], t = c.map(([x, y]) => [x, y, 10]);
    s.line([...t, t[0]], { role: "accent-3", width: 2.2, dash: "4 3" });
    s.line([c[1], c[2], c[3]], { role: "accent-3", width: 2.2, dash: "4 3" });
    for (const i of [1, 2, 3]) s.line([c[i], t[i]], { role: "accent-3", width: 2.2, dash: "4 3" });
    return s;
  },
  "star": () => token(sc(), "M32 8L38.5 23.2L54.8 24.6L42.4 35.4L46.1 51.4L32 43L17.9 51.4L21.6 35.4L9.2 24.6L25.5 23.2Z", { top: "accent-1" }),
  "play": () => sc({ cy: 34 }).prism([[-10, -12], [14, 0], [-10, 12]], { from: -4, to: 4, kind: A }),
  "timeline_play": () => {
    const s = sc({ cy: 36 }).box([-16, -4, -6], [16, 4, -2]);
    return s.prism([[-6, -8], [10, 0], [-6, 8]], { from: -2, to: 3, kind: A });
  },
  "timeline_stop": () => sc({ cy: 36 }).box([-16, -4, -6], [16, 4, -2]).box([-6, -6, -2], [6, 6, 3], { kind: A }),
  "compute": () => {
    const s = sc({ cy: 34 }).box([-12, -12, -4], [12, 12, 0]);
    for (let i = -8; i <= 8; i += 4) { s.box([i - 1, 12, -3], [i + 1, 15, -1]); s.box([12, i - 1, -3], [15, i + 1, -1]); }
    return s.box([-6, -6, 0], [6, 6, 3], { kind: A });
  },
  "cam": () => {
    const s = sc({ cy: 36 }).box([-15, -15, -12], [15, 15, -4]);
    s.pocket([[-12, -4, -4], [6, -4, -4], [6, 4, -4], [-12, 4, -4]], 3, { kind: A });
    s.cylinder([6, 0, -2], 3, 10, { kind: A });
    return s.cylinder([6, 0, 8], 5.5, 8);
  },
  "agent_chat": () => {
    const s = token(sc(), "M14 12L50 12A6 6 0 0 1 56 18L56 36A6 6 0 0 1 50 42L30 42L18 52L20 42L14 42A6 6 0 0 1 8 36L8 18A6 6 0 0 1 14 12Z");
    return s.path("M21 27L21.1 27M32 27L32.1 27M43 27L43.1 27", { fill: "none", stroke: "accent-3", width: 4.6 });
  },
  "accessibility": () => {
    const s = token(sc(), "M32 6A5.5 5.5 0 1 1 31.9 6ZM12 20L52 20L52 26L38 27L38 38L44 56L37.5 57L32 42L26.5 57L20 56L26 38L26 27L12 26Z");
    return s;
  },
  "status_error": () => badge(sc({ cy: 32 }).box([-11, -11, -10], [11, 11, 8]), "cross"),
  "status_warning": () => badge(sc({ cy: 32 }).box([-11, -11, -10], [11, 11, 8]), "warning"),
  "status_recovery": () => badge(sc({ cy: 32 }).box([-11, -11, -10], [11, 11, 8]), "refresh"),
  "ws_design": () => {
    const s = sc({ cy: 36 }).box([-13, -13, -10], [13, 13, 2]);
    return s.box([-13, -13, 2], [3, 3, 10], { kind: A });
  },
  "ws_render": () => {
    const s = sc({ cy: 34 });
    s.plane([[-17, -17, -10], [17, -17, -10], [17, 17, -10], [-17, 17, -10]], { role: "face-1", stroke: "face-3", width: 1.8 });
    return s.sphere([0, 0, 0], 11, { kind: A });
  },
  "ws_drawing": () => {
    const s = sc({ cy: 34 }).box([-16, -16, -3], [16, 16, 0]);
    s.line([[-10, -10, 0], [6, -10, 0], [6, 6, 0], [-10, 6, 0]], { role: "accent-3", width: 2.2, closed: true });
    s.line([[-10, 11, 0], [6, 11, 0]], { role: "ink", width: 1.6 });
    return s.line([[11, -10, 0], [11, 6, 0]], { role: "ink", width: 1.6 });
  },
  "ws_animate": () => {
    const s = sc({ cy: 34 });
    s.box([-14, 4, -10], [-4, 14, 0], { kind: "face" });
    s.box([4, -14, -2], [14, -4, 8], { kind: A });
    return s.line([[-6, 8, 2], [0, 0, 8], [6, -6, 10]], { role: "accent-3", width: 2, dash: "3 3" });
  },
};
