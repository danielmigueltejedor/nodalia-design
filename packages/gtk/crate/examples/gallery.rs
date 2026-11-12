//! Every control NDL draws, in one window: the page to review the language against.
//!
//! `cargo run --example gallery` opens it. With `NDL_GALLERY_SHOT=<file.png>` it renders itself
//! at 2x to that file and exits; `NDL_GALLERY_SCHEME=dark|light` picks the mode.

use ndl::gtk;
use ndl::gtk::glib;
use ndl::gtk::prelude::*;
use nodalia_gtk as ndl;

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("com.getnodalia.Gallery")
        .build();
    app.connect_activate(build);
    app.run_with_args::<&str>(&[])
}

fn build(app: &gtk::Application) {
    ndl::init();
    let manager = ndl::StyleManager::default();
    manager.set_color_scheme(match std::env::var("NDL_GALLERY_SCHEME").as_deref() {
        Ok("light") => ndl::ColorScheme::ForceLight,
        _ => ndl::ColorScheme::ForceDark,
    });

    let columns = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    columns.set_margin_top(8);
    columns.set_margin_bottom(8);
    columns.set_margin_start(8);
    columns.set_margin_end(8);
    columns.set_homogeneous(true);
    columns.append(&island(&[buttons(), choices(), fields()]));
    columns.append(&island(&[values(), tabs_and_chips(), list()]));
    columns.append(&island(&[preferences(), scroller()]));

    let header = ndl::HeaderBar::new();
    header.set_title_widget(Some(&ndl::WindowTitle::new("NDL", "Componentes")));
    let view = ndl::ToolbarView::new();
    view.add_top_bar(&header);
    view.set_content(Some(&columns));

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .default_width(1180)
        .default_height(760)
        .child(&view)
        .build();
    window.present();

    if let Ok(path) = std::env::var("NDL_GALLERY_SHOT") {
        let window = window.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(1500), move || {
            shoot(&window, &path);
            window.application().unwrap().quit();
        });
    }
}

/// Renders the window at 2x to `path`, with its rounded corners see-through.
fn shoot(window: &gtk::ApplicationWindow, path: &str) {
    let (w, h) = (window.width() as f32, window.height() as f32);
    let paintable = gtk::WidgetPaintable::new(Some(window));
    let snapshot = gtk::Snapshot::new();
    snapshot.scale(2.0, 2.0);
    paintable.snapshot(&snapshot, w as f64, h as f64);
    let (Some(node), Some(renderer)) = (
        snapshot.to_node(),
        window.native().and_then(|n| n.renderer()),
    ) else {
        return;
    };
    let rect = gtk::graphene::Rect::new(0.0, 0.0, w * 2.0, h * 2.0);
    let _ = renderer
        .render_texture(&node, Some(&rect))
        .save_to_png(path);
}

fn island(sections: &[gtk::Widget]) -> gtk::Widget {
    let column = gtk::Box::new(gtk::Orientation::Vertical, 18);
    column.set_margin_top(18);
    column.set_margin_bottom(18);
    for section in sections {
        column.append(section);
    }
    let scroller = gtk::ScrolledWindow::new();
    scroller.set_hscrollbar_policy(gtk::PolicyType::Never);
    scroller.set_child(Some(&column));
    let frame = gtk::Box::new(gtk::Orientation::Vertical, 0);
    frame.add_css_class("ndl-island");
    frame.append(&scroller);
    scroller.set_vexpand(true);
    frame.upcast()
}

fn section(title: &str, body: &impl IsA<gtk::Widget>) -> gtk::Widget {
    let block = gtk::Box::new(gtk::Orientation::Vertical, 8);
    block.set_margin_start(18);
    block.set_margin_end(18);
    let label = gtk::Label::new(Some(title));
    label.add_css_class("ndl-micro");
    label.set_xalign(0.0);
    block.append(&label);
    block.append(body);
    block.upcast()
}

fn row(children: &[gtk::Widget]) -> gtk::Box {
    let line = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    for child in children {
        line.append(child);
    }
    line
}

fn button(label: &str, classes: &[&str]) -> gtk::Widget {
    let b = gtk::Button::with_label(label);
    for class in classes {
        b.add_css_class(class);
    }
    b.upcast()
}

fn buttons() -> gtk::Widget {
    let body = gtk::Box::new(gtk::Orientation::Vertical, 8);
    body.append(&row(&[
        button("Reproducir", &["suggested-action"]),
        button("Guardar", &[]),
        button("Eliminar", &["destructive-action"]),
    ]));
    let icon = gtk::Button::from_icon_name("emblem-system-symbolic");
    let flat = gtk::Button::from_icon_name("system-search-symbolic");
    flat.add_css_class("flat");
    let disabled = gtk::Button::with_label("Deshabilitado");
    disabled.set_sensitive(false);
    body.append(&row(&[icon.upcast(), flat.upcast(), disabled.upcast()]));
    section("Botones", &body)
}

