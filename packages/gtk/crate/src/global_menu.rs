//! The app's menu in the shell's top bar.
//!
//! A Nodalia app hands the shell its menus (the top bar shows "Vinilo · Reproducción · Ver …"),
//! the way macOS apps do. GTK exports a `GtkApplication`'s menubar over D-Bus
//! (`org.gtk.Menus` at `<app path>/menus/menubar`), and the shell activates its entries
//! through the exported `org.gtk.Actions`.
//!
//! Only `app.*` actions are exported that way: actions a window keeps in an action group
//! inserted on a widget (`win.*` from relm4, say) never reach the bus. [`global_menu`] therefore
//! gives each entry an `app.*` twin that activates the window's own action, and copies the
//! shortcut the app set for it so the shell can show it.

use gtk::gio;
use gtk::prelude::*;

/// One entry: its label and the action it runs (`win.preferences`, `app.quit`…).
pub type MenuEntry<'a> = (&'a str, &'a str);

/// A top-level menu: its title and its sections, each a list of entries.
pub type TopMenu<'a> = (&'a str, &'a [&'a [MenuEntry<'a>]]);

/// Sets `menus` as `app`'s menubar for the shell. Entries naming a `win.*` action run it on
/// `window`. The window's own menubar is turned off: the menu lives in the shell.
pub fn global_menu(app: &gtk::Application, window: &gtk::ApplicationWindow, menus: &[TopMenu<'_>]) {
    let bar = gio::Menu::new();
    for (title, sections) in menus {
        let menu = gio::Menu::new();
        for entries in *sections {
            let section = gio::Menu::new();
            for (label, action) in *entries {
                section.append_item(&entry(app, window, label, action));
            }
            menu.append_section(None, &section);
        }
        bar.append_submenu(Some(title), &menu);
    }
    app.set_menubar(Some(&bar));
    window.set_show_menubar(false);
}

fn entry(
    app: &gtk::Application,
    window: &gtk::ApplicationWindow,
    label: &str,
    action: &str,
) -> gio::MenuItem {
    let exported = match action.strip_prefix("win.") {
        Some(name) => forward(app, window, name),
        None => action.to_owned(),
    };
    let item = gio::MenuItem::new(Some(label), Some(&exported));
    if let Some(accel) = app.accels_for_action(action).first() {
        item.set_attribute_value("accel", Some(&accel.to_variant()));
    }
    item
}

/// An `app.*` action that runs the window's `win.<name>`. Named `app.win-<name>` so it
/// cannot clash with an action the app has of its own.
fn forward(app: &gtk::Application, window: &gtk::ApplicationWindow, name: &str) -> String {
    let twin = format!("win-{name}");
    if app.lookup_action(&twin).is_none() {
        let action = gio::SimpleAction::new(&twin, None);
        let window = window.downgrade();
        let target = format!("win.{name}");
        action.connect_activate(move |_, _| {
            if let Some(window) = window.upgrade() {
                let _ = WidgetExt::activate_action(&window, &target, None);
            }
        });
        app.add_action(&action);
    }
    format!("app.{twin}")
}
