//! Menu rows with an icon on every entry.
//!
//! GTK's own menu rows hide an item's icon whenever it also has a label, so an NDL menu is
//! made of custom rows instead: [`menu_item`] marks an entry as one, and [`menu_rows`] builds
//! those rows into a [`gtk::PopoverMenu`] showing that model. A row is an icon, the label and
//! the action's shortcut, and activating it runs the action and closes the menu, as GTK's do.

use std::cell::Cell;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

/// The `custom` id prefix that marks an NDL row.
const PREFIX: &str = "ndl-row-";
/// Set on an entry whose action destroys or signs out: its row is drawn as a danger.
const DANGER: &str = "ndl-danger";

thread_local! {
    static NEXT: Cell<u64> = const { Cell::new(0) };
}

/// A menu entry that [`menu_rows`] draws with `icon` beside its label.
pub fn menu_item(label: &str, action: &str, icon: &str) -> gio::MenuItem {
    let item = gio::MenuItem::new(Some(label), Some(action));
    item.set_icon(&gio::ThemedIcon::new(icon));
    let id = NEXT.with(|n| {
        let id = n.get();
        n.set(id + 1);
        id
    });
    item.set_attribute_value("custom", Some(&format!("{PREFIX}{id}").to_variant()));
    item
}

/// [`menu_item`], drawn as a danger (signing out, deleting).
pub fn menu_item_danger(label: &str, action: &str, icon: &str) -> gio::MenuItem {
    let item = menu_item(label, action, icon);
    item.set_attribute_value(DANGER, Some(&true.to_variant()));
    item
}

/// Builds the rows of every [`menu_item`] in `popover`'s model. Call it once the popover
/// has its model, and again if the model is replaced.
pub fn menu_rows(popover: &gtk::PopoverMenu) {
    if let Some(model) = popover.menu_model() {
        add_rows(popover, &model);
    }
}

fn add_rows(popover: &gtk::PopoverMenu, model: &gio::MenuModel) {
    for i in 0..model.n_items() {
        let id = model
            .item_attribute_value(i, "custom", Some(glib::VariantTy::STRING))
            .and_then(|v| v.get::<String>());
        if let Some(id) = id.filter(|id| id.starts_with(PREFIX)) {
            popover.add_child(&row(popover, model, i), &id);
        }
        for link in [gio::MENU_LINK_SECTION, gio::MENU_LINK_SUBMENU] {
            if let Some(child) = model.item_link(i, link) {
                add_rows(popover, &child);
            }
        }
    }
}

fn string(model: &gio::MenuModel, i: i32, name: &str) -> Option<String> {
    model
        .item_attribute_value(i, name, Some(glib::VariantTy::STRING))
        .and_then(|v| v.get::<String>())
}

fn row(popover: &gtk::PopoverMenu, model: &gio::MenuModel, i: i32) -> gtk::Button {
    let action = string(model, i, "action");
    let target = model.item_attribute_value(i, "target", None);
    let label = string(model, i, "label").unwrap_or_default();

    let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    if let Some(icon) = model
        .item_attribute_value(i, "icon", None)
        .and_then(|v| gio::Icon::deserialize(&v))
    {
        content.append(&gtk::Image::from_gicon(&icon));
    }
    let text = gtk::Label::with_mnemonic(&label);
    text.set_xalign(0.0);
    text.set_hexpand(true);
    content.append(&text);
    let shortcut = gtk::Label::new(None);
    shortcut.add_css_class("accelerator");
    content.append(&shortcut);

    let button = gtk::Button::builder().child(&content).build();
    button.add_css_class("ndl-menu-row");
    button.add_css_class("flat");
    if model
        .item_attribute_value(i, DANGER, Some(glib::VariantTy::BOOLEAN))
        .is_some()
    {
        button.add_css_class("danger");
    }
    // Read when shown, not when built: an app sets its shortcuts after building its menus.
    let (wanted, wanted_target) = (action.clone(), target.clone());
    button.connect_map(move |_| {
        let accel = wanted
            .as_deref()
            .and_then(|a| accelerator(a, wanted_target.as_ref()));
        shortcut.set_visible(accel.is_some());
        shortcut.set_label(accel.as_deref().unwrap_or_default());
    });
    button.set_action_name(action.as_deref());
    button.set_action_target_value(target.as_ref());
    let popover = popover.downgrade();
    button.connect_clicked(move |_| {
        if let Some(popover) = popover.upgrade() {
            popover.popdown();
        }
    });
    button
}

/// The first shortcut the application has for `action`, as GTK writes it.
fn accelerator(action: &str, target: Option<&glib::Variant>) -> Option<String> {
    let app = gio::Application::default().and_downcast::<gtk::Application>()?;
    let detailed = gio::Action::print_detailed_name(action, target);
    let accels = app.accels_for_action(&detailed);
    let first = accels.first()?;
    let (key, mods) = gtk::accelerator_parse(first)?;
    Some(gtk::accelerator_get_label(key, mods).to_string())
}