fn choices() -> gtk::Widget {
    let body = gtk::Box::new(gtk::Orientation::Vertical, 6);
    let a = gtk::CheckButton::with_label("Recordar la ventana");
    a.set_active(true);
    let b = gtk::CheckButton::with_label("Mostrar etiquetas");
    let r1 = gtk::CheckButton::with_label("Claro");
    let r2 = gtk::CheckButton::with_label("Oscuro");
    r2.set_group(Some(&r1));
    r2.set_active(true);
    let on = gtk::Switch::new();
    on.set_active(true);
    let off = gtk::Switch::new();
    body.append(&a);
    body.append(&b);
    body.append(&row(&[r1.upcast(), r2.upcast()]));
    body.append(&row(&[on.upcast(), off.upcast()]));
    section("Casillas, opciones e interruptores", &body)
}

fn fields() -> gtk::Widget {
    let body = gtk::Box::new(gtk::Orientation::Vertical, 8);
    let entry = gtk::Entry::new();
    entry.set_placeholder_text(Some("Nombre del proyecto"));
    let search = gtk::SearchEntry::new();
    search.set_placeholder_text(Some("Buscar"));
    let spin = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
    spin.set_value(42.0);
    let drop = gtk::DropDown::from_strings(&["Turquesa Nodalia", "Coral", "Ámbar"]);
    body.append(&entry);
    body.append(&search);
    body.append(&row(&[spin.upcast(), drop.upcast()]));
    section("Campos", &body)
}

fn values() -> gtk::Widget {
    let body = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.set_value(62.0);
    let progress = gtk::ProgressBar::new();
    progress.set_fraction(0.4);
    let level = gtk::LevelBar::for_interval(0.0, 1.0);
    level.set_value(0.7);
    let spinner = ndl::Spinner::new();
    body.append(&scale);
    body.append(&progress);
    body.append(&level);
    body.append(&row(&[
        spinner.upcast(),
        gtk::Label::new(Some("Cargando…")).upcast(),
    ]));
    section("Valores y progreso", &body)
}

fn tabs_and_chips() -> gtk::Widget {
    let body = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let stack = gtk::Stack::new();
    for (name, title) in [("a", "Resumen"), ("b", "Detalle"), ("c", "Datos")] {
        stack.add_titled(&gtk::Label::new(None), Some(name), title);
    }
    let switcher = gtk::StackSwitcher::new();
    switcher.set_stack(Some(&stack));
    let notebook = gtk::Notebook::new();
    for title in ["Modelos", "Contornos"] {
        notebook.append_page(
            &gtk::Label::new(Some(title)),
            Some(&gtk::Label::new(Some(title))),
        );
    }
    let chip = |text: &str, class: &str| {
        let label = gtk::Label::new(Some(text));
        label.add_css_class(class);
        label.upcast::<gtk::Widget>()
    };
    body.append(&switcher);
    body.append(&notebook);
    body.append(&row(&[chip("Beta", "ndl-chip"), chip("24", "ndl-count")]));
    section("Pestañas e insignias", &body)
}

fn list() -> gtk::Widget {
    let list = gtk::ListBox::new();
    list.add_css_class("boxed-list");
    for (title, subtitle) in [
        ("Faro norte", "Lumen Costa"),
        ("Avenida 9", "Los Satélites"),
    ] {
        let row = ndl::ActionRow::new();
        row.set_title(title);
        row.set_subtitle(subtitle);
        list.append(&row);
    }
    section("Lista", &list)
}

fn preferences() -> gtk::Widget {
    let group = ndl::PreferencesGroup::new();
    group.set_title("Apariencia");
    let switch_row = ndl::SwitchRow::new();
    switch_row.set_title("Color de la portada");
    switch_row.set_subtitle("Un tinte detrás del reproductor");
    switch_row.set_active(true);
    let combo = ndl::ComboRow::new();
    combo.set_title("Tema");
    combo.set_model(Some(&gtk::StringList::new(&["Sistema", "Claro", "Oscuro"])));
    group.add(&switch_row);
    group.add(&combo);
    section("Preferencias", &group)
}

fn scroller() -> gtk::Widget {
    let content = gtk::Box::new(gtk::Orientation::Vertical, 6);
    for i in 1..=30 {
        let label = gtk::Label::new(Some(&format!("Elemento {i}")));
        label.set_xalign(0.0);
        content.append(&label);
    }
    let scroller = gtk::ScrolledWindow::new();
    scroller.set_min_content_height(170);
    scroller.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Always);
    scroller.set_overlay_scrolling(false);
    scroller.set_child(Some(&content));
    section("Desplazamiento", &scroller)
}
