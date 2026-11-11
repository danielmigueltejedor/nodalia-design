//! The Nodalia icon theme.
//!
//! Every NDL icon is installed as `ndl-<name>-symbolic`, and the freedesktop names an app or GTK
//! itself asks for (`window-close-symbolic`, `pan-down-symbolic`…) answer with the NDL drawing
//! too, so nothing on screen falls back to another set's look. A name NDL does not draw falls
//! through to the theme the system had, then Adwaita, then hicolor.
//!
//! GTK only reads icon themes from directories, so the theme is written once to the user's cache
//! (a directory per content, so an update never mixes files) and added to the display's search
//! path. The SVGs come from `icons_generated.rs`, which `@nodalia/gtk` writes from the icon set.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use gtk::glib;

#[path = "icons_generated.rs"]
mod generated;

use generated::ICONS;

/// The theme's name, as `gtk-icon-theme-name` reads it.
pub const THEME: &str = "Nodalia";

/// Whether the Nodalia icon theme draws `name` itself (rather than leaving it to the
/// inherited theme). Apps use it in a test so no icon of another set reaches the screen.
pub fn has_icon(name: &str) -> bool {
    ICONS.iter().any(|(icon, _)| *icon == name)
}

/// Makes the Nodalia icon theme the display's, inheriting from the theme it replaces. Keeps it
/// in place if the system changes its icon theme later.
pub(crate) fn install(display: &gtk::gdk::Display) {
    let settings = gtk::Settings::for_display(display);
    apply(display, &settings);
    let display = display.clone();
    settings.connect_gtk_icon_theme_name_notify(move |settings| {
        if settings.gtk_icon_theme_name().as_deref() != Some(THEME) {
            apply(&display, settings);
        }
    });
}

fn apply(display: &gtk::gdk::Display, settings: &gtk::Settings) {
    let previous = settings
        .gtk_icon_theme_name()
        .map(|n| n.to_string())
        .filter(|n| n != THEME);
    match write(previous.as_deref()) {
        Ok(root) => {
            gtk::IconTheme::for_display(display).add_search_path(&root);
            settings.set_gtk_icon_theme_name(Some(THEME));
        }
        Err(err) => glib::g_warning!("nodalia-gtk", "could not install the icon theme: {err}"),
    }
}

/// Writes the theme under the user's cache and returns the directory to search.
fn write(inherits: Option<&str>) -> std::io::Result<PathBuf> {
    let index = index_theme(inherits);
    let system = crate::theme::window_controls() == crate::theme::WindowControls::System;
    let mut hasher = DefaultHasher::new();
    (ICONS, &index, system).hash(&mut hasher);
    let root = glib::user_cache_dir()
        .join("nodalia-gtk")
        .join(format!("icons-{:016x}", hasher.finish()));
    let theme = root.join(THEME);
    let done = theme.join(".complete");
    if !done.exists() {
        write_files(&theme, &index)?;
        std::fs::write(done, b"")?;
    }
    Ok(root)
}

fn write_files(theme: &Path, index: &str) -> std::io::Result<()> {
    let actions = theme.join("scalable").join("actions");
    std::fs::create_dir_all(&actions)?;
    std::fs::write(theme.join("index.theme"), index)?;
    let system = crate::theme::window_controls() == crate::theme::WindowControls::System;
    for (name, svg) in ICONS {
        // With the desktop's window controls its own glyphs answer for them.
        if system && is_window_glyph(name) {
            continue;
        }
        std::fs::write(actions.join(format!("{name}.svg")), svg)?;
    }
    Ok(())
}

/// The names GTK's window controls ask for.
fn is_window_glyph(name: &str) -> bool {
    matches!(
        name,
        "window-minimize-symbolic"
            | "window-maximize-symbolic"
            | "window-restore-symbolic"
            | "window-close-symbolic"
    )
}

fn index_theme(inherits: Option<&str>) -> String {
    let mut parents: Vec<&str> = inherits
        .into_iter()
        .filter(|n| !matches!(*n, "Adwaita" | "hicolor"))
        .collect();
    parents.extend(["Adwaita", "hicolor"]);
    format!(
        "[Icon Theme]\nName={THEME}\nComment=Nodalia Design Language icons\nInherits={}\n\
         Directories=scalable/actions\n\n[scalable/actions]\nSize=16\nMinSize=8\nMaxSize=512\n\
         Type=Scalable\nContext=Actions\n",
        parents.join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_is_a_symbolic_svg() {
        assert!(ICONS.len() > 300);
        for (name, svg) in ICONS {
            assert!(name.ends_with("-symbolic"), "{name}");
            assert!(
                svg.starts_with("<svg") && svg.contains("foreground-"),
                "{name}"
            );
        }
    }

    #[test]
    fn the_replaced_theme_is_inherited_first() {
        let index = index_theme(Some("Colloid"));
        assert!(index.contains("Inherits=Colloid,Adwaita,hicolor\n"));
        assert!(index_theme(Some("Adwaita")).contains("Inherits=Adwaita,hicolor\n"));
    }
}
