#![cfg(feature = "engineering")]

use egui::{FontDefinitions, FontFamily};
use nodalia_theme::engineering::{self as eng, Look, TreeHit, TreeRow};
use nodalia_theme::{Accent, Mode};

#[test]
fn plex_leads_every_family_and_keeps_what_was_there() {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "app-cjk".into(),
        std::sync::Arc::new(egui::FontData::from_static(eng::PLEX_MONO)),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("app-cjk".into());
    eng::add_fonts(&mut fonts);

    let regular = &fonts.families[&FontFamily::Proportional];
    assert_eq!(regular[0], "ndl-plex-sans-400");
    assert!(regular.contains(&"app-cjk".to_owned()), "fallbacks kept");
    for (family, weight) in [
        (eng::MEDIUM, 500.0),
        (eng::SEMIBOLD, 600.0),
        (eng::BOLD, 700.0),
    ] {
        let list = &fonts.families[&FontFamily::Name(family.into())];
        let face = &fonts.font_data[&list[0]];
        assert_eq!(face.bytes(), eng::PLEX_SANS);
        assert_eq!(face.tweak.coords.as_ref()[0].1, weight, "{family}");
        assert!(
            list.contains(&"app-cjk".to_owned()),
            "{family} keeps fallbacks"
        );
    }
    assert_eq!(
        fonts.families[&FontFamily::Monospace][0],
        "ndl-plex-mono-400"
    );
}

#[test]
fn adding_twice_does_not_duplicate_faces() {
    let mut fonts = eng::font_definitions();
    eng::add_fonts(&mut fonts);
    let regular = &fonts.families[&FontFamily::Proportional];
    assert_eq!(
        regular.iter().filter(|f| *f == "ndl-plex-sans-400").count(),
        1
    );
    let mono = &fonts.families[&FontFamily::Monospace];
    assert_eq!(mono.iter().filter(|f| *f == "ndl-plex-mono-400").count(), 1);
}

#[test]
fn the_embedded_faces_are_plex() {
    // The name table of both files carries the family name.
    for bytes in [eng::PLEX_SANS, eng::PLEX_MONO] {
        assert!(bytes.windows(4).any(|w| w == b"Plex" || w == b"\0P\0l"));
    }
}

#[test]
fn the_parts_draw_and_the_text_styles_follow_the_scale() {
    let ctx = egui::Context::default();
    eng::install_fonts(&ctx);
    let look = Look::new(Mode::Dark.palette_with(Accent::Torno));
    let mut hit = TreeHit::None;
    for _ in 0..2 {
        let mut out = ctx.run_ui(Default::default(), |ui| {
            eng::island(&look).show(ui, |ui| {
                eng::panel_header(ui, &look, "Browser", |ui| {
                    ui.label("1 body");
                });
                let row = TreeRow {
                    depth: 1,
                    expanded: Some(true),
                    icon: "folder",
                    label: "Bodies",
                    selected: true,
                    muted: false,
                };
                hit = eng::tree_row(ui, &look, &row).1;
                eng::properties(ui, "t", |ui| {
                    eng::property(ui, &look, "Length", |ui| eng::value(ui, &look, "12.500 mm"));
                });
                ui.horizontal(|ui| {
                    eng::ribbon_tab(ui, &look, "Model", true);
                    eng::ribbon_group(ui, &look, "Create", |ui| {
                        eng::ribbon_tool(ui, &look, "extrude", "Extrude", false);
                    });
                });
            });
        });
        out.textures_delta.clear();
    }
    assert_eq!(hit, TreeHit::None);
    let style = ctx.global_style();
    assert_eq!(style.text_styles[&egui::TextStyle::Body], eng::body());
    assert_eq!(
        style.text_styles[&egui::TextStyle::Heading],
        eng::panel_title()
    );
    assert_eq!(
        style.text_styles[&egui::TextStyle::Monospace],
        eng::value_font()
    );
}

#[test]
fn the_shell_parts_draw() {
    use eng::{Document, TitleBar, TitleLabels, ViewItem};
    let ctx = egui::Context::default();
    eng::install_fonts(&ctx);
    let look = Look::new(Mode::Light.palette_with(Accent::Forja));
    let docs = [
        Document {
            name: "Bracket",
            modified: true,
        },
        Document {
            name: "A very long drawing name that is clipped",
            modified: false,
        },
    ];
    let bar = TitleBar {
        app: "Forja",
        documents: &docs,
        active: Some(0),
        labels: TitleLabels {
            home: "Home",
            file: "File",
            save: "Save",
            undo: "Undo",
            redo: "Redo",
            new_document: "New",
            close_document: "Close",
            search: "Search commands",
            menu: "Menu",
        },
        controls_width: nodalia_theme::window_controls::WIDTH,
        leading_width: 0.0,
    };
    let mut controls = egui::Rect::NOTHING;
    for _ in 0..2 {
        let mut out = ctx.run_ui(Default::default(), |ui| {
            let t = eng::title_bar(ui, &look, &bar);
            assert_eq!(t.action, None);
            controls = t.controls;
            eng::status_bar(
                ui,
                |ui| {
                    ui.label("Model");
                },
                |ui| {
                    eng::status_toggle(ui, &look, "grid", "Grid", true);
                    eng::status_value(ui, &look, "12.5000, 4.0039, 0.0000");
                },
            );
            let items = [
                ViewItem::Tool {
                    icon: "orbit",
                    name: "Orbit",
                    active: true,
                },
                ViewItem::Separator,
                ViewItem::Tool {
                    icon: "zoom",
                    name: "Zoom",
                    active: false,
                },
            ];
            assert_eq!(eng::view_bar(ui, &look, &items), None);
        });
        out.textures_delta.clear();
    }
    assert!((controls.width() - nodalia_theme::window_controls::WIDTH).abs() < 0.01);
    assert!((controls.height() - eng::layout::TITLE_BAR).abs() < 0.01);
}
