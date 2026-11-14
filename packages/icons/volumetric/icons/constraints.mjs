// Sketch constraints: the constrained geometry in ink, the relation it gains in the accent.
import { sk, bez } from "./sketch.mjs";

export default {
  "c_coincident": () => sk().ref([[-14, 8], [0, 0]]).ref([[0, 0], [12, 10]]).pt(0, 0, { r: 3.6 }).s,
  "c_collinear": () => sk().line([[-15, 9], [-3, 1.8]]).line([[3, -1.8], [15, -9]]).guide([[-3, 1.8], [3, -1.8]], { role: "accent-3" }).s,
  "c_concentric": () => sk().refCircle(0, 0, 12).circle(0, 0, 6).pt(0, 0).s,
  "c_equal": () => sk().line([[-14, 6], [-2, 6]]).line([[2, -6], [14, -6]]).ref([[-8, 3], [-8, 9]], { width: 1.8 }).ref([[8, -9], [8, -3]], { width: 1.8 }).s,
  "c_fix": () => {
    const g = sk().ref([[-14, 10], [4, -4]]).pt(4, -4, { r: 3.4 });
    for (let x = -2; x <= 10; x += 4) g.line([[x, -8], [x + 3, -12]], { width: 1.8 });
    return g.line([[-3, -8], [12, -8]], { width: 2.2 }).s;
  },
  "c_hv": () => sk().line([[-12, 8], [10, 8]]).line([[10, 8], [10, -12]]).ref([[6, 8], [6, 4], [10, 4]], { width: 1.6 }).s,
  "c_midpoint": () => sk().ref([[-14, 10], [14, -10]]).ref([[0, 0], [-10, -12]], { width: 1.8 }).pt(0, 0, { r: 3.4 }).s,
  "c_parallel": () => sk().line([[-14, 4], [6, -12]]).line([[-6, 12], [14, -4]]).s,
  "c_perpendicular": () => sk().ref([[-14, 8], [14, 8]]).line([[0, 8], [0, -14]]).line([[0, 3], [5, 3], [5, 8]], { width: 1.8 }).s,
  "c_polygon": () => sk().loop([[0, -12], [11.4, -3.7], [7, 9.7], [-7, 9.7], [-11.4, -3.7]]).ref([[-2, -9], [2, -8]], { width: 1.6 }).s,
  "c_smooth": () => sk().ref(bez([-15, 8], [-10, -6], [-4, -6], [0, 0])).line(bez([0, 0], [4, 6], [10, 6], [15, -8])).pt(0, 0).s,
  "c_symmetry": () => sk().guide([[0, -16], [0, 16]], { dash: "6 3 2 3" }).pt(-9, 4).pt(9, 4).line([[-9, 4], [-3, 4]], { width: 1.8 }).line([[3, 4], [9, 4]], { width: 1.8 }).s,
  "c_tangent": () => sk().refCircle(-2, 3, 9).line([[-16, -6], [16, -6]]).pt(-2, -6).s,
};
