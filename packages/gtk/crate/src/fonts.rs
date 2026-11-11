//! Sora and Inter, the typefaces NDL names, registered for the app.
//!
//! They ship inside the crate (OFL-1.1, licences beside them in `fonts/`) and are added to the
//! app's Pango font map at start, so an NDL app looks the same on a system that has neither
//! installed, without installing anything for the user. Pango reads font files from disk, so they
//! are written once to the user's cache, in a directory named by their content.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use gtk::glib;
use gtk::prelude::*;

const FONTS: &[(&str, &[u8])] = &[
    (
        "Sora[wght].ttf",
        include_bytes!("../fonts/sora/Sora[wght].ttf"),
    ),
    (
        "Inter[opsz,wght].ttf",
        include_bytes!("../fonts/inter/Inter[opsz,wght].ttf"),
    ),
];

/// Adds the bundled fonts to the font map GTK draws with.
pub(crate) fn install() {
    let dir = match write() {
        Ok(dir) => dir,
        Err(err) => {
            glib::g_warning!("nodalia-gtk", "could not write the bundled fonts: {err}");
            return;
        }
    };
    // Every widget draws through the same default font map; a throwaway label reaches it.
    let map = gtk::Label::new(None).pango_context().font_map();
    let Some(map) = map else { return };
    for (name, _) in FONTS {
        if let Err(err) = map.add_font_file(dir.join(name)) {
            glib::g_warning!("nodalia-gtk", "could not register {name}: {err}");
        }
    }
}

fn write() -> std::io::Result<PathBuf> {
    let mut hasher = DefaultHasher::new();
    for (name, bytes) in FONTS {
        (name, bytes.len()).hash(&mut hasher);
    }
    let dir = glib::user_cache_dir()
        .join("nodalia-gtk")
        .join(format!("fonts-{:016x}", hasher.finish()));
    std::fs::create_dir_all(&dir)?;
    for (name, bytes) in FONTS {
        let path = dir.join(name);
        if std::fs::metadata(&path).map(|m| m.len()).ok() != Some(bytes.len() as u64) {
            std::fs::write(&path, bytes)?;
        }
    }
    Ok(dir)
}
