// The NDL icon set. One entry per icon: the SVG elements drawn inside a 24 x 24 view box.
// Rules (docs/iconography.md): 3 px of clear margin, 1.75 px stroke with round caps and joins,
// filled circles for nodes, dashes only for what is not real geometry, a soft fill only for a face.
// Colours are always currentColor. Edit here and rebuild; never retouch a generated file.

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
  "settings": { group: "edit", body: [
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
  "run": { group: "run", body: [
    '<path d="M7.5 4.5v15L20 12z"/>',
  ] },
  "stop": { group: "run", body: [
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
  "line": { group: "sketch", body: [
    '<path d="M6.5 17.5l11-11"/>',
    '<circle cx="5" cy="19" r="2"/>',
    '<circle cx="19" cy="5" r="2"/>',
  ] },
  "rectangle": { group: "sketch", body: [
    '<rect x="4.5" y="6.5" width="15" height="11" rx="0.5"/>',
    '<circle cx="4.5" cy="6.5" r="1.9" fill="currentColor" stroke="none"/>',
    '<circle cx="19.5" cy="17.5" r="1.9" fill="currentColor" stroke="none"/>',
  ] },
  "circle": { group: "sketch", body: [
    '<circle cx="12" cy="12" r="8"/>',
    '<circle cx="12" cy="12" r="1.5" fill="currentColor" stroke="none"/>',
  ] },
  "radius": { group: "sketch", body: [
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
  "dimension": { group: "sketch", body: [
    '<path d="M4 5v14M20 5v14"/>',
    '<path d="M4 12h16"/>',
    '<path d="M7.5 9 4.5 12l3 3M16.5 9l3 3-3 3"/>',
  ] },
  "fillet": { group: "sketch", body: [
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
};
