// Builds the volumetric CAD icons: dist/cad3d/<name>.svg, dist/cad3d/index.json and a preview.
// usage: node volumetric/build.mjs [outDir]
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { ROLES, ROLES_DARK } from "./scene.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const out = process.argv[2] || path.join(here, "..", "dist", "cad3d");
const groups = {};
for (const file of fs.readdirSync(path.join(here, "icons")).filter((f) => f.endsWith(".mjs")).sort()) {
  groups[file.replace(/\.mjs$/, "")] = (await import(pathToFileURL(path.join(here, "icons", file)))).default;
}
fs.mkdirSync(out, { recursive: true });
const index = { roles: ROLES, rolesDark: ROLES_DARK, icons: {} };
let html = `<!doctype html><meta charset="utf-8"><title>NDL volumetric CAD icons</title><style>
body{font:12px system-ui,sans-serif;margin:16px;background:#fff;color:#15171e}body.dark{background:#2b2e38;color:#eef0f4}
.g{display:flex;flex-wrap:wrap;gap:6px}.i{width:${process.env.BIG ? 120 : 92}px;text-align:center}.i img{display:block;margin:0 auto}h2{margin:18px 0 6px}
</style><body class="${process.argv[3] || ""}">`;
const seen = new Set();
for (const [group, icons] of Object.entries(groups)) {
  const shown = !process.env.ONLY || process.env.ONLY.split(",").includes(group);
  if (shown) html += `<h2>${group}</h2><div class="g">`;
  for (const [id, make] of Object.entries(icons)) {
    const name = id.replace(/_/g, "-");
    if (seen.has(name)) throw new Error("duplicate icon " + name);
    seen.add(name);
    const title = name.replace(/-/g, " ").replace(/^./, (c) => c.toUpperCase());
    const svg = make().svg(title);
    fs.writeFileSync(path.join(out, name + ".svg"), svg);
    fs.mkdirSync(path.join(out, "dark"), { recursive: true });
    fs.writeFileSync(path.join(out, "dark", name + ".svg"), Object.entries(ROLES).reduce((t, [r, hex]) => t.replaceAll(`"${hex}"`, `"${ROLES_DARK[r]}"`), svg));
    index.icons[name] = { group };
    if (shown) html += `<div class="i"><img src="${name}.svg" width="${process.env.BIG ? 96 : 48}"><img src="${name}.svg" width="24" style="margin-top:4px">${name}</div>`;
  }
  if (shown) html += "</div>";
}
fs.writeFileSync(path.join(out, "index.json"), JSON.stringify(index, null, 2) + "\n");
fs.writeFileSync(path.join(out, "preview.html"), html);
fs.writeFileSync(path.join(out, "preview-dark.html"), html.replace('<body class="', '<body class="dark ').replaceAll('<img src="', '<img src="dark/'));
console.log(`${seen.size} volumetric icons -> ${out}`);
