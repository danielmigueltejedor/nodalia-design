// Shared parts of the volumetric icons: a standard block, sketch planes, badges.
import { Scene, ROLES, STROKE } from "./scene.mjs";

export const sc = (o = {}) => new Scene({ cy: 34, ...o });

// The standard block most feature icons start from.
export function block(s, { x = 14, y = 14, z0 = -8, z1 = 4, kind = "face" } = {}) {
  return s.box([-x, -y, z0], [x, y, z1], { kind });
}

// A horizontal sketch plane at height z, half-size h.
export function sketchPlane(s, z = -6, h = 18, o = {}) {
  return s.plane([[-h, -h, z], [h, -h, z], [h, h, z], [-h, h, z]], o);
}

const r2 = (n) => Math.round(n * 100) / 100;

// A circle path in screen space using two arcs.
export function circlePath(cx, cy, r) {
  return `M${r2(cx - r)} ${r2(cy)}A${r2(r)} ${r2(r)} 0 1 0 ${r2(cx + r)} ${r2(cy)}A${r2(r)} ${r2(r)} 0 1 0 ${r2(cx - r)} ${r2(cy)}Z`;
}

// A round badge in the lower right corner carrying a small glyph: plus, minus, check, cross,
// arrow-up, arrow-down, refresh, play, pause, stop, warning, link.
export function badge(s, glyph, { role = "accent-3", cx = 50, cy = 50, r = 10 } = {}) {
  s.path(circlePath(cx, cy, r), { fill: role, stroke: "ink", width: 1.8 });
  const w = (d, width = 2.6) => s.path(d, { fill: "none", stroke: "paper", width });
  const f = (d) => s.path(d, { fill: "paper", stroke: null });
  const k = r / 10, X = (v) => r2(cx + v * k), Y = (v) => r2(cy + v * k);
  switch (glyph) {
    case "plus": w(`M${X(-5)} ${Y(0)}L${X(5)} ${Y(0)}M${X(0)} ${Y(-5)}L${X(0)} ${Y(5)}`); break;
    case "minus": w(`M${X(-5)} ${Y(0)}L${X(5)} ${Y(0)}`); break;
    case "check": w(`M${X(-5)} ${Y(0)}L${X(-1.5)} ${Y(4)}L${X(5)} ${Y(-4)}`); break;
    case "cross": w(`M${X(-4)} ${Y(-4)}L${X(4)} ${Y(4)}M${X(4)} ${Y(-4)}L${X(-4)} ${Y(4)}`); break;
    case "arrow-up": w(`M${X(0)} ${Y(5)}L${X(0)} ${Y(-5)}M${X(-4)} ${Y(-1)}L${X(0)} ${Y(-5)}L${X(4)} ${Y(-1)}`); break;
    case "arrow-down": w(`M${X(0)} ${Y(-5)}L${X(0)} ${Y(5)}M${X(-4)} ${Y(1)}L${X(0)} ${Y(5)}L${X(4)} ${Y(1)}`); break;
    case "arrow-right": w(`M${X(-5)} ${Y(0)}L${X(5)} ${Y(0)}M${X(1)} ${Y(-4)}L${X(5)} ${Y(0)}L${X(1)} ${Y(4)}`); break;
    case "refresh": w(`M${X(4.5)} ${Y(-1)}A${r2(5 * k)} ${r2(5 * k)} 0 1 0 ${X(3)} ${Y(4)}M${X(5)} ${Y(-5)}L${X(4.5)} ${Y(-1)}L${X(0.8)} ${Y(-1.6)}`); break;
    case "undo": w(`M${X(-4.5)} ${Y(-1)}A${r2(5 * k)} ${r2(5 * k)} 0 1 1 ${X(-3)} ${Y(4)}M${X(-5)} ${Y(-5)}L${X(-4.5)} ${Y(-1)}L${X(-0.8)} ${Y(-1.6)}`); break;
    case "play": f(`M${X(-3)} ${Y(-5)}L${X(5)} ${Y(0)}L${X(-3)} ${Y(5)}Z`); break;
    case "pause": w(`M${X(-2.5)} ${Y(-4.5)}L${X(-2.5)} ${Y(4.5)}M${X(2.5)} ${Y(-4.5)}L${X(2.5)} ${Y(4.5)}`, 3); break;
    case "stop": f(`M${X(-4)} ${Y(-4)}L${X(4)} ${Y(-4)}L${X(4)} ${Y(4)}L${X(-4)} ${Y(4)}Z`); break;
    case "warning": w(`M${X(0)} ${Y(-5)}L${X(0)} ${Y(1)}M${X(0)} ${Y(4.6)}L${X(0)} ${Y(4.7)}`, 3); break;
    case "dot": f(circlePath(cx, cy, 3.5 * k)); break;
    case "eye": w(`M${X(-6)} ${Y(0)}Q${X(0)} ${Y(-6)} ${X(6)} ${Y(0)}Q${X(0)} ${Y(6)} ${X(-6)} ${Y(0)}Z`, 2); f(circlePath(cx, cy, 1.8 * k)); break;
    default: throw new Error("unknown badge glyph " + glyph);
  }
  return s;
}

export { Scene, ROLES, STROKE };
