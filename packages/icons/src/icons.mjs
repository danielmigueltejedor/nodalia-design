// The NDL icon set. One entry per icon: the SVG elements drawn inside a 24 x 24 view box.
// Rules (docs/iconography.md): 3 px of clear margin, 1.75 px stroke with round caps and joins,
// filled circles for nodes, dashes only for what is not real geometry, a soft fill only for a face.
// Colours are always currentColor. Edit here and rebuild; never retouch a generated file.
// `fill` lists the elements the bold style draws solid (see bold.mjs); the regular style ignores it.
// `accent` lists the elements a CAD app draws in its accent colour: the geometry a tool creates,
// against the existing geometry in the ink. Every other consumer draws the icon in one colour.

export const ICONS = {
  // --- edit ---
  "undo": { group: "edit", body: [
    '<path d="M8 5 3.5 9.5 8 14"/>',
    '<path d="M3.5 9.5h11a5.25 5.25 0 0 1 0 10.5H11"/>',
  ] },
  "redo": { group: "edit", body: [
    '<path d="M16 5l4.5 4.5L16 14"/>',
    '<path d="M20.5 9.5h-11a5.25 5.25 0 0 0 0 10.5H13"/>',
  ] },
  "copy": { group: "edit", body: [
    '<rect x="4" y="8" width="12" height="12" rx="1.5"/>',
    '<path d="M8 8V5.5A1.5 1.5 0 0 1 9.5 4h9A1.5 1.5 0 0 1 20 5.5v9a1.5 1.5 0 0 1-1.5 1.5H16"/>',
  ] },
  "paste": { group: "edit", body: [
    '<path d="M9 5H6.5A1.5 1.5 0 0 0 5 6.5v13A1.5 1.5 0 0 0 6.5 21h11a1.5 1.5 0 0 0 1.5-1.5v-13A1.5 1.5 0 0 0 17.5 5H15"/>',
    '<rect x="9" y="3" width="6" height="4" rx="1"/>',
    '<path d="M9 12h6M9 16h4"/>',
  ] },
  "delete": { group: "edit", body: [
    '<path d="M4 7h16"/>',
    '<path d="M9.5 7V5a1 1 0 0 1 1-1h3a1 1 0 0 1 1 1v2"/>',
    '<path d="M6.5 7l.8 12.1a1.5 1.5 0 0 0 1.5 1.4h6.4a1.5 1.5 0 0 0 1.5-1.4L17.5 7"/>',
    '<path d="M10 11v5.5M14 11v5.5"/>',
  ] },
  "close": { group: "edit", body: [
    '<path d="M6 6l12 12M18 6 6 18"/>',
  ] },
  "check": { group: "edit", body: [
    '<path d="M4.5 12.5l5 5 10-11"/>',
  ] },
  "chevron-down": { group: "edit", body: [
    '<path d="M6 9.5l6 6 6-6"/>',
  ] },
  "chevron-up": { group: "edit", body: [
    '<path d="M6 14.5l6-6 6 6"/>',
  ] },
  "chevron-right": { group: "edit", body: [
    '<path d="M9.5 6l6 6-6 6"/>',
  ] },
  "search": { group: "edit", body: [
    '<circle cx="10.5" cy="10.5" r="6.5"/>',
    '<path d="M15.5 15.5l5 5"/>',
  ] },
  "settings": { group: "edit", fill: [1,3], body: [
    '<path d="M4 7h8M18 7h2"/>',
    '<circle cx="15" cy="7" r="2.5"/>',
    '<path d="M4 17h2M12 17h8"/>',
    '<circle cx="9" cy="17" r="2.5"/>',
  ] },
  // --- files ---
  "new": { group: "files", body: [
    '<path d="M13.5 3h-6A1.5 1.5 0 0 0 6 4.5v15A1.5 1.5 0 0 0 7.5 21h9a1.5 1.5 0 0 0 1.5-1.5v-12z"/>',
    '<path d="M13.5 3v4.5H18"/>',
    '<path d="M12 11.5v6M9 14.5h6"/>',
  ] },
  "open": { group: "files", body: [
    '<path d="M3.5 18.5v-12A1.5 1.5 0 0 1 5 5h4l2 2.5h8A1.5 1.5 0 0 1 20.5 9v9.5A1.5 1.5 0 0 1 19 20H5a1.5 1.5 0 0 1-1.5-1.5z"/>',
    '<path d="M3.5 11h17"/>',
  ] },
  "save": { group: "files", body: [
    '<path d="M5.5 4H16l4 4v10.5a1.5 1.5 0 0 1-1.5 1.5h-13A1.5 1.5 0 0 1 4 18.5v-13A1.5 1.5 0 0 1 5.5 4z"/>',
    '<path d="M8 4v4.5h6.5V4"/>',
    '<path d="M8 20v-6h8v6"/>',
  ] },
  "import": { group: "files", body: [
    '<path d="M12 3.5v11"/>',
    '<path d="M7.5 10.5 12 15l4.5-4.5"/>',
    '<path d="M4 16v3a1.5 1.5 0 0 0 1.5 1.5h13A1.5 1.5 0 0 0 20 19v-3"/>',
  ] },
  "export-image": { group: "files", body: [
    '<path d="M12 6H5A1.5 1.5 0 0 0 3.5 7.5v11A1.5 1.5 0 0 0 5 20h11a1.5 1.5 0 0 0 1.5-1.5V12"/>',
    '<path d="M3.5 16.5l4-4 6 6"/>',
    '<path d="M15.5 3.5h5v5"/>',
    '<path d="M20.5 3.5l-6 6"/>',
  ] },
  "camera": { group: "files", body: [
    '<path d="M4.5 7.5h3L9 5h6l1.5 2.5h3A1.5 1.5 0 0 1 21 9v9a1.5 1.5 0 0 1-1.5 1.5h-15A1.5 1.5 0 0 1 3 18V9a1.5 1.5 0 0 1 1.5-1.5z"/>',
    '<circle cx="12" cy="13" r="3.5"/>',
  ] },
  "undock": { group: "files", body: [
    '<path d="M10 6H5.5A1.5 1.5 0 0 0 4 7.5v11A1.5 1.5 0 0 0 5.5 20h11a1.5 1.5 0 0 0 1.5-1.5V14"/>',
    '<path d="M14 4h6v6"/>',
    '<path d="M20 4l-8 8"/>',
  ] },
  // --- run ---
  "run": { group: "run", fill: [0], body: [
    '<path d="M7.5 4.5v15L20 12z"/>',
  ] },
  "stop": { group: "run", fill: [0], body: [
    '<rect x="5.5" y="5.5" width="13" height="13" rx="2"/>',
  ] },
  // --- view ---
  "select": { group: "view", body: [
    '<path d="M5 3.5l5.4 15.2 2.3-6.2 6.2-2.3z"/>',
  ] },
  "pan": { group: "view", body: [
    '<path d="M12 3v18M3 12h18"/>',
    '<path d="M9.5 5.5 12 3l2.5 2.5M9.5 18.5 12 21l2.5-2.5M5.5 9.5 3 12l2.5 2.5M18.5 9.5 21 12l-2.5 2.5"/>',
  ] },
  "orbit": { group: "view", body: [
    '<circle cx="12" cy="12" r="2.5"/>',
    '<path d="M19.9 10.5a8 8 0 1 0-.9 5.4"/>',
    '<path d="M20.5 5.5v5h-5"/>',
  ] },
  "zoom": { group: "view", body: [
    '<circle cx="10.5" cy="10.5" r="6.5"/>',
    '<path d="M15.5 15.5l5 5"/>',
    '<path d="M10.5 8v5M8 10.5h5"/>',
  ] },
  "fit": { group: "view", body: [
    '<path d="M4 8.5v-3A1.5 1.5 0 0 1 5.5 4h3M15.5 4h3A1.5 1.5 0 0 1 20 5.5v3M20 15.5v3a1.5 1.5 0 0 1-1.5 1.5h-3M8.5 20h-3A1.5 1.5 0 0 1 4 18.5v-3"/>',
    '<rect x="8.5" y="8.5" width="7" height="7" rx="1"/>',
  ] },
  "view": { group: "view", body: [
    '<path d="M2.5 12S6 5.5 12 5.5 21.5 12 21.5 12 18 18.5 12 18.5 2.5 12 2.5 12z"/>',
    '<circle cx="12" cy="12" r="3"/>',
  ] },
  "wireframe": { group: "view", body: [
    '<path d="M12 3l8 4.5v9L12 21l-8-4.5v-9z"/>',
    '<path d="M4 7.5l8 4.5 8-4.5M12 12v9"/>',
  ] },
  "surface": { group: "view", body: [
    '<path d="M12 3l8 4.5-8 4.5-8-4.5z" fill="currentColor" fill-opacity="0.18"/>',
    '<path d="M12 3l8 4.5v9L12 21l-8-4.5v-9z"/>',
    '<path d="M4 7.5l8 4.5 8-4.5M12 12v9"/>',
  ] },
  // --- sketch ---
  "geometry": { group: "sketch", body: [
    '<path d="M5 18.5V8.5l7-4 7 5.5v8.5z"/>',
    '<circle cx="5" cy="18.5" r="1.9" fill="currentColor" stroke="none"/>',
    '<circle cx="5" cy="8.5" r="1.9" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="4.5" r="1.9" fill="currentColor" stroke="none"/>',
    '<circle cx="19" cy="10" r="1.9" fill="currentColor" stroke="none"/>',
    '<circle cx="19" cy="18.5" r="1.9" fill="currentColor" stroke="none"/>',
  ] },
  "sketch": { group: "sketch", body: [
    '<path d="M4 20l1-4.5L16.5 4a2.1 2.1 0 0 1 3 3L8 18.5z"/>',
    '<path d="M14.5 6l3.5 3.5"/>',
  ] },
  "line": { group: "sketch", accent: [0,1,2], body: [
    '<path d="M6.5 17.5l11-11"/>',
    '<circle cx="5" cy="19" r="2"/>',
    '<circle cx="19" cy="5" r="2"/>',
  ] },
  "rectangle": { group: "sketch", accent: [0,1,2], body: [
    '<rect x="4.5" y="6.5" width="15" height="11" rx="0.5"/>',
    '<circle cx="4.5" cy="6.5" r="1.9" fill="currentColor" stroke="none"/>',
    '<circle cx="19.5" cy="17.5" r="1.9" fill="currentColor" stroke="none"/>',
  ] },
  "circle": { group: "sketch", accent: [0,1], body: [
    '<circle cx="12" cy="12" r="8"/>',
    '<circle cx="12" cy="12" r="1.5" fill="currentColor" stroke="none"/>',
  ] },
  "radius": { group: "sketch", accent: [1,2], body: [
    '<circle cx="12" cy="12" r="8"/>',
    '<path d="M12 12l5.6-5.6"/>',
    '<circle cx="12" cy="12" r="1.5" fill="currentColor" stroke="none"/>',
  ] },
  "nozzle": { group: "sketch", body: [
    '<path d="M3 6c5 0 6 3.5 9 3.5S17 7 21 5"/>',
    '<path d="M3 18c5 0 6-3.5 9-3.5s5 2.5 9 4.5"/>',
    '<path d="M3 12h18" stroke-dasharray="2 3"/>',
  ] },
  "vertex-add": { group: "sketch", body: [
    '<path d="M4 19.5l5.5-9 5 4.5"/>',
    '<circle cx="9.5" cy="10.5" r="2"/>',
    '<path d="M18 4v6M15 7h6"/>',
  ] },
  "vertex-move": { group: "sketch", body: [
    '<circle cx="12" cy="12" r="2.2"/>',
    '<path d="M12 3v5M12 16v5M3 12h5M16 12h5"/>',
    '<path d="M9.5 5.5 12 3l2.5 2.5M9.5 18.5 12 21l2.5-2.5M5.5 9.5 3 12l2.5 2.5M18.5 9.5 21 12l-2.5 2.5"/>',
  ] },
  "segment-split": { group: "sketch", body: [
    '<path d="M3.5 12h5.5M15 12h5.5"/>',
    '<circle cx="12" cy="12" r="2.5"/>',
    '<path d="M12 4.5v3M12 16.5v3"/>',
  ] },
  "profile-close": { group: "sketch", body: [
    '<path d="M9 19.5H5.5v-11l6.5-4 6.5 4v11H15"/>',
    '<path d="M9 19.5h6" stroke-dasharray="2 3"/>',
    '<circle cx="9" cy="19.5" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="15" cy="19.5" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "dimension": { group: "sketch", accent: [1,2], body: [
    '<path d="M4 5v14M20 5v14"/>',
    '<path d="M4 12h16"/>',
    '<path d="M7.5 9 4.5 12l3 3M16.5 9l3 3-3 3"/>',
  ] },
  "fillet": { group: "sketch", accent: [0], body: [
    '<path d="M5 20v-8a7 7 0 0 1 7-7h8"/>',
    '<path d="M5 10V5h5" stroke-dasharray="2 3"/>',
  ] },
  "revolve": { group: "sketch", body: [
    '<path d="M12 2.5v19" stroke-dasharray="2 3"/>',
    '<path d="M12 6.5h3.5a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H12"/>',
    '<path d="M12 6.5H8.5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2H12" stroke-dasharray="2 3"/>',
  ] },
  "coordinates": { group: "sketch", body: [
    '<path d="M5 4v15h15"/>',
    '<path d="M2.5 6.5 5 4l2.5 2.5M17.5 16.5 20 19l-2.5 2.5"/>',
    '<circle cx="14" cy="10" r="2.1" fill="currentColor" stroke="none"/>',
  ] },
  // --- simulation ---
  "mesh": { group: "simulation", body: [
    '<rect x="4" y="4" width="16" height="16" rx="1"/>',
    '<path d="M4 9.33h16M4 14.67h16M9.33 4v16M14.67 4v16"/>',
  ] },
  "mesh-cut": { group: "simulation", body: [
    '<path d="M4 4v16h16z"/>',
    '<path d="M4 12h8v8M4 16h12M8 8v12"/>',
  ] },
  "boundary": { group: "simulation", body: [
    '<path d="M9 5h10v14H9" stroke-dasharray="2 3"/>',
    '<path d="M5 4v16" stroke-width="3"/>',
  ] },
  "material": { group: "simulation", body: [
    '<path d="M12 3.5l8.5 4.5-8.5 4.5L3.5 8z"/>',
    '<path d="M3.5 12l8.5 4.5 8.5-4.5"/>',
    '<path d="M3.5 16l8.5 4.5 8.5-4.5"/>',
  ] },
  "solver": { group: "simulation", body: [
    '<rect x="6" y="6" width="12" height="12" rx="1.5"/>',
    '<rect x="10" y="10" width="4" height="4" rx="0.5"/>',
    '<path d="M9.5 3v3M14.5 3v3M9.5 18v3M14.5 18v3M3 9.5h3M3 14.5h3M18 9.5h3M18 14.5h3"/>',
  ] },
  "fluent": { group: "simulation", body: [
    '<path d="M3 7.5c3-3 6 3 9 0s6-3 9 0"/>',
    '<path d="M3 12.5c3-3 6 3 9 0s6-3 9 0"/>',
    '<path d="M3 17.5c3-3 6 3 9 0s6-3 9 0"/>',
  ] },
  // --- results ---
  "chart": { group: "results", body: [
    '<path d="M4 4v16h16"/>',
    '<path d="M7.5 15.5l3.5-4 3 2.5 5-6.5"/>',
  ] },
  "histogram": { group: "results", body: [
    '<path d="M3.5 20.5h17"/>',
    '<rect x="5" y="12" width="3.5" height="8.5" rx="0.5"/>',
    '<rect x="10.25" y="5.5" width="3.5" height="15" rx="0.5"/>',
    '<rect x="15.5" y="9" width="3.5" height="11.5" rx="0.5"/>',
  ] },
  "probe": { group: "results", body: [
    '<circle cx="12" cy="12" r="6"/>',
    '<path d="M12 2.5v5M12 16.5v5M2.5 12h5M16.5 12h5"/>',
    '<circle cx="12" cy="12" r="1.4" fill="currentColor" stroke="none"/>',
  ] },
  "slice": { group: "results", body: [
    '<path d="M8 5.5l8-2.5v15.5l-8 2.5z" fill="currentColor" fill-opacity="0.18"/>',
    '<path d="M8 5.5l8-2.5v15.5l-8 2.5z"/>',
    '<path d="M2.5 12h5.5M16 12h5.5"/>',
  ] },
  "vectors": { group: "results", body: [
    '<path d="M3.5 6.5h11"/>',
    '<path d="M11.5 3.5l3 3-3 3"/>',
    '<path d="M3.5 12h17"/>',
    '<path d="M17.5 9l3 3-3 3"/>',
    '<path d="M3.5 17.5h8"/>',
    '<path d="M8.5 14.5l3 3-3 3"/>',
  ] },
  // --- general ---
  "plus": { group: "general", body: [
    '<path d="M12 5v14M5 12h14"/>',
  ] },
  "minus": { group: "general", body: [
    '<path d="M5 12h14"/>',
  ] },
  "chevron-left": { group: "general", body: [
    '<path d="M14.5 6l-6 6 6 6"/>',
  ] },
  "arrow-left": { group: "general", body: [
    '<path d="M20 12H4"/>',
    '<path d="M10 6l-6 6 6 6"/>',
  ] },
  "arrow-right": { group: "general", body: [
    '<path d="M4 12h16"/>',
    '<path d="M14 6l6 6-6 6"/>',
  ] },
  "arrow-up": { group: "general", body: [
    '<path d="M12 20V4"/>',
    '<path d="M6 10l6-6 6 6"/>',
  ] },
  "arrow-down": { group: "general", body: [
    '<path d="M12 4v16"/>',
    '<path d="M6 14l6 6 6-6"/>',
  ] },
  "more-horizontal": { group: "general", body: [
    '<circle cx="5" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="19" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "more-vertical": { group: "general", body: [
    '<circle cx="12" cy="5" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="19" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "menu": { group: "general", body: [
    '<path d="M4 6.5h16M4 12h16M4 17.5h16"/>',
  ] },
  "drag-handle": { group: "general", body: [
    '<circle cx="9" cy="6" r="1.4" fill="currentColor" stroke="none"/>',
    '<circle cx="15" cy="6" r="1.4" fill="currentColor" stroke="none"/>',
    '<circle cx="9" cy="12" r="1.4" fill="currentColor" stroke="none"/>',
    '<circle cx="15" cy="12" r="1.4" fill="currentColor" stroke="none"/>',
    '<circle cx="9" cy="18" r="1.4" fill="currentColor" stroke="none"/>',
    '<circle cx="15" cy="18" r="1.4" fill="currentColor" stroke="none"/>',
  ] },
  "home": { group: "general", body: [
    '<path d="M3.5 11.5l8.5-7.5 8.5 7.5"/>',
    '<path d="M6 9.5V19a1 1 0 0 0 1 1h3.5v-5.5h3V20H17a1 1 0 0 0 1-1V9.5"/>',
  ] },
  "info": { group: "general", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M12 11v5.5"/>',
    '<circle cx="12" cy="7.75" r="1.15" fill="currentColor" stroke="none"/>',
  ] },
  "warning": { group: "general", body: [
    '<path d="M12 4l9 15.5H3z"/>',
    '<path d="M12 10v4.5"/>',
    '<circle cx="12" cy="17" r="1.15" fill="currentColor" stroke="none"/>',
  ] },
  "error": { group: "general", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M9 9l6 6M15 9l-6 6"/>',
  ] },
  "success": { group: "general", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M8 12.5l3 3 5-6"/>',
  ] },
  "help": { group: "general", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M9.5 9.5a2.5 2.5 0 1 1 3.6 2.2c-.7.4-1.1.9-1.1 1.8"/>',
    '<circle cx="12" cy="16.75" r="1.15" fill="currentColor" stroke="none"/>',
  ] },
  "lock": { group: "general", fill: [0], body: [
    '<rect x="5" y="11" width="14" height="9.5" rx="1.5"/>',
    '<path d="M8 11V8a4 4 0 0 1 8 0v3"/>',
  ] },
  "unlock": { group: "general", body: [
    '<rect x="5" y="11" width="14" height="9.5" rx="1.5"/>',
    '<path d="M8 11V8a4 4 0 0 1 7.5-2"/>',
  ] },
  "eye-off": { group: "general", body: [
    '<path d="M3 3l18 18"/>',
    '<path d="M10 5.75a9.6 9.6 0 0 1 2-.25c6 0 9.5 6.5 9.5 6.5a17 17 0 0 1-2.6 3.4"/>',
    '<path d="M6.3 7.3A16.5 16.5 0 0 0 2.5 12S6 18.5 12 18.5a9 9 0 0 0 3.8-.8"/>',
    '<path d="M9.9 9.9a3 3 0 0 0 4.2 4.2"/>',
  ] },
  "star": { group: "general", fill: [0], body: [
    '<path d="M12 3.5l2.6 5.4 5.9.8-4.3 4.1 1.05 5.85L12 16.9l-5.25 2.75 1.05-5.85L3.5 9.7l5.9-.8z"/>',
  ] },
  "heart": { group: "general", fill: [0], body: [
    '<path d="M12 20s-7.5-4.5-7.5-10A4.25 4.25 0 0 1 12 7.3 4.25 4.25 0 0 1 19.5 10c0 5.5-7.5 10-7.5 10z"/>',
  ] },
  "bookmark": { group: "general", fill: [0], body: [
    '<path d="M7 4h10a1 1 0 0 1 1 1v15.5l-6-4-6 4V5a1 1 0 0 1 1-1z"/>',
  ] },
  "flag": { group: "general", fill: [1], body: [
    '<path d="M6 21V4"/>',
    '<path d="M6 4.5h11l-2 4 2 4H6"/>',
  ] },
  "tag": { group: "general", body: [
    '<path d="M4 5.5v6l8.5 8.5a1.5 1.5 0 0 0 2.1 0l5.4-5.4a1.5 1.5 0 0 0 0-2.1L11.5 4h-6A1.5 1.5 0 0 0 4 5.5z"/>',
    '<circle cx="8" cy="8" r="1.3" fill="currentColor" stroke="none"/>',
  ] },
  "pin": { group: "general", fill: [1], body: [
    '<path d="M12 21v-6"/>',
    '<path d="M8 4h8l-1 6 3 3.5V15H6v-1.5L9 10z"/>',
  ] },
  "link": { group: "general", body: [
    '<path d="M10 14a4 4 0 0 0 5.7 0l3-3a4 4 0 0 0-5.7-5.7l-1 1"/>',
    '<path d="M14 10a4 4 0 0 0-5.7 0l-3 3a4 4 0 0 0 5.7 5.7l1-1"/>',
  ] },
  "upload": { group: "general", body: [
    '<path d="M12 15V4"/>',
    '<path d="M7.5 8.5 12 4l4.5 4.5"/>',
    '<path d="M4 16v3a1.5 1.5 0 0 0 1.5 1.5h13A1.5 1.5 0 0 0 20 19v-3"/>',
  ] },
  "share": { group: "general", body: [
    '<circle cx="6" cy="12" r="2.5"/>',
    '<circle cx="18" cy="6" r="2.5"/>',
    '<circle cx="18" cy="18" r="2.5"/>',
    '<path d="M8.2 10.9l7.6-3.8M8.2 13.1l7.6 3.8"/>',
  ] },
  "refresh": { group: "general", body: [
    '<path d="M20 12a8 8 0 0 1-14.3 4.9"/>',
    '<path d="M4 12a8 8 0 0 1 14.3-4.9"/>',
    '<path d="M18.5 3.5v4h-4"/>',
    '<path d="M5.5 20.5v-4h4"/>',
  ] },
  "history": { group: "general", body: [
    '<path d="M4.25 13.5A8 8 0 1 0 6.3 6.4"/>',
    '<path d="M3.5 4.5v4h4"/>',
    '<path d="M12 8v4l2.5 2"/>',
  ] },
  "clock": { group: "general", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M12 7.5V12l3 2"/>',
  ] },
  "calendar": { group: "general", body: [
    '<rect x="4" y="5.5" width="16" height="14.5" rx="1.5"/>',
    '<path d="M4 10h16"/>',
    '<path d="M8 3.5v4M16 3.5v4"/>',
  ] },
  "filter": { group: "general", body: [
    '<path d="M4 5h16l-6 7.5V19l-4 1.5v-8z"/>',
  ] },
  "sort": { group: "general", body: [
    '<path d="M7 5v14"/>',
    '<path d="M3.5 15.5 7 19l3.5-3.5"/>',
    '<path d="M17 19V5"/>',
    '<path d="M13.5 8.5 17 5l3.5 3.5"/>',
  ] },
  "grid-view": { group: "general", fill: [0,1,2,3], body: [
    '<rect x="4" y="4" width="6.5" height="6.5" rx="1"/>',
    '<rect x="13.5" y="4" width="6.5" height="6.5" rx="1"/>',
    '<rect x="4" y="13.5" width="6.5" height="6.5" rx="1"/>',
    '<rect x="13.5" y="13.5" width="6.5" height="6.5" rx="1"/>',
  ] },
  "list-view": { group: "general", body: [
    '<path d="M9 6.5h11M9 12h11M9 17.5h11"/>',
    '<circle cx="4.75" cy="6.5" r="1.3" fill="currentColor" stroke="none"/>',
    '<circle cx="4.75" cy="12" r="1.3" fill="currentColor" stroke="none"/>',
    '<circle cx="4.75" cy="17.5" r="1.3" fill="currentColor" stroke="none"/>',
  ] },
  "sidebar": { group: "general", body: [
    '<rect x="3.5" y="4.5" width="17" height="15" rx="1.5"/>',
    '<path d="M9.5 4.5v15"/>',
  ] },
  "panel-right": { group: "general", body: [
    '<rect x="3.5" y="4.5" width="17" height="15" rx="1.5"/>',
    '<path d="M14.5 4.5v15"/>',
  ] },
  "panel-bottom": { group: "general", body: [
    '<rect x="3.5" y="4.5" width="17" height="15" rx="1.5"/>',
    '<path d="M3.5 14.5h17"/>',
  ] },
  "window": { group: "general", body: [
    '<rect x="3.5" y="4.5" width="17" height="15" rx="1.5"/>',
    '<path d="M3.5 9h17"/>',
    '<circle cx="6.5" cy="6.75" r="0.8" fill="currentColor" stroke="none"/>',
    '<circle cx="9" cy="6.75" r="0.8" fill="currentColor" stroke="none"/>',
  ] },
  // Two corners on one diagonal, no shafts: open outwards to fill the screen, turned to the
  // centre to leave it.
  "fullscreen": { group: "general", body: [
    '<path d="M14 4h6v6"/>',
    '<path d="M10 20H4v-6"/>',
  ] },
  "fullscreen-exit": { group: "general", body: [
    '<path d="M14 4v6h6"/>',
    '<path d="M10 20v-6H4"/>',
  ] },
  "user": { group: "general", fill: [0,1], body: [
    '<circle cx="12" cy="8" r="4"/>',
    '<path d="M4.5 20a7.5 7.5 0 0 1 15 0"/>',
  ] },
  "users": { group: "general", fill: [0,1], body: [
    '<circle cx="9" cy="8.5" r="3.5"/>',
    '<path d="M2.5 19.5a6.5 6.5 0 0 1 13 0"/>',
    '<path d="M16 5.3a3.5 3.5 0 0 1 0 6.4"/>',
    '<path d="M18 14.2a6.5 6.5 0 0 1 3.5 5.3"/>',
  ] },
  "bell": { group: "general", fill: [0], body: [
    '<path d="M6 16.5V11a6 6 0 0 1 12 0v5.5l1.5 2h-15z"/>',
    '<path d="M10 20.5a2 2 0 0 0 4 0"/>',
  ] },
  "file": { group: "general", body: [
    '<path d="M13.5 3h-6A1.5 1.5 0 0 0 6 4.5v15A1.5 1.5 0 0 0 7.5 21h9a1.5 1.5 0 0 0 1.5-1.5v-12z"/>',
    '<path d="M13.5 3v4.5H18"/>',
  ] },
  "file-text": { group: "general", body: [
    '<path d="M13.5 3h-6A1.5 1.5 0 0 0 6 4.5v15A1.5 1.5 0 0 0 7.5 21h9a1.5 1.5 0 0 0 1.5-1.5v-12z"/>',
    '<path d="M13.5 3v4.5H18"/>',
    '<path d="M9 12h6M9 16h4"/>',
  ] },
  "folder": { group: "general", fill: [0], body: [
    '<path d="M3.5 18.5v-12A1.5 1.5 0 0 1 5 5h4l2 2.5h8A1.5 1.5 0 0 1 20.5 9v9.5A1.5 1.5 0 0 1 19 20H5a1.5 1.5 0 0 1-1.5-1.5z"/>',
  ] },
  "folder-plus": { group: "general", body: [
    '<path d="M3.5 18.5v-12A1.5 1.5 0 0 1 5 5h4l2 2.5h8A1.5 1.5 0 0 1 20.5 9v9.5A1.5 1.5 0 0 1 19 20H5a1.5 1.5 0 0 1-1.5-1.5z"/>',
    '<path d="M12 11v5.5M9.25 13.75h5.5"/>',
  ] },
  "print": { group: "general", body: [
    '<path d="M7 8V4h10v4"/>',
    '<path d="M7 16.5H5A1.5 1.5 0 0 1 3.5 15V9.5A1.5 1.5 0 0 1 5 8h14a1.5 1.5 0 0 1 1.5 1.5V15a1.5 1.5 0 0 1-1.5 1.5h-2"/>',
    '<rect x="7" y="13.5" width="10" height="7" rx="0.5"/>',
  ] },
  "cut": { group: "general", body: [
    '<circle cx="6.5" cy="6.5" r="2.5"/>',
    '<circle cx="6.5" cy="17.5" r="2.5"/>',
    '<path d="M8.6 8 20 17.5"/>',
    '<path d="M8.6 16 20 6.5"/>',
  ] },
  "cloud": { group: "general", body: [
    '<path d="M7 18.5a4.5 4.5 0 0 1-.6-8.95A6 6 0 0 1 18 10.5a4 4 0 0 1-.5 8z"/>',
  ] },
  "sun": { group: "general", body: [
    '<circle cx="12" cy="12" r="4"/>',
    '<path d="M12 2.5V5M12 19v2.5M2.5 12H5M19 12h2.5M5.3 5.3l1.75 1.75M16.95 16.95l1.75 1.75M5.3 18.7l1.75-1.75M16.95 7.05l1.75-1.75"/>',
  ] },
  "moon": { group: "general", body: [
    '<path d="M20 14.5A8.5 8.5 0 0 1 9.5 4 8.5 8.5 0 1 0 20 14.5z"/>',
  ] },
  "globe": { group: "general", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M3.5 12h17"/>',
    '<path d="M12 3.5c2.5 2.3 3.8 5.2 3.8 8.5s-1.3 6.2-3.8 8.5c-2.5-2.3-3.8-5.2-3.8-8.5S9.5 5.8 12 3.5z"/>',
  ] },
  "terminal": { group: "general", body: [
    '<rect x="3.5" y="5" width="17" height="14" rx="1.5"/>',
    '<path d="M7 9.5l3 2.5-3 2.5"/>',
    '<path d="M12.5 15h4"/>',
  ] },
  "code": { group: "general", body: [
    '<path d="M8.5 7l-5 5 5 5M15.5 7l5 5-5 5"/>',
  ] },
  "sparkle": { group: "general", body: [
    '<path d="M11 3.5l1.9 5.6 5.6 1.9-5.6 1.9L11 18.5l-1.9-5.6L3.5 11l5.6-1.9z"/>',
    '<path d="M18.5 15.5v5M16 18h5"/>',
  ] },
  "check-square": { group: "general", body: [
    '<rect x="4" y="4" width="16" height="16" rx="2"/>',
    '<path d="M8 12.5l3 3 5-6"/>',
  ] },
  "radio": { group: "general", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<circle cx="12" cy="12" r="3.5" fill="currentColor"/>',
  ] },
  "toggle": { group: "general", body: [
    '<rect x="2.5" y="7" width="19" height="10" rx="5"/>',
    '<circle cx="16" cy="12" r="2.75"/>',
  ] },
  "slider": { group: "general", body: [
    '<path d="M3.5 12H6M12 12h8.5"/>',
    '<circle cx="9" cy="12" r="3"/>',
  ] },
  "shield": { group: "general", body: [
    '<path d="M12 3.5l7 2.5v5.5c0 4.5-3 7.7-7 9-4-1.3-7-4.5-7-9V6z"/>',
  ] },
  "shield-check": { group: "general", body: [
    '<path d="M12 3.5l7 2.5v5.5c0 4.5-3 7.7-7 9-4-1.3-7-4.5-7-9V6z"/>',
    '<path d="M9 12l2.2 2.2 4-4.4"/>',
  ] },
  "key": { group: "general", fill: [0], body: [
    '<circle cx="7.5" cy="16.5" r="3.5"/>',
    '<path d="M10 14l9.5-9.5"/>',
    '<path d="M16.5 7.5 19 10M14 10l2 2"/>',
  ] },
  "power": { group: "general", body: [
    '<path d="M12 3.5v8"/>',
    '<path d="M7 6.5a7.5 7.5 0 1 0 10 0"/>',
  ] },
  "log-out": { group: "general", body: [
    '<path d="M9.5 4.5h-4A1.5 1.5 0 0 0 4 6v12a1.5 1.5 0 0 0 1.5 1.5h4"/>',
    '<path d="M15 7.5l4.5 4.5-4.5 4.5"/>',
    '<path d="M19.5 12h-10"/>',
  ] },
  "monitor": { group: "general", body: [
    '<rect x="3.5" y="4.5" width="17" height="12" rx="1.5"/>',
    '<path d="M12 16.5V20M8 20h8"/>',
  ] },
  "wifi": { group: "general", body: [
    '<path d="M3.5 9.5a12.5 12.5 0 0 1 17 0"/>',
    '<path d="M6.5 13a8 8 0 0 1 11 0"/>',
    '<path d="M9.5 16.5a3.5 3.5 0 0 1 5 0"/>',
    '<circle cx="12" cy="19.5" r="1.2" fill="currentColor" stroke="none"/>',
  ] },
  "battery": { group: "general", body: [
    '<rect x="3.5" y="8" width="15.5" height="8" rx="1.5"/>',
    '<path d="M21.5 10.5v3"/>',
    '<rect x="5.5" y="10" width="8" height="4" rx="0.5" fill="currentColor" stroke="none"/>',
  ] },
  "package": { group: "general", body: [
    '<path d="M12 3l8 4.5v9L12 21l-8-4.5v-9z"/>',
    '<path d="M4 7.5l8 4.5 8-4.5M12 12v9"/>',
    '<path d="M8 5.25l8 4.5v3"/>',
  ] },
  "zoom-out": { group: "general", body: [
    '<circle cx="10.5" cy="10.5" r="6.5"/>',
    '<path d="M15.5 15.5l5 5"/>',
    '<path d="M8 10.5h5"/>',
  ] },
  // --- media ---
  "play": { group: "media", fill: [0], body: [
    '<path d="M8 5.5v13l10.5-6.5z"/>',
  ] },
  "repeat-one": { group: "media", body: [
    '<path d="M17 3.5l3 3-3 3"/>',
    '<path d="M4 11V9.5a3 3 0 0 1 3-3h13"/>',
    '<path d="M7 20.5l-3-3 3-3"/>',
    '<path d="M20 13v1.5a3 3 0 0 1-3 3H4"/>',
    '<path d="M11 11l1.5-1v4.5"/>',
  ] },
  "volume-low": { group: "media", fill: [0], body: [
    '<path d="M4 9.5h3.5l4.5-4v13l-4.5-4H4z"/>',
    '<path d="M15.5 9a4.25 4.25 0 0 1 0 6"/>',
  ] },
  "pause": { group: "media", fill: [0,1], body: [
    '<rect x="6.5" y="5" width="3.5" height="14" rx="1"/>',
    '<rect x="14" y="5" width="3.5" height="14" rx="1"/>',
  ] },
  "skip-next": { group: "media", fill: [0], body: [
    '<path d="M6 5v14l10-7z"/>',
    '<path d="M18 5v14"/>',
  ] },
  "skip-previous": { group: "media", fill: [0], body: [
    '<path d="M18 5v14L8 12z"/>',
    '<path d="M6 5v14"/>',
  ] },
  "fast-forward": { group: "media", fill: [0,1], body: [
    '<path d="M4 6v12l8-6z"/>',
    '<path d="M12 6v12l8-6z"/>',
  ] },
  "rewind": { group: "media", fill: [0,1], body: [
    '<path d="M20 6v12l-8-6z"/>',
    '<path d="M12 6v12l-8-6z"/>',
  ] },
  "record": { group: "media", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<circle cx="12" cy="12" r="4.5" fill="currentColor"/>',
  ] },
  "volume": { group: "media", fill: [0], body: [
    '<path d="M4 9.5h3.5l4.5-4v13l-4.5-4H4z"/>',
    '<path d="M15.5 9a4.25 4.25 0 0 1 0 6"/>',
    '<path d="M18 6.5a8 8 0 0 1 0 11"/>',
  ] },
  "mute": { group: "media", fill: [0], body: [
    '<path d="M4 9.5h3.5l4.5-4v13l-4.5-4H4z"/>',
    '<path d="M16 9.5l5 5M21 9.5l-5 5"/>',
  ] },
  "microphone": { group: "media", fill: [0], body: [
    '<rect x="9" y="3.5" width="6" height="10.5" rx="3"/>',
    '<path d="M5.5 11a6.5 6.5 0 0 0 13 0"/>',
    '<path d="M12 17.5v3"/>',
  ] },
  "headphones": { group: "media", fill: [1,2], body: [
    '<path d="M4.5 14v-2a7.5 7.5 0 0 1 15 0v2"/>',
    '<rect x="3.5" y="13.5" width="4" height="6.5" rx="1.5"/>',
    '<rect x="16.5" y="13.5" width="4" height="6.5" rx="1.5"/>',
  ] },
  "music": { group: "media", fill: [0,1], body: [
    '<circle cx="7.5" cy="17.5" r="2.5"/>',
    '<circle cx="17.5" cy="15.5" r="2.5"/>',
    '<path d="M10 17.5V6l10-2v11.5"/>',
  ] },
  "disc": { group: "media", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<circle cx="12" cy="12" r="2.5"/>',
  ], bold: [
    '<path fill="currentColor" fill-rule="evenodd" d="M12 3.5a8.5 8.5 0 1 1 0 17 8.5 8.5 0 0 1 0-17zM12 9.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5z"/>',
  ] },
  "repeat": { group: "media", body: [
    '<path d="M17 3.5l3 3-3 3"/>',
    '<path d="M4 11V9.5a3 3 0 0 1 3-3h13"/>',
    '<path d="M7 20.5l-3-3 3-3"/>',
    '<path d="M20 13v1.5a3 3 0 0 1-3 3H4"/>',
  ] },
  "shuffle": { group: "media", body: [
    '<path d="M3.5 7H7c5 0 5 10 10 10h3.5"/>',
    '<path d="M3.5 17H7c1.2 0 2.1-.6 2.9-1.4"/>',
    '<path d="M14.1 8.4c.8-.8 1.7-1.4 2.9-1.4h3.5"/>',
    '<path d="M18 4.5 20.5 7 18 9.5"/>',
    '<path d="M18 14.5l2.5 2.5-2.5 2.5"/>',
  ] },
  "playlist": { group: "media", fill: [1], body: [
    '<path d="M4 6.5h12M4 11.5h12M4 16.5h7"/>',
    '<path d="M15.5 14v6l5-3z"/>',
  ] },
  "waveform": { group: "media", body: [
    '<path d="M4 10v4M8 6.5v11M12 3.5v17M16 7.5v9M20 10.5v3"/>',
  ] },
  "film": { group: "media", body: [
    '<rect x="3.5" y="4.5" width="17" height="15" rx="1.5"/>',
    '<path d="M7.5 4.5v15M16.5 4.5v15"/>',
    '<path d="M3.5 9.5h4M3.5 14.5h4M16.5 9.5h4M16.5 14.5h4"/>',
  ] },
  "clapper": { group: "media", body: [
    '<path d="M4 10h16v8.5a1.5 1.5 0 0 1-1.5 1.5h-13A1.5 1.5 0 0 1 4 18.5z"/>',
    '<path d="M4 10l-.5-3L19 3.5l.8 3.2z"/>',
    '<path d="M8 6l2 3.5M13 4.9l2 3.5"/>',
  ] },
  "video-camera": { group: "media", body: [
    '<rect x="3.5" y="6.5" width="12.5" height="11" rx="1.5"/>',
    '<path d="M16 10.5l4.5-2.5v8L16 13.5"/>',
  ] },
  "timeline": { group: "media", body: [
    '<rect x="4" y="5" width="10" height="3.5" rx="1"/>',
    '<rect x="9" y="10.25" width="11" height="3.5" rx="1"/>',
    '<rect x="4" y="15.5" width="7" height="3.5" rx="1"/>',
  ] },
  "split-clip": { group: "media", body: [
    '<path d="M12 3.5v17" stroke-dasharray="2 3"/>',
    '<rect x="3.5" y="8" width="6" height="8" rx="1"/>',
    '<rect x="14.5" y="8" width="6" height="8" rx="1"/>',
  ] },
  "keyframe": { group: "media", body: [
    '<path d="M12 5l7 7-7 7-7-7z"/>',
  ] },
  "speed": { group: "media", body: [
    '<path d="M4.5 16.5a8 8 0 1 1 15 0"/>',
    '<path d="M12 14.5l4-5.5"/>',
    '<circle cx="12" cy="14.5" r="1.5" fill="currentColor" stroke="none"/>',
  ] },
  "subtitles": { group: "media", body: [
    '<rect x="3.5" y="5.5" width="17" height="13" rx="1.5"/>',
    '<path d="M7 12h3.5M13.5 12H17M7 15.5h7"/>',
  ] },
  "transition": { group: "media", body: [
    '<rect x="3.5" y="6" width="8" height="12" rx="1"/>',
    '<path d="M12.5 6H19a1.5 1.5 0 0 1 1.5 1.5v9A1.5 1.5 0 0 1 19 18h-6.5" stroke-dasharray="2 3"/>',
  ] },
  // --- text ---
  "bold": { group: "text", body: [
    '<path d="M7.5 4.5H13a3.5 3.5 0 0 1 0 7H7.5z"/>',
    '<path d="M7.5 11.5H14a4 4 0 0 1 0 8H7.5z"/>',
  ] },
  "italic": { group: "text", body: [
    '<path d="M10.5 4.5h7M6.5 19.5h7M14.5 4.5l-5 15"/>',
  ] },
  "underline": { group: "text", body: [
    '<path d="M7 4.5V11a5 5 0 0 0 10 0V4.5"/>',
    '<path d="M5.5 20h13"/>',
  ] },
  "strikethrough": { group: "text", body: [
    '<path d="M4 12h16"/>',
    '<path d="M16.5 7.5c-.5-1.8-2.2-3-4.5-3-2.6 0-4.5 1.3-4.5 3.3 0 1.2.7 2.1 1.9 2.7"/>',
    '<path d="M7.5 16c.4 2 2.2 3.5 4.7 3.5 2.7 0 4.6-1.4 4.6-3.5 0-.6-.15-1.1-.4-1.5"/>',
  ] },
  "text": { group: "text", body: [
    '<path d="M5 6.5V5h14v1.5"/>',
    '<path d="M12 5v14.5"/>',
    '<path d="M9.5 19.5h5"/>',
  ] },
  "heading": { group: "text", body: [
    '<path d="M5 4.5v15M14 4.5v15M5 12h9"/>',
    '<path d="M17.5 13.5l2-1v7"/>',
  ] },
  "font-size": { group: "text", body: [
    '<path d="M3.5 19l5-13.5 5 13.5M5.3 14.5h6.4"/>',
    '<path d="M14.5 19l3-8 3 8M15.6 16.25h3.8"/>',
  ] },
  "text-color": { group: "text", body: [
    '<path d="M6.5 15.5 12 4l5.5 11.5M8.4 11.5h7.2"/>',
    '<path d="M5 20h14" stroke-width="3"/>',
  ] },
  "highlight": { group: "text", body: [
    '<path d="M14.5 4.5l5 5L11 18H6v-5z"/>',
    '<path d="M4 21h8"/>',
  ] },
  "paragraph": { group: "text", body: [
    '<path d="M17.5 4.5H10a4 4 0 0 0 0 8h2.5"/>',
    '<path d="M12.5 4.5v15M16 4.5v15"/>',
  ] },
  "align-left": { group: "text", body: [
    '<path d="M4 5h16M4 9.75h10M4 14.5h16M4 19.25h10"/>',
  ] },
  "align-center": { group: "text", body: [
    '<path d="M4 5h16M7 9.75h10M4 14.5h16M7 19.25h10"/>',
  ] },
  "align-right": { group: "text", body: [
    '<path d="M4 5h16M10 9.75h10M4 14.5h16M10 19.25h10"/>',
  ] },
  "align-justify": { group: "text", body: [
    '<path d="M4 5h16M4 9.75h16M4 14.5h16M4 19.25h16"/>',
  ] },
  "list-numbers": { group: "text", body: [
    '<path d="M10 6.5h10M10 12h10M10 17.5h10"/>',
    '<path d="M4.5 5 6 4.25V9"/>',
    '<path d="M4.25 15.5a1.5 1.5 0 1 1 2.6 1l-2.6 3h3"/>',
  ] },
  "indent": { group: "text", body: [
    '<path d="M4 5h16M11 9.75h9M11 14.5h9M4 19.25h16"/>',
    '<path d="M4 9.5l3.5 2.6L4 14.75"/>',
  ] },
  "outdent": { group: "text", body: [
    '<path d="M4 5h16M11 9.75h9M11 14.5h9M4 19.25h16"/>',
    '<path d="M7.5 9.5 4 12.1l3.5 2.65"/>',
  ] },
  "quote": { group: "text", body: [
    '<path d="M4.5 17.5V13A5.5 5.5 0 0 1 10 7.5"/>',
    '<path d="M4.5 13H9v4.5H4.5z"/>',
    '<path d="M14 17.5V13a5.5 5.5 0 0 1 5.5-5.5"/>',
    '<path d="M14 13h4.5v4.5H14z"/>',
  ] },
  "comment": { group: "text", body: [
    '<path d="M5.5 4.5h13A1.5 1.5 0 0 1 20 6v9a1.5 1.5 0 0 1-1.5 1.5H11L6.5 20v-3.5h-1A1.5 1.5 0 0 1 4 15V6a1.5 1.5 0 0 1 1.5-1.5z"/>',
  ] },
  "spellcheck": { group: "text", body: [
    '<path d="M4 15.5l4-10 4 10M5.4 12h5.2"/>',
    '<path d="M13 17.5l2.5 2.5 5-5.5"/>',
  ] },
  "table": { group: "text", body: [
    '<rect x="3.5" y="5" width="17" height="14" rx="1.5"/>',
    '<path d="M3.5 10h17M3.5 14.5h17M9.5 5v14M15 5v14"/>',
  ] },
  "image": { group: "text", body: [
    '<rect x="3.5" y="5" width="17" height="14" rx="1.5"/>',
    '<circle cx="8.5" cy="9.5" r="1.5"/>',
    '<path d="M3.5 16 8 12l4 3.5 3-2.5 5.5 4.5"/>',
  ] },
  "page-break": { group: "text", body: [
    '<path d="M4 12h3M10.5 12h3M17 12h3"/>',
    '<path d="M6 8.5V5a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v3.5"/>',
    '<path d="M6 15.5V19a1 1 0 0 0 1 1h10a1 1 0 0 0 1-1v-3.5"/>',
  ] },
  "columns": { group: "text", body: [
    '<rect x="4" y="4.5" width="6.5" height="15" rx="1"/>',
    '<rect x="13.5" y="4.5" width="6.5" height="15" rx="1"/>',
  ] },
  "text-box": { group: "text", body: [
    '<rect x="3.5" y="5.5" width="17" height="13" rx="1"/>',
    '<path d="M8 9.5h8M12 9.5v6"/>',
  ] },
  // --- data ---
  "sum": { group: "data", body: [
    '<path d="M17.5 6.5V5h-11l6 7-6 7h11v-1.5"/>',
  ] },
  "hash": { group: "data", body: [
    '<path d="M9.5 4l-2 16M16.5 4l-2 16M4.5 9h16M3.5 15h16"/>',
  ] },
  "percent": { group: "data", body: [
    '<circle cx="7" cy="7" r="2.5"/>',
    '<circle cx="17" cy="17" r="2.5"/>',
    '<path d="M18.5 5.5l-13 13"/>',
  ] },
  "row-insert": { group: "data", body: [
    '<rect x="3.5" y="13" width="17" height="6.5" rx="1"/>',
    '<path d="M12 3.5v6M9 6.5h6"/>',
  ] },
  "column-insert": { group: "data", body: [
    '<rect x="13" y="3.5" width="6.5" height="17" rx="1"/>',
    '<path d="M3.5 12h6M6.5 9v6"/>',
  ] },
  "merge-cells": { group: "data", body: [
    '<rect x="3.5" y="5" width="17" height="14" rx="1.5"/>',
    '<path d="M12 5v3M12 16v3"/>',
    '<path d="M7 12h10"/>',
    '<path d="M9 10l-2 2 2 2M15 10l2 2-2 2"/>',
  ] },
  "chart-pie": { group: "data", body: [
    '<path d="M12 3.5V12h8.5A8.5 8.5 0 1 1 12 3.5z"/>',
    '<path d="M15 3.8A8.5 8.5 0 0 1 20.2 9H15z"/>',
  ] },
  "calculator": { group: "data", body: [
    '<rect x="5" y="3.5" width="14" height="17" rx="1.5"/>',
    '<rect x="8" y="6.5" width="8" height="3.5" rx="0.5"/>',
    '<circle cx="8.75" cy="13.5" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="13.5" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="15.25" cy="13.5" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="8.75" cy="17" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="17" r="1.1" fill="currentColor" stroke="none"/>',
    '<circle cx="15.25" cy="17" r="1.1" fill="currentColor" stroke="none"/>',
  ] },
  "database": { group: "data", body: [
    '<path d="M4.5 6.5c0-1.7 3.4-3 7.5-3s7.5 1.3 7.5 3-3.4 3-7.5 3-7.5-1.3-7.5-3z"/>',
    '<path d="M4.5 6.5v11c0 1.7 3.4 3 7.5 3s7.5-1.3 7.5-3v-11"/>',
    '<path d="M4.5 12c0 1.7 3.4 3 7.5 3s7.5-1.3 7.5-3"/>',
  ] },
  "form": { group: "data", body: [
    '<rect x="4" y="4.5" width="5" height="5" rx="1"/>',
    '<rect x="4" y="14.5" width="5" height="5" rx="1"/>',
    '<path d="M12.5 7H20M12.5 17H20"/>',
  ] },
  // --- slides ---
  "slide": { group: "slides", body: [
    '<rect x="3.5" y="5" width="17" height="14" rx="1.5"/>',
    '<path d="M7.5 9.5h9M7.5 13h5"/>',
  ] },
  "present": { group: "slides", body: [
    '<rect x="3.5" y="4.5" width="17" height="12" rx="1.5"/>',
    '<path d="M12 16.5V20M8 20h8"/>',
    '<path d="M10.5 8v5l4-2.5z"/>',
  ] },
  "shapes": { group: "slides", body: [
    '<circle cx="8" cy="15.5" r="4.5"/>',
    '<rect x="11.5" y="4" width="8.5" height="8.5" rx="1"/>',
  ] },
  "layout": { group: "slides", body: [
    '<rect x="3.5" y="4.5" width="17" height="15" rx="1.5"/>',
    '<path d="M3.5 9.5h17M10 9.5v10"/>',
  ] },
  // --- design ---
  "pen": { group: "design", body: [
    '<path d="M12 3.5l5 9-2.5 6h-5L7 12.5z"/>',
    '<path d="M12 3.5v8.5"/>',
    '<circle cx="12" cy="13.5" r="1.5"/>',
    '<path d="M8.5 21h7"/>',
  ] },
  "bezier": { group: "design", body: [
    '<path d="M4 18C4 9 20 15 20 6"/>',
    '<circle cx="4" cy="18" r="1.9" fill="currentColor" stroke="none"/>',
    '<circle cx="20" cy="6" r="1.9" fill="currentColor" stroke="none"/>',
    '<path d="M4 18h6M20 6h-6" stroke-dasharray="2 3"/>',
  ] },
  "brush": { group: "design", body: [
    '<path d="M19.5 4.5a2 2 0 0 0-2.9 0L9 12.5l2.5 2.5 8-7.6a2 2 0 0 0 0-2.9z"/>',
    '<path d="M9 12.5c-2.5 0-3.5 1.5-3.5 3.5 0 1.5-1 2.5-2 3 1 1 2.5 1.5 4 1.5 2.5 0 4-1.5 4-4z"/>',
  ] },
  "eraser": { group: "design", body: [
    '<path d="M9 19.5 4.5 15a1.5 1.5 0 0 1 0-2.1l7.9-7.9a1.5 1.5 0 0 1 2.1 0l5 5a1.5 1.5 0 0 1 0 2.1L12 19.5z"/>',
    '<path d="M9 19.5h11"/>',
    '<path d="M9.5 8.5l6 6"/>',
  ] },
  "fill": { group: "design", body: [
    '<path d="M11 4l8 8-6.4 6.4a1.5 1.5 0 0 1-2.1 0l-5.9-5.9a1.5 1.5 0 0 1 0-2.1z"/>',
    '<path d="M4.6 12H19"/>',
    '<path d="M20.5 16.5s-1.5 1.8-1.5 2.8a1.5 1.5 0 0 0 3 0c0-1-1.5-2.8-1.5-2.8z"/>',
  ] },
  "eyedropper": { group: "design", body: [
    '<path d="M17 4.5A1.8 1.8 0 0 1 19.5 7L17 9.5 14.5 7z"/>',
    '<path d="M13 6l5 5"/>',
    '<path d="M15 9l-8 8-3 .5 .5-3 8-8"/>',
  ] },
  "crop": { group: "design", body: [
    '<path d="M7 3v12.5A1.5 1.5 0 0 0 8.5 17H21"/>',
    '<path d="M3 7h12.5A1.5 1.5 0 0 1 17 8.5V21"/>',
  ] },
  "rotate": { group: "design", body: [
    '<path d="M19.5 12A7.5 7.5 0 1 1 12 4.5h4"/>',
    '<path d="M13.5 2 16 4.5 13.5 7"/>',
  ] },
  "flip": { group: "design", accent: [2], body: [
    '<path d="M12 3.5v17" stroke-dasharray="2 3"/>',
    '<path d="M9 7 3.5 17H9z"/>',
    '<path d="M15 7l5.5 10H15z" fill="currentColor" fill-opacity="0.18"/>',
  ] },
  "group": { group: "design", body: [
    '<rect x="4" y="4" width="9" height="9" rx="1"/>',
    '<rect x="11" y="11" width="9" height="9" rx="1"/>',
  ] },
  "align-objects-left": { group: "design", body: [
    '<path d="M4 3.5v17"/>',
    '<rect x="7" y="6" width="10" height="4.5" rx="1"/>',
    '<rect x="7" y="13.5" width="6" height="4.5" rx="1"/>',
  ] },
  "align-objects-center": { group: "design", body: [
    '<path d="M12 3.5v17"/>',
    '<rect x="6" y="6" width="12" height="4.5" rx="1"/>',
    '<rect x="8.5" y="13.5" width="7" height="4.5" rx="1"/>',
  ] },
  "distribute": { group: "design", body: [
    '<path d="M4 3.5v17M20 3.5v17"/>',
    '<rect x="9.5" y="7" width="5" height="10" rx="1"/>',
  ] },
  "gradient": { group: "design", body: [
    '<rect x="4" y="4" width="16" height="16" rx="1.5"/>',
    '<path d="M19.5 4.5v15h-15z" fill="currentColor" fill-opacity="0.18" stroke="none"/>',
  ] },
  "palette": { group: "design", body: [
    '<path d="M12 3.5a8.5 8.5 0 1 0 0 17c1.2 0 2-.8 2-1.8 0-.5-.2-.9-.5-1.3-.300-.3-.5-.800-.5-1.2 0-1 .800-1.7 1.8-1.7H17a3.5 3.5 0 0 0 3.5-3.5c0-4.1-3.8-7.5-8.5-7.5z"/>',
    '<circle cx="7.5" cy="11.5" r="1.15" fill="currentColor" stroke="none"/>',
    '<circle cx="10" cy="7.5" r="1.15" fill="currentColor" stroke="none"/>',
    '<circle cx="14.5" cy="7.5" r="1.15" fill="currentColor" stroke="none"/>',
  ] },
  "polygon": { group: "design", accent: [0], body: [
    '<path d="M12 3.5l8 5.8-3 9.7H7l-3-9.7z"/>',
  ] },
  "ruler": { group: "design", body: [
    '<path d="M4 15.5 15.5 4 20 8.5 8.5 20z"/>',
    '<path d="M8 13l1.5 1.5M11 10l1.5 1.5M14 7l1.5 1.5"/>',
  ] },
  "magic-wand": { group: "design", body: [
    '<path d="M4.5 19.5l10-10 1.5 1.5-10 10z"/>',
    '<path d="M18 3.5v3M16.5 5h3"/>',
    '<path d="M20 10.5v2M19 11.5h2"/>',
    '<path d="M11 3.5v2M10 4.5h2"/>',
  ] },
  "lasso": { group: "design", body: [
    '<path d="M8.5 14.9C5.8 14 4 12.2 4 10c0-3 3.6-5.5 8-5.5s8 2.5 8 5.5-3.6 5.5-8 5.5c-.6 0-1.2 0-1.8-.15"/>',
    '<circle cx="8.5" cy="15.25" r="1.75"/>',
    '<path d="M8.5 17c0 1.5-.5 2.5-1.5 3.5"/>',
  ] },
  "marquee": { group: "design", body: [
    '<rect x="4" y="4" width="16" height="16" rx="1" stroke-dasharray="2 3"/>',
  ] },
  "transform": { group: "design", body: [
    '<rect x="6" y="6" width="12" height="12" rx="0.5"/>',
    '<circle cx="6" cy="6" r="1.7" fill="currentColor" stroke="none"/>',
    '<circle cx="18" cy="6" r="1.7" fill="currentColor" stroke="none"/>',
    '<circle cx="6" cy="18" r="1.7" fill="currentColor" stroke="none"/>',
    '<circle cx="18" cy="18" r="1.7" fill="currentColor" stroke="none"/>',
  ] },
  // --- photo ---
  "exposure": { group: "photo", body: [
    '<rect x="4" y="4" width="16" height="16" rx="1.5"/>',
    '<path d="M4.5 19.5l15-15"/>',
    '<path d="M7 8.5h3.5M8.75 6.75v3.5"/>',
    '<path d="M13.5 15.5H17"/>',
  ] },
  "contrast": { group: "photo", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M12 3.5a8.5 8.5 0 0 1 0 17z" fill="currentColor"/>',
  ] },
  "droplet": { group: "photo", body: [
    '<path d="M12 3.5s6 6.2 6 10.5a6 6 0 0 1-12 0c0-4.3 6-10.5 6-10.5z"/>',
  ] },
  "temperature": { group: "photo", body: [
    '<path d="M10 13.5v-8a2 2 0 0 1 4 0v8a4 4 0 1 1-4 0z"/>',
    '<path d="M12 9v7"/>',
    '<circle cx="12" cy="16.75" r="1.5" fill="currentColor" stroke="none"/>',
  ] },
  "flash": { group: "photo", body: [
    '<path d="M13 3.5 5.5 13.5H11l-1 7 7.5-10H12z"/>',
  ] },
  "focus": { group: "photo", body: [
    '<path d="M4 9V5.5A1.5 1.5 0 0 1 5.5 4H9M15 4h3.5A1.5 1.5 0 0 1 20 5.5V9M20 15v3.5a1.5 1.5 0 0 1-1.5 1.5H15M9 20H5.5A1.5 1.5 0 0 1 4 18.5V15"/>',
    '<circle cx="12" cy="12" r="2.5"/>',
  ] },
  "compare": { group: "photo", body: [
    '<rect x="3.5" y="5" width="17" height="14" rx="1.5"/>',
    '<path d="M12 3v18"/>',
    '<path d="M3.5 16 8 12l4 3.5"/>',
  ] },
  // --- mail ---
  "mail": { group: "mail", body: [
    '<rect x="3.5" y="5.5" width="17" height="13" rx="1.5"/>',
    '<path d="M4 7l8 6.5L20 7"/>',
  ] },
  "inbox": { group: "mail", body: [
    '<path d="M3.5 13.5l3-8.5h11l3 8.5V18a1.5 1.5 0 0 1-1.5 1.5H5A1.5 1.5 0 0 1 3.5 18z"/>',
    '<path d="M3.5 13.5h5l1 2.5h5l1-2.5h5"/>',
  ] },
  "send": { group: "mail", body: [
    '<path d="M20.5 3.5 10 14"/>',
    '<path d="M20.5 3.5 14 20.5 10 14 3.5 10z"/>',
  ] },
  "reply": { group: "mail", body: [
    '<path d="M9.5 6 4 11.5 9.5 17"/>',
    '<path d="M4 11.5h9a7 7 0 0 1 7 7v.5"/>',
  ] },
  "reply-all": { group: "mail", body: [
    '<path d="M12.5 6 7 11.5l5.5 5.5"/>',
    '<path d="M7 11.5h7a6 6 0 0 1 6 6v1"/>',
    '<path d="M7.5 6 2 11.5 7.5 17"/>',
  ] },
  "forward": { group: "mail", body: [
    '<path d="M14.5 6l5.5 5.5-5.5 5.5"/>',
    '<path d="M20 11.5h-9a7 7 0 0 0-7 7v.5"/>',
  ] },
  "attach": { group: "mail", body: [
    '<path d="M19.5 11.5l-7.8 7.8a4.5 4.5 0 0 1-6.4-6.4l8.3-8.3a3 3 0 0 1 4.3 4.3l-8.3 8.2a1.5 1.5 0 0 1-2.1-2.1l7.5-7.5"/>',
  ] },
  "archive": { group: "mail", body: [
    '<rect x="3.5" y="4.5" width="17" height="4.5" rx="1"/>',
    '<path d="M5 9v9.5A1.5 1.5 0 0 0 6.5 20h11a1.5 1.5 0 0 0 1.5-1.5V9"/>',
    '<path d="M10 13h4"/>',
  ] },
  "at": { group: "mail", body: [
    '<circle cx="12" cy="12" r="3.5"/>',
    '<path d="M15.5 8.5V13a2.5 2.5 0 0 0 5 0v-1a8.5 8.5 0 1 0-3.5 6.9"/>',
  ] },
  // --- model ---
  "extrude": { group: "model", body: [
    '<path d="M5 15l7-3 7 3-7 3z"/>',
    '<path d="M12 12V4"/>',
    '<path d="M9 7l3-3 3 3"/>',
    '<path d="M5 15v3l7 3 7-3v-3"/>',
  ] },
  "cylinder": { group: "model", body: [
    '<path d="M5 6.5c0-1.7 3.1-3 7-3s7 1.3 7 3-3.1 3-7 3-7-1.3-7-3z"/>',
    '<path d="M5 6.5v11c0 1.7 3.1 3 7 3s7-1.3 7-3v-11"/>',
  ] },
  "sphere": { group: "model", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<path d="M3.5 12c0 1.7 3.8 3 8.5 3s8.5-1.3 8.5-3"/>',
    '<path d="M12 3.5c-1.7 0-3 3.8-3 8.5s1.3 8.5 3 8.5"/>',
  ] },
  "plane": { group: "model", body: [
    '<path d="M3.5 16l5-8.5h12l-5 8.5z"/>',
  ] },
  "axis": { group: "model", body: [
    '<path d="M12 12V3.5M12 12l7.5 4.5M12 12l-7.5 4.5"/>',
    '<circle cx="12" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "angle": { group: "model", accent: [2], body: [
    '<path d="M4.5 19.5h15"/>',
    '<path d="M4.5 19.5 15 5"/>',
    '<path d="M11.5 19.5a7 7 0 0 0-2.9-5.7"/>',
  ] },
  "chamfer": { group: "model", accent: [0], body: [
    '<path d="M5 20v-9l6-6h9"/>',
  ] },
  "hole": { group: "model", body: [
    '<circle cx="12" cy="12" r="8.5"/>',
    '<circle cx="12" cy="12" r="3.5"/>',
  ] },
  "pattern": { group: "model", accent: [1,2,3,4,5,6,7,8], body: [
    '<circle cx="6" cy="6" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="6" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="18" cy="6" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="6" cy="12" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="12" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="18" cy="12" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="6" cy="18" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="18" r="1.5" fill="currentColor" stroke="none"/>',
    '<circle cx="18" cy="18" r="1.5" fill="currentColor" stroke="none"/>',
  ] },
  "union": { group: "model", body: [
    '<circle cx="9" cy="12" r="5.5" fill="currentColor" fill-opacity="0.18"/>',
    '<circle cx="15" cy="12" r="5.5" fill="currentColor" fill-opacity="0.18"/>',
  ] },
  "subtract": { group: "model", body: [
    '<circle cx="9" cy="12" r="5.5" fill="currentColor" fill-opacity="0.18"/>',
    '<circle cx="15" cy="12" r="5.5" stroke-dasharray="2 3"/>',
  ] },
  "intersect": { group: "model", body: [
    '<circle cx="9" cy="12" r="5.5"/>',
    '<circle cx="15" cy="12" r="5.5"/>',
    '<path d="M12 7.4a5.5 5.5 0 0 1 0 9.2 5.5 5.5 0 0 1 0-9.2z" fill="currentColor" fill-opacity="0.18" stroke="none"/>',
  ] },
  "magnet": { group: "model", body: [
    '<path d="M6 4.5V12a6 6 0 0 0 12 0V4.5h-4V12a2 2 0 0 1-4 0V4.5z"/>',
    '<path d="M6 8h4M14 8h4"/>',
  ] },
  // --- document ---
  "pages": { group: "document", body: [
    '<path d="M9 3.5h6.5l4 4V17a1.5 1.5 0 0 1-1.5 1.5H9A1.5 1.5 0 0 1 7.5 17V5A1.5 1.5 0 0 1 9 3.5z"/>',
    '<path d="M15.5 3.5v4h4"/>',
    '<path d="M4.5 7.5V19A1.5 1.5 0 0 0 6 20.5h8.5"/>',
  ] },
  "signature": { group: "document", body: [
    '<path d="M3.5 16.5c2.5-6 4.5-9 6-9s.5 4-1 7c2-3 3.5-4.5 4.5-4.5s0 3 1 3 2-1.5 3-1.5 1.5 1.5 3.5 1.5"/>',
    '<path d="M3.5 20h17"/>',
  ] },
  "stamp": { group: "document", body: [
    '<path d="M9.5 4.5h5l-.75 7H17.5a1.5 1.5 0 0 1 1.5 1.5v3H5v-3a1.5 1.5 0 0 1 1.5-1.5h3.75z"/>',
    '<path d="M5 20h14"/>',
  ] },
  "fit-width": { group: "document", body: [
    '<path d="M3.5 4v16M20.5 4v16"/>',
    '<path d="M7 12h10"/>',
    '<path d="M9.5 9.5 7 12l2.5 2.5M14.5 9.5 17 12l-2.5 2.5"/>',
  ] },
  "margins": { group: "document", body: [
    '<rect x="4.5" y="3.5" width="15" height="17" rx="1"/>',
    '<rect x="7.5" y="6.5" width="9" height="11" rx="0" stroke-dasharray="2 3"/>',
  ] },
  "book": { group: "document", body: [
    '<path d="M12 6.5C10 5 7.5 4.5 4 4.5V18c3.5 0 6 .5 8 2 2-1.5 4.5-2 8-2V4.5c-3.5 0-6 .5-8 2z"/>',
    '<path d="M12 6.5V20"/>',
  ] },
  // --- window ---
  // Window glyphs are drawn for 1:1 at 24 px: 2 px strokes centred on whole pixels, so every
  // straight edge lands on the pixel grid.
  "window-minimize": { group: "general", body: [
    '<path d="M8 12h8" stroke-width="2"/>',
  ] },
  // The fullscreen corners on the window glyphs' grid: open outwards to maximise, turned to the
  // centre to restore.
  "window-maximize": { group: "general", body: [
    '<path d="M12 7h5v5M12 17H7v-5" stroke-width="2"/>',
  ] },
  "window-restore": { group: "general", body: [
    '<path d="M14 7v3h3M10 17v-3H7" stroke-width="2"/>',
  ] },
  "window-close": { group: "general", body: [
    '<path d="M8 8l8 8M16 8l-8 8" stroke-width="2"/>',
  ] },
  "offline": { group: "general", body: [
    '<path d="M3.5 9.5a12.5 12.5 0 0 1 4.25-2.75"/>',
    '<path d="M12.5 5.5a12.5 12.5 0 0 1 8 4"/>',
    '<path d="M6.5 13a8 8 0 0 1 3-1.75"/>',
    '<path d="M15.25 11.75a8 8 0 0 1 2.25 1.25"/>',
    '<path d="M9.5 16.5a3.5 3.5 0 0 1 5 0"/>',
    '<path d="M4 4l16 16"/>',
  ] },
  // --- cad: drafting, dimensions, layers and blocks; `accent` lists the parts a CAD app tints ---
  "add-leader": { group: "cad", accent: [0,1,3], body: [
    '<path d="M3 20L10 9M10 9L14 9"/>',
    '<path d="M3 20L3.22 16.7L5.67 18.05Z" fill="currentColor" stroke="none"/>',
    '<path d="M15 6L22 6M15 10L21 10"/>',
    '<path d="M19 15.5v5M16.5 18h5"/>',
  ] },
  "align-leaders": { group: "cad", accent: [0,1], body: [
    '<path d="M3 20L10 9M10 9L14 9"/>',
    '<path d="M3 20L3.22 16.7L5.67 18.05Z" fill="currentColor" stroke="none"/>',
    '<path d="M15 6L22 6M15 10L21 10"/>',
  ] },
  "annotation": { group: "cad", body: [
    '<path d="M12 3L21 20L3 20ZM12 9L12 15"/>',
    '<circle cx="12" cy="17.5" r="1" fill="currentColor" stroke="none"/>',
  ] },
  "arc": { group: "cad", accent: [0,1,2,3], body: [
    '<path d="M20.86 14.44A9 9 0 0 0 3.14 14.44"/>',
    '<circle cx="3.2" cy="14.5" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="7" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="20.8" cy="14.5" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "area": { group: "cad", accent: [0], body: [
    '<path d="M4 18L6 5L19 7L20 19Z"/>',
    '<path d="M9.5 15.5 12 9.5l2.5 6M10.4 13.5h3.2"/>',
  ] },
  "array-polar": { group: "cad", accent: [0,2,3,4,5,6], body: [
    '<circle cx="12" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="19.5" cy="12" r="2"/>',
    '<circle cx="15.75" cy="18.5" r="2"/>',
    '<circle cx="8.25" cy="18.5" r="2"/>',
    '<circle cx="4.5" cy="12" r="2"/>',
    '<circle cx="8.25" cy="5.5" r="2"/>',
    '<circle cx="15.75" cy="5.5" r="2"/>',
  ] },
  "att-def": { group: "cad", accent: [1], body: [
    '<path d="M9 4L17 8L9 12L1 8ZM1 8L1 15L9 19L17 15L17 8"/>',
    '<path d="M16 22.5l3-7 3 7M17.1 20h3.8"/>',
  ] },
  "block-create": { group: "cad", accent: [1], body: [
    '<path d="M9 4L17 8L9 12L1 8ZM1 8L1 15L9 19L17 15L17 8"/>',
    '<path d="M19 2v6M16.4 3.5l5.2 3M16.4 6.5l5.2-3"/>',
  ] },
  "block-edit": { group: "cad", accent: [1], body: [
    '<path d="M9 4L17 8L9 12L1 8ZM1 8L1 15L9 19L17 15L17 8"/>',
    '<path d="M14 22L22 14"/>',
  ] },
  "blocks": { group: "cad", body: [
    '<path d="M12 3L20 7L12 11L4 7ZM4 7L4 16L12 20L20 16L20 7M12 11L12 20"/>',
  ] },
  "break": { group: "cad", accent: [1,2], body: [
    '<path d="M3 12L9 12M15 12L21 12"/>',
    '<circle cx="9" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="15" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "bulb": { group: "cad", accent: [0,1], body: [
    '<circle cx="12" cy="10" r="6" fill="currentColor" stroke="none"/>',
    '<rect x="9.5" y="15" width="5" height="4" fill="currentColor" stroke="none"/>',
  ] },
  "bulb-off": { group: "cad", accent: [0,1], body: [
    '<circle cx="12" cy="10" r="6" fill="currentColor" stroke="none"/>',
    '<rect x="9.5" y="15" width="5" height="4" fill="currentColor" stroke="none"/>',
  ] },
  "closed-boundary": { group: "cad", accent: [2,3], body: [
    '<path d="M4 4L20 4L20 20L4 20Z"/>',
    '<circle cx="12" cy="12" r="6"/>',
    '<path d="M4 4L20 4L20 20L4 20Z"/>',
    '<circle cx="6" cy="6" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "coincident": { group: "cad", accent: [1], body: [
    '<path d="M3 20L12 12M12 12L21 18"/>',
    '<circle cx="12" cy="12" r="2.5" fill="currentColor" stroke="none"/>',
  ] },
  "collinear": { group: "cad", accent: [1], body: [
    '<path d="M3 19L9 15"/>',
    '<path d="M12 13L21 7"/>',
  ] },
  "concentric": { group: "cad", accent: [1], body: [
    '<circle cx="12" cy="12" r="8"/>',
    '<circle cx="12" cy="12" r="4"/>',
  ] },
  "constraint": { group: "cad", accent: [0,1], body: [
    '<rect x="7" y="11" width="10" height="9" fill="currentColor" stroke="none"/>',
    '<path d="M16 11A4 4 0 0 0 8 11"/>',
  ] },
  "del-constraint": { group: "cad", accent: [0,1], body: [
    '<rect x="3" y="12" width="9" height="8" fill="currentColor" stroke="none"/>',
    '<path d="M11 12A3.5 3.5 0 1 0 4 12"/>',
    '<path d="M15 13L21 19M21 13L15 19"/>',
  ] },
  "dim-aligned": { group: "cad", accent: [1,2,3], body: [
    '<path d="M5.78 20.62L2.86 7.95M21.37 17.02L18.45 4.35"/>',
    '<path d="M3.53 10.88L19.12 7.28"/>',
    '<path d="M3.53 10.88L6.14 8.84L6.77 11.57Z" fill="currentColor" stroke="none"/>',
    '<path d="M19.12 7.28L16.51 9.32L15.88 6.59Z" fill="currentColor" stroke="none"/>',
  ] },
  "dim-arc": { group: "cad", accent: [1], body: [
    '<path d="M19.52 15.26A8 8 0 0 0 4.48 15.26"/>',
    '<path d="M22.88 12.93A12 12 0 0 0 1.12 12.93"/>',
  ] },
  "dim-baseline": { group: "cad", accent: [1,2,3,4], body: [
    '<path d="M4 18L4 5M20 18L20 5"/>',
    '<path d="M4 8L20 8"/>',
    '<path d="M4 8L7 6.6L7 9.4Z" fill="currentColor" stroke="none"/>',
    '<path d="M20 8L17 9.4L17 6.6Z" fill="currentColor" stroke="none"/>',
    '<path d="M4 4L13 4"/>',
  ] },
  "dim-continue": { group: "cad", accent: [1,2,3], body: [
    '<path d="M4 18L4 5M20 18L20 5"/>',
    '<path d="M4 8L20 8"/>',
    '<path d="M4 8L7 6.6L7 9.4Z" fill="currentColor" stroke="none"/>',
    '<path d="M20 8L17 9.4L17 6.6Z" fill="currentColor" stroke="none"/>',
    '<path d="M12 18L12 5"/>',
  ] },
  "dim-diameter": { group: "cad", accent: [1,2,3], body: [
    '<circle cx="12" cy="12" r="8"/>',
    '<path d="M6.4 17.6L17.6 6.4"/>',
    '<path d="M6.4 17.6L7.53 14.49L9.51 16.47Z" fill="currentColor" stroke="none"/>',
    '<path d="M17.6 6.4L16.47 9.51L14.49 7.53Z" fill="currentColor" stroke="none"/>',
  ] },
  "dim-ordinate": { group: "cad", accent: [1], body: [
    '<path d="M4 20L4 4M4 20L20 20"/>',
    '<path d="M10 20L10 8M16 20L16 11"/>',
  ] },
  "dim-style": { group: "cad", accent: [3,4], body: [
    '<path d="M2 4L2 12M15 4L15 12M2 8L15 8"/>',
    '<path d="M2 8L5 6.6L5 9.4Z" fill="currentColor" stroke="none"/>',
    '<path d="M15 8L12 9.4L12 6.6Z" fill="currentColor" stroke="none"/>',
    '<path d="M20.88 16.27L22.39 16.46L22.39 18.54L20.88 18.73L20.26 19.81L20.85 21.22L19.05 22.25L18.13 21.05L16.87 21.05L15.95 22.25L14.15 21.22L14.74 19.81L14.12 18.73L12.61 18.54L12.61 16.46L14.12 16.27L14.74 15.19L14.15 13.78L15.95 12.75L16.87 13.95L18.13 13.95L19.05 12.75L20.85 13.78L20.26 15.19Z"/>',
    '<circle cx="17.5" cy="17.5" r="1.5"/>',
  ] },
  "distance": { group: "cad", accent: [1,2,3], body: [
    '<path d="M3 15L21 15L21 20L3 20ZM6 15L6 17.5M9 15L9 17.5M12 15L12 17.5M15 15L15 17.5M18 15L18 17.5"/>',
    '<path d="M3 8L21 8"/>',
    '<path d="M3 8L6 6.6L6 9.4Z" fill="currentColor" stroke="none"/>',
    '<path d="M21 8L18 9.4L18 6.6Z" fill="currentColor" stroke="none"/>',
  ] },
  "donut": { group: "cad", accent: [0,1], body: [
    '<circle cx="12" cy="12" r="8"/>',
    '<circle cx="12" cy="12" r="4.5"/>',
    '<circle cx="12" cy="12" r="1" fill="currentColor" stroke="none"/>',
  ] },
  "dyn-input": { group: "cad", body: [
    '<path d="M3 6L15 6L15 13L3 13ZM6 9.5L12 9.5M17 13L21 21M17 13L17 21"/>',
  ] },
  "ellipse": { group: "cad", accent: [0,1,2], body: [
    '<path d="M21 12L20.86 12.87L20.46 13.71L19.79 14.5L18.89 15.21L17.79 15.83L16.5 16.33L15.08 16.7L13.56 16.92L12 17L10.44 16.92L8.92 16.7L7.5 16.33L6.21 15.83L5.11 15.21L4.21 14.5L3.54 13.71L3.14 12.87L3 12L3.14 11.13L3.54 10.29L4.21 9.5L5.11 8.79L6.21 8.17L7.5 7.67L8.92 7.3L10.44 7.08L12 7L13.56 7.08L15.08 7.3L16.5 7.67L17.79 8.17L18.89 8.79L19.79 9.5L20.46 10.29L20.86 11.13Z"/>',
    '<circle cx="3" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="21" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "equal": { group: "cad", accent: [1], body: [
    '<path d="M5 9L19 9"/>',
    '<path d="M5 15L19 15"/>',
  ] },
  "explode": { group: "cad", accent: [0,1], body: [
    '<path d="M4 9L9 4M15 4L20 9M20 15L15 20M9 20L4 15"/>',
    '<circle cx="12" cy="12" r="2" fill="currentColor" stroke="none"/>',
  ] },
  "extend": { group: "cad", accent: [1,2], body: [
    '<path d="M21 2L21 22M2 12L9 12"/>',
    '<path d="M9 12L14 12"/>',
    '<path d="M20.5 12L13.5 16L13.5 8Z" fill="currentColor" stroke="none"/>',
  ] },
  "fix": { group: "cad", accent: [0,1], body: [
    '<rect x="7" y="11" width="10" height="9" fill="currentColor" stroke="none"/>',
    '<path d="M16 11A4 4 0 0 0 8 11"/>',
  ] },
  "grid": { group: "cad", body: [
    '<path d="M4 3L4 21M3 4L21 4M9.33 3L9.33 21M3 9.33L21 9.33M14.66 3L14.66 21M3 14.66L21 14.66M19.99 3L19.99 21M3 19.99L21 19.99"/>',
  ] },
  "hatch": { group: "cad", accent: [1], body: [
    '<path d="M3 3L21 3L21 21L3 21Z"/>',
    '<path d="M3 21L21 3M3 21L3 21M7.5 21L21 7.5M3 16.5L7.5 16.5M12 21L21 12M3 12L12 12M16.5 21L21 16.5M3 7.5L16.5 7.5M21 21L21 21M3 3L21 3"/>',
  ] },
  "horizontal": { group: "cad", accent: [0,1,2], body: [
    '<path d="M3 12L21 12"/>',
    '<circle cx="3" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="21" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "insert": { group: "cad", accent: [1,2], body: [
    '<path d="M9 4L17 8L9 12L1 8ZM1 8L1 15L9 19L17 15L17 8"/>',
    '<path d="M14 20L22 20"/>',
    '<path d="M13 20L16 18.6L16 21.4Z" fill="currentColor" stroke="none"/>',
  ] },
  "isodraft": { group: "cad", body: [
    '<path d="M12 3L20 8L20 17L12 21L4 17L4 8ZM4 8L12 12L20 8M12 12L12 21"/>',
  ] },
  "join": { group: "cad", accent: [1,2], body: [
    '<path d="M3 16L11 8"/>',
    '<path d="M11 8L21 16"/>',
    '<circle cx="11" cy="8" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "layer-freeze": { group: "cad", accent: [1], body: [
    '<path d="M2 9L10 5L18 9L10 13ZM2 13L10 17L18 13"/>',
    '<path d="M18 13.5v7M15 15.25l6 3.5M15 18.75l6-3.5"/>',
  ] },
  "layer-iso": { group: "cad", accent: [1], body: [
    '<path d="M2 9L10 5L18 9L10 13ZM2 13L10 17L18 13"/>',
    '<circle cx="18" cy="17" r="3.5" fill="currentColor" stroke="none"/>',
  ] },
  "layer-lock": { group: "cad", accent: [1], body: [
    '<path d="M2 9L10 5L18 9L10 13ZM2 13L10 17L18 13"/>',
    '<path d="M15 16L21 16L21 21L15 21Z"/>',
  ] },
  "layer-match": { group: "cad", accent: [1], body: [
    '<path d="M2 9L10 5L18 9L10 13ZM2 13L10 17L18 13"/>',
    '<path d="M14 17L22 17"/>',
  ] },
  "layer-off": { group: "cad", accent: [1], body: [
    '<path d="M2 9L10 5L18 9L10 13ZM2 13L10 17L18 13"/>',
    '<circle cx="18" cy="17" r="3.5" fill="currentColor" stroke="none"/>',
  ] },
  "layer-prev": { group: "cad", accent: [1], body: [
    '<path d="M2 9L10 5L18 9L10 13ZM2 13L10 17L18 13"/>',
    '<path d="M18 13.5A3.5 3.5 0 1 0 21.5 17"/>',
  ] },
  "layer-props": { group: "cad", accent: [0], body: [
    '<path d="M3 7L12 3L21 7L12 11Z"/>',
    '<path d="M3 12L12 8L21 12L12 16ZM3 17L12 13L21 17L12 21Z"/>',
  ] },
  "layers": { group: "cad", accent: [0], body: [
    '<path d="M3 7L12 3L21 7L12 11Z"/>',
    '<path d="M3 12L12 8L21 12L12 16ZM3 17L12 13L21 17L12 21Z"/>',
  ] },
  "lengthen": { group: "cad", accent: [1,2], body: [
    '<path d="M3 12L13 12"/>',
    '<path d="M13 12L20 12"/>',
    '<path d="M21 12L18 13.4L18 10.6Z" fill="currentColor" stroke="none"/>',
  ] },
  "lineweight": { group: "cad", body: [
    '<rect x="3" y="5.5" width="18" height="1" fill="currentColor" stroke="none"/>',
    '<rect x="3" y="10.5" width="18" height="2" fill="currentColor" stroke="none"/>',
    '<rect x="3" y="16" width="18" height="3.5" rx="0.5" fill="currentColor" stroke="none"/>',
  ] },
  "m-leader": { group: "cad", accent: [0,1], body: [
    '<path d="M3 20L10 9M10 9L14 9"/>',
    '<path d="M3 20L3.22 16.7L5.67 18.05Z" fill="currentColor" stroke="none"/>',
    '<path d="M15 6L22 6M15 10L21 10"/>',
  ] },
  "m-text": { group: "cad", accent: [0], body: [
    '<path d="M14 15L21 15M14 18L21 18M14 21L19 21"/>',
    '<path d="M3.5 16 8 4l4.5 12M5.2 11.5h5.6"/>',
  ] },
  "make-current": { group: "cad", accent: [1], body: [
    '<path d="M2 9L10 5L18 9L10 13ZM2 13L10 17L18 13"/>',
    '<path d="M15 17L17 19L21 14.5"/>',
  ] },
  "match-prop": { group: "cad", accent: [1], body: [
    '<path d="M4 3L18 3L18 9L4 9ZM18 6L20 6M20 6L20 11L11 11L11 14"/>',
    '<rect x="9.5" y="14" width="3" height="8" fill="currentColor" stroke="none"/>',
  ] },
  "measure": { group: "cad", accent: [1,2,3], body: [
    '<path d="M3 15L21 15L21 20L3 20ZM6 15L6 17.5M9 15L9 17.5M12 15L12 17.5M15 15L15 17.5M18 15L18 17.5"/>',
    '<path d="M3 8L21 8"/>',
    '<path d="M3 8L6 6.6L6 9.4Z" fill="currentColor" stroke="none"/>',
    '<path d="M21 8L18 9.4L18 6.6Z" fill="currentColor" stroke="none"/>',
  ] },
  "move": { group: "cad", accent: [0,1,2,3,4], body: [
    '<path d="M12 6L12 18M6 12L18 12"/>',
    '<path d="M12 1.5L15.3 7L8.7 7Z" fill="currentColor" stroke="none"/>',
    '<path d="M12 22.5L8.7 17L15.3 17Z" fill="currentColor" stroke="none"/>',
    '<path d="M1.5 12L7 8.7L7 15.3Z" fill="currentColor" stroke="none"/>',
    '<path d="M22.5 12L17 15.3L17 8.7Z" fill="currentColor" stroke="none"/>',
  ] },
  "move-bottom": { group: "cad", accent: [0], body: [
    '<path d="M5 20L19 20"/>',
    '<path d="M12 3L12 12"/>',
    '<path d="M12 17L7.5 11L16.5 11Z" fill="currentColor" stroke="none"/>',
  ] },
  "move-down": { group: "cad", body: [
    '<path d="M12 4L12 14"/>',
    '<path d="M12 20L7 13L17 13Z" fill="currentColor" stroke="none"/>',
  ] },
  "move-top": { group: "cad", accent: [0], body: [
    '<path d="M5 4L19 4"/>',
    '<path d="M12 21L12 12"/>',
    '<path d="M12 7L16.5 13L7.5 13Z" fill="currentColor" stroke="none"/>',
  ] },
  "move-up": { group: "cad", body: [
    '<path d="M12 20L12 10"/>',
    '<path d="M12 4L17 11L7 11Z" fill="currentColor" stroke="none"/>',
  ] },
  "o-track": { group: "cad", body: [
    '<path d="M9 9L15 9L15 15L9 15ZM2 12L3.5 12M5 12L6.5 12M17 12L18.5 12M20 12L21.5 12M12 2L12 3.5M12 5L12 6.5M12 17L12 18.5M12 20L12 21.5"/>',
  ] },
  "offset": { group: "cad", accent: [1], body: [
    '<path d="M14 12A6 6 0 0 0 8 18M14 12L21 12M8 18L8 22"/>',
    '<path d="M14 8A10 10 0 0 0 4 18M14 8L21 8M4 18L4 22"/>',
  ] },
  "ortho": { group: "cad", body: [
    '<path d="M4 3L4 20L21 20"/>',
  ] },
  "osnap": { group: "cad", body: [
    '<path d="M7 7L17 7L17 17L7 17ZM12 2L12 7M12 17L12 22M2 12L7 12M17 12L22 12"/>',
  ] },
  "parallel": { group: "cad", accent: [1], body: [
    '<path d="M4 18L16 4"/>',
    '<path d="M9 20L21 6"/>',
  ] },
  "parameters": { group: "cad", accent: [1,2], body: [
    '<path d="M3 4L21 4L21 20L3 20Z"/>',
    '<path d="M11 8a1.5 1.5 0 0 0-3 0v8.5M6.5 11.5h3.5"/>',
    '<path d="M12.5 11.5l3.5 5M16 11.5l-3.5 5"/>',
  ] },
  "perpendicular": { group: "cad", accent: [1], body: [
    '<path d="M4 20L20 20"/>',
    '<path d="M12 20L12 4"/>',
    '<path d="M12 16L16 16L16 20L12 20Z"/>',
  ] },
  "pick-add": { group: "cad", accent: [1], body: [
    '<path d="M3 3L14 3L14 14L3 14Z"/>',
    '<path d="M17 11L17 21M12 16L22 16"/>',
  ] },
  "point": { group: "cad", accent: [0], body: [
    '<circle cx="12" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<path d="M12 5L12 9M12 15L12 19M5 12L9 12M15 12L19 12"/>',
  ] },
  "polar": { group: "cad", body: [
    '<path d="M4 20L21 20M4 20L18 6M13 20A9 9 0 0 0 10.36 13.64"/>',
    '<circle cx="4" cy="20" r="1.2"/>',
  ] },
  "polyline": { group: "cad", accent: [0,1,2,3], body: [
    '<path d="M3 19L8 8L14 15M13.17 14.21A5 5 0 0 0 21.7 9.29"/>',
    '<circle cx="3" cy="19" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="8" cy="8" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="14" cy="15" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "q-dim": { group: "cad", accent: [1,2,3], body: [
    '<path d="M4 18L4 5M20 18L20 5"/>',
    '<path d="M4 8L20 8"/>',
    '<path d="M4 8L7 6.6L7 9.4Z" fill="currentColor" stroke="none"/>',
    '<path d="M20 8L17 9.4L17 6.6Z" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="14.75" r="2.25"/>',
    '<path d="M13.5 16.25 15 17.75"/>',
  ] },
  "quick-props": { group: "cad", accent: [1,3], body: [
    '<path d="M3 4L18 4L18 17L3 17ZM3 7L18 7M5 10L9 10"/>',
    '<path d="M11 10L16 10"/>',
    '<path d="M5 13L9 13"/>',
    '<path d="M11 13L16 13"/>',
    '<path d="M15 14L15 22L17 20L19.5 22.5L20.5 21.5L18 19L21 18.5Z"/>',
  ] },
  "quick-select": { group: "cad", accent: [1], body: [
    '<path d="M3 4L19 4L13 11L13 19L9 17L9 11Z"/>',
    '<path d="M20 10L16.5 15.5L20 15.5L17 21"/>',
  ] },
  "ray": { group: "cad", accent: [0,1], body: [
    '<path d="M5 19L23 3"/>',
    '<circle cx="5" cy="19" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "region": { group: "cad", accent: [0,1], body: [
    '<path d="M4 18L6 6L16 4L20 14L12 20Z"/>',
    '<circle cx="12" cy="12" r="3" fill="currentColor" stroke="none"/>',
  ] },
  "remove-leader": { group: "cad", accent: [0,1,3], body: [
    '<path d="M3 20L10 9M10 9L14 9"/>',
    '<path d="M3 20L3.22 16.7L5.67 18.05Z" fill="currentColor" stroke="none"/>',
    '<path d="M15 6L22 6M15 10L21 10"/>',
    '<path d="M16.5 18h5"/>',
  ] },
  "rev-cloud": { group: "cad", accent: [0], body: [
    '<path d="M19.09 15A3.2 3.2 0 1 0 16.91 9M12.94 19.65A3.2 3.2 0 1 0 17.06 14.75M5.85 16.64A3.2 3.2 0 1 0 12.15 17.75M4.91 9A3.2 3.2 0 1 0 7.09 15M11.06 4.35A3.2 3.2 0 0 0 6.94 9.25M18.15 7.36A3.2 3.2 0 1 0 11.85 6.25"/>',
  ] },
  "scale": { group: "cad", accent: [1,2], body: [
    '<path d="M3 11L13 11L13 21L3 21Z"/>',
    '<path d="M3 3L21 3L21 21L3 21ZM9 15L18 6"/>',
    '<path d="M19 5L17.87 8.11L15.89 6.13Z" fill="currentColor" stroke="none"/>',
  ] },
  "select-objects": { group: "cad", accent: [0], body: [
    '<path d="M12 3L21 3L21 12L12 12Z"/>',
    '<path d="M4 6L4 20L7.5 16.5L10.5 21.5L12.5 20.5L9.5 15.5L14 15Z"/>',
  ] },
  "smooth": { group: "cad", accent: [1,2], body: [
    '<path d="M3 18L11 18"/>',
    '<path d="M11 18A8 8 0 0 0 19 10"/>',
    '<circle cx="11" cy="18" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "snap": { group: "cad", body: [
    '<circle cx="4" cy="4" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="4" cy="9.33" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="4" cy="14.66" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="4" cy="19.99" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="9.33" cy="4" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="9.33" cy="9.33" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="9.33" cy="14.66" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="9.33" cy="19.99" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="14.66" cy="4" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="14.66" cy="9.33" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="14.66" cy="14.66" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="14.66" cy="19.99" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="19.99" cy="4" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="19.99" cy="9.33" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="19.99" cy="14.66" r="1" fill="currentColor" stroke="none"/>',
    '<circle cx="19.99" cy="19.99" r="1" fill="currentColor" stroke="none"/>',
  ] },
  "snowflake": { group: "cad", accent: [0], body: [
    '<path d="M4 12L20 12M8 5.07L16 18.93M16 5.07L8 18.93"/>',
  ] },
  "spline": { group: "cad", accent: [0,1,2,3,4], body: [
    '<path d="M3 12L3.9 10.15L4.8 8.47L5.7 7.15L6.6 6.29L7.5 6L8.4 6.29L9.3 7.15L10.2 8.47L11.1 10.15L12 12L12.9 13.85L13.8 15.53L14.7 16.85L15.6 17.71L16.5 18L17.4 17.71L18.3 16.85L19.2 15.53L20.1 13.85L21 12"/>',
    '<circle cx="3" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="7.5" cy="6" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="16.5" cy="18" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="21" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "stretch": { group: "cad", accent: [1], body: [
    '<path d="M3 18L3 8L10 8"/>',
    '<path d="M10 8L20 8L20 18L3 18"/>',
    '<path d="M12 5L13.5 5M15 5L16.5 5M18 5L19.5 5M21 5L22.5 5"/>',
  ] },
  "symmetric": { group: "cad", accent: [1], body: [
    '<path d="M9 5L4 12L9 19M15 5L20 12L15 19"/>',
    '<path d="M12 3L12 7M12 9L12 13M12 15L12 19"/>',
  ] },
  "tangent": { group: "cad", accent: [1], body: [
    '<circle cx="10" cy="13" r="6"/>',
    '<path d="M3 7L21 7"/>',
  ] },
  "text-style": { group: "cad", accent: [0,1], body: [
    '<path d="M20.88 16.27L22.39 16.46L22.39 18.54L20.88 18.73L20.26 19.81L20.85 21.22L19.05 22.25L18.13 21.05L16.87 21.05L15.95 22.25L14.15 21.22L14.74 19.81L14.12 18.73L12.61 18.54L12.61 16.46L14.12 16.27L14.74 15.19L14.15 13.78L15.95 12.75L16.87 13.95L18.13 13.95L19.05 12.75L20.85 13.78L20.26 15.19Z"/>',
    '<circle cx="17.5" cy="17.5" r="1.5"/>',
    '<path d="M5 16.5 9.5 4.5l4.5 12M6.7 12h5.6"/>',
  ] },
  "transparency": { group: "cad", accent: [0,1], body: [
    '<rect x="3" y="3" width="12" height="12" fill="currentColor" stroke="none"/>',
    '<rect x="9" y="9" width="12" height="12" fill="currentColor" stroke="none"/>',
  ] },
  "trim": { group: "cad", accent: [1,2,3,4,5,6], body: [
    '<path d="M1.5 20L9 20M15 20L22.5 20"/>',
    '<circle cx="7.5" cy="4.5" r="3"/>',
    '<circle cx="16.5" cy="4.5" r="3"/>',
    '<path d="M9.3 6.9L14.6 18.5M14.7 6.9L9.4 18.5"/>',
    '<path d="M14.6 18.5L13 13.5L11.8 14.6Z" fill="currentColor" stroke="none"/>',
    '<path d="M9.4 18.5L11 13.5L12.2 14.6Z" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="12.6" r="1.3" fill="currentColor" stroke="none"/>',
  ] },
  "vertical": { group: "cad", accent: [0,1,2], body: [
    '<path d="M12 3L12 21"/>',
    '<circle cx="12" cy="3" r="1.6" fill="currentColor" stroke="none"/>',
    '<circle cx="12" cy="21" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "wipeout": { group: "cad", accent: [1,2], body: [
    '<path d="M3 3L16 3L16 16L3 16Z"/>',
    '<rect x="8" y="8" width="13" height="13" fill="currentColor" stroke="none"/>',
    '<path d="M8 8L21 8L21 21L8 21Z"/>',
  ] },
  "x-line": { group: "cad", accent: [0,1], body: [
    '<path d="M1 22L23 2"/>',
    '<circle cx="12" cy="12" r="1.6" fill="currentColor" stroke="none"/>',
  ] },
  "export": { group: "files", accent: [1,2], body: [
    '<path d="M5 3L14 3L18 7L18 21L5 21Z"/>',
    '<path d="M9 13L17 13"/>',
    '<path d="M23 13L17 18L17 8Z" fill="currentColor" stroke="none"/>',
  ] },
  "list": { group: "general", body: [
    '<circle cx="5" cy="6" r="1.2" fill="currentColor" stroke="none"/>',
    '<path d="M8 6L20 6"/>',
    '<circle cx="5" cy="11" r="1.2" fill="currentColor" stroke="none"/>',
    '<path d="M8 11L20 11"/>',
    '<circle cx="5" cy="16" r="1.2" fill="currentColor" stroke="none"/>',
    '<path d="M8 16L20 16"/>',
  ] },
  "page-setup": { group: "files", accent: [1,2], body: [
    '<path d="M5 3L17 3L17 20L5 20Z"/>',
    '<circle cx="17" cy="17" r="3"/>',
    '<circle cx="17" cy="17" r="1" fill="currentColor" stroke="none"/>',
  ] },
  "preview": { group: "files", accent: [1,2], body: [
    '<path d="M1.5 8L13.5 8L13.5 14.5L1.5 14.5ZM3.8 4L11.2 4L11.2 8L3.8 8ZM3.8 12L11.2 12L11.2 18L3.8 18Z"/>',
    '<circle cx="17.5" cy="13" r="4"/>',
    '<path d="M20.4 15.9L23 18.5"/>',
  ] },
  "properties": { group: "general", accent: [1,3,5], body: [
    '<path d="M4 3L20 3L20 21L4 21ZM7 8L10 8"/>',
    '<path d="M12 8L17 8"/>',
    '<path d="M7 12L10 12"/>',
    '<path d="M12 12L17 12"/>',
    '<path d="M7 16L10 16"/>',
    '<path d="M12 16L17 16"/>',
  ] },
  "publish": { group: "files", accent: [1], body: [
    '<path d="M3 10L16 10L16 17L3 17ZM6 6L13 6L13 10L6 10Z"/>',
    '<path d="M9 3L21 3L21 12L9 12Z"/>',
  ] },
  "save-as": { group: "files", accent: [1,2], body: [
    '<path d="M4 4L17 4L20 7L20 20L4 20ZM8 4L16 4L16 9L8 9ZM7 13L17 13L17 20L7 20Z"/>',
    '<path d="M12.6 19.4L20.6 11.4L23.6 14.4L15.6 22.4ZM19 13L22 16"/>',
    '<path d="M12.6 19.4L15.6 22.4L10.8 24Z" fill="currentColor" stroke="none"/>',
  ] },
  "switcher": { group: "view", accent: [0], body: [
    '<rect x="3" y="3" width="8" height="8" fill="currentColor" stroke="none"/>',
    '<rect x="13" y="3" width="8" height="8" fill="currentColor" stroke="none"/>',
    '<rect x="3" y="13" width="8" height="8" fill="currentColor" stroke="none"/>',
    '<rect x="13" y="13" width="8" height="8" fill="currentColor" stroke="none"/>',
  ] },
  "workspace": { group: "view", body: [
    '<path d="M3 4L21 4L21 20L3 20ZM9 4L9 20M9 9L21 9"/>',
  ] },
  "zoom-window": { group: "view", accent: [1,2], body: [
    '<path d="M3 3L15 3L15 15L3 15Z"/>',
    '<circle cx="14" cy="14" r="4.5"/>',
    '<path d="M17.5 17.5L21 21"/>',
  ] },
};
