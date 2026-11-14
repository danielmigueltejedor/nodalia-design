// Builds the volumetric CAD icons: <out>/<name>.svg, <out>/dark/<name>.svg, index.json and previews.
// usage: node volumetric/build.mjs [outDir]  (the package build writes dist/cad3d)
// ONLY=group,group limits the preview to those groups; BIG=1 draws it larger.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { ROLES, ROLES_DARK } from "./scene.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));

/** Every icon, in group order: [{ name, group, svg }]. Names are kebab-case and unique. */
export async function render() {
  const icons = [];
  const seen = new Set();
  for (const file of fs.readdirSync(path.join(here, "icons")).filter((f) => f.endsWith(".mjs")).sort()) {
    const group = file.replace(/\.mjs$/, "");
    const set = (await import(pathToFileURL(path.join(here, "icons", file)))).default;
    for (const [id, make] of Object.entries(set)) {
      const name = id.replace(/_/g, "-");
      if (seen.has(name)) throw new Error("duplicate icon " + name);
      seen.add(name);
      const title = name.replace(/-/g, " ").replace(/^./, (c) => c.toUpperCase());
      icons.push({ name, group, svg: make().svg(title) });
    }
  }
  return icons;
}

/** The same drawing in the dark display colours. */
export const dark = (svg) => Object.entries(ROLES).reduce((t, [r, hex]) => t.replaceAll(`"${hex}"`, `"${ROLES_DARK[r]}"`), svg);

export async function build(out) {
  const icons = await render();
  fs.mkdirSync(path.join(out, "dark"), { recursive: true });
  const index = { roles: ROLES, rolesDark: ROLES_DARK, icons: {} };
  const big = Boolean(process.env.BIG);
  let html = `<!doctype html><meta charset="utf-8"><title>NDL volumetric CAD icons</title><style>
body{font:12px system-ui,sans-serif;margin:16px;background:#fff;color:#15171e}body.dark{background:#2b2e38;color:#eef0f4}
.g{display:flex;flex-wrap:wrap;gap:6px}.i{width:${big ? 120 : 92}px;text-align:center}.i img{display:block;margin:0 auto}h2{margin:18px 0 6px}
</style><body class="">`;
  let group = null;
  for (const icon of icons) {
    fs.writeFileSync(path.join(out, icon.name + ".svg"), icon.svg);
    fs.writeFileSync(path.join(out, "dark", icon.name + ".svg"), dark(icon.svg));
    index.icons[icon.name] = { group: icon.group };
    if (process.env.ONLY && !process.env.ONLY.split(",").includes(icon.group)) continue;
    if (icon.group !== group) html += `${group ? "</div>" : ""}<h2>${icon.group}</h2><div class="g">`;
    group = icon.group;
    html += `<div class="i"><img src="${icon.name}.svg" width="${big ? 96 : 48}"><img src="${icon.name}.svg" width="24" style="margin-top:4px">${icon.name}</div>`;
  }
  if (group) html += "</div>";
  fs.writeFileSync(path.join(out, "index.json"), JSON.stringify(index, null, 2) + "\n");
  fs.writeFileSync(path.join(out, "preview.html"), html);
  fs.writeFileSync(path.join(out, "preview-dark.html"), html.replace('<body class="', '<body class="dark ').replaceAll('<img src="', '<img src="dark/'));
  return icons.length;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const out = process.argv[2] || path.join(here, "..", "dist", "cad3d");
  console.log(`${await build(out)} volumetric icons -> ${out}`);
}
