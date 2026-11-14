// 2D profile helpers for Scene.prism: points are [u, v] or [u, v, "s"] for a smooth vertex.

// Points along an arc (degrees), every vertex smooth except the two ends.
export function arcPts(cx, cy, r, a0, a1, n = Math.max(4, Math.round(Math.abs(a1 - a0) / 10))) {
  return Array.from({ length: n + 1 }, (_, i) => {
    const t = ((a0 + ((a1 - a0) * i) / n) * Math.PI) / 180;
    return [cx + Math.cos(t) * r, cy + Math.sin(t) * r, i > 0 && i < n ? "s" : undefined].filter((x) => x !== undefined);
  });
}

export function circle(cx, cy, r, n = 40) {
  return Array.from({ length: n }, (_, i) => {
    const t = (i / n) * Math.PI * 2;
    return [cx + Math.cos(t) * r, cy + Math.sin(t) * r, "s"];
  });
}

export function rect(x0, y0, x1, y1) { return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]; }

// A rectangle with rounded corners (radius r), counter-clockwise.
export function roundRect(x0, y0, x1, y1, r) {
  return smoothAll([
    ...arcPts(x1 - r, y0 + r, r, -90, 0),
    ...arcPts(x1 - r, y1 - r, r, 0, 90),
    ...arcPts(x0 + r, y1 - r, r, 90, 180),
    ...arcPts(x0 + r, y0 + r, r, 180, 270),
  ]);
}

export function smoothAll(pts) { return pts.map((p) => [p[0], p[1], "s"]); }

// Regular polygon.
export function ngon(cx, cy, r, n, rot = -90) {
  return Array.from({ length: n }, (_, i) => {
    const t = ((rot + (360 * i) / n) * Math.PI) / 180;
    return [cx + Math.cos(t) * r, cy + Math.sin(t) * r];
  });
}

// A gear outline: n teeth between radii r0 and r1.
export function gear(cx, cy, r0, r1, n) {
  const out = [];
  for (let i = 0; i < n; i++) {
    const a = (360 * i) / n, d = 360 / n;
    for (const [f, r] of [[0.0, r0], [0.18, r1], [0.5, r1], [0.68, r0]]) {
      const t = ((a + d * f) * Math.PI) / 180;
      out.push([cx + Math.cos(t) * r, cy + Math.sin(t) * r]);
    }
  }
  return out;
}
