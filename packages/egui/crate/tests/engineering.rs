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
    use eng::{Document, TitleAction, TitleBar, TitleLabels, ViewItem};
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
        icon: None,
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
        quick_access: &[TitleAction::Save, TitleAction::Undo, TitleAction::Redo],
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

#[test]
fn the_title_bar_starts_with_the_app_icon_when_there_is_one() {
    use eng::{TitleAction, TitleBar, TitleLabels};
    fn walk(shape: &egui::Shape, texts: &mut Vec<String>, images: &mut Vec<egui::TextureId>) {
        match shape {
            egui::Shape::Text(t) => texts.push(t.galley.text().to_owned()),
            egui::Shape::Mesh(m) => images.push(m.texture_id),
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, texts, images)),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    eng::install_fonts(&ctx);
    let look = Look::new(Mode::Dark.palette_with(Accent::Forja));
    let icon = egui::TextureId::User(7);
    let bar = TitleBar {
        app: "Forja",
        icon: Some(icon),
        documents: &[],
        active: None,
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
        quick_access: &[TitleAction::Save],
        controls_width: 0.0,
        leading_width: 0.0,
    };
    let (mut texts, mut images) = (Vec::new(), Vec::new());
    for _ in 0..2 {
        let mut out = ctx.run_ui(Default::default(), |ui| {
            eng::title_bar(ui, &look, &bar);
        });
        out.textures_delta.clear();
        texts.clear();
        images.clear();
        for s in &out.shapes {
            walk(&s.shape, &mut texts, &mut images);
        }
    }
    assert!(images.contains(&icon), "the icon is painted");
    assert!(
        !texts.iter().any(|t| t == "Forja"),
        "the name gives way to the icon"
    );
}

#[test]
fn home_draws_in_the_office_view_with_the_suite_type() {
    use eng::{RecentDocument, StartCard};
    use nodalia_theme::office::{self, Entry, Nav};
    let ctx = egui::Context::default();
    eng::install_fonts(&ctx);
    let look = Look::new(Mode::Dark.palette_with(Accent::Forja));
    let home = eng::home_look(&look);
    assert_eq!(home.bold, egui::FontFamily::Name(eng::SEMIBOLD.into()));
    let nav = Nav {
        back: "",
        top: vec![
            Entry::item("home", "Home", "home"),
            Entry::item("open", "Open", "folder-open"),
        ],
        bottom: vec![Entry::item("options", "Preferences", "settings")],
        selected: Some("home"),
    };
    let cards = [StartCard {
        icon: "line",
        name: "Drawing (mm)",
        detail: "Metric units",
    }];
    let recents = [RecentDocument {
        name: "Bracket.dxf",
        folder: "~/Drawings",
        when: "2026-11-14 04:00",
        icon: "line",
        thumbnail: None,
    }];
    for _ in 0..2 {
        let mut out = ctx.run_ui(Default::default(), |ui| {
            let picked = office::file_view(ui, &home, &nav, |ui| {
                eng::home_header(ui, &look, None, "Forja", "Version 0.1.0");
                assert_eq!(eng::start_cards(ui, &look, &cards), None);
                assert_eq!(
                    eng::recent_documents(ui, &look, &recents, "Nothing yet"),
                    None
                );
                assert_eq!(eng::recent_documents(ui, &look, &[], "Nothing yet"), None);
            });
            assert_eq!(picked, None);
        });
        out.textures_delta.clear();
    }
}

#[test]
fn a_ribbon_group_is_as_wide_as_its_tools() {
    let ctx = egui::Context::default();
    eng::install_fonts(&ctx);
    let look = Look::new(Mode::Light.palette_with(Accent::Forja));
    let mut group = egui::Rect::NOTHING;
    for _ in 0..2 {
        let mut out = ctx.run_ui(Default::default(), |ui| {
            ui.horizontal(|ui| {
                let before = ui.cursor().left();
                eng::ribbon_group(ui, &look, "Fields", |ui| {
                    eng::ribbon_tool(ui, &look, "line", "Line", false);
                });
                group = egui::Rect::from_x_y_ranges(before..=ui.cursor().left(), 0.0..=1.0);
            });
        });
        out.textures_delta.clear();
    }
    // One tool, a gap and the divider: nowhere near the window's width.
    assert!(
        group.width() < eng::layout::TOOL.x + 40.0,
        "{} points wide",
        group.width()
    );
}

#[test]
fn home_items_report_their_responses() {
    use eng::{RecentDocument, StartCard};
    use nodalia_theme::office::{self, Entry, Nav};
    let ctx = egui::Context::default();
    eng::install_fonts(&ctx);
    let look = Look::new(Mode::Light.palette_with(Accent::Forja));
    let home = eng::home_look(&look);
    let nav = Nav {
        back: "Back",
        top: vec![Entry::item("home", "Home", "home")],
        bottom: vec![Entry::item("options", "Preferences", "settings")],
        selected: Some("home"),
    };
    let cards = [StartCard {
        icon: "line",
        name: "Drawing",
        detail: "Inches",
    }];
    let recents = [RecentDocument {
        name: "a.dxf",
        folder: "~",
        when: "",
        icon: "line",
        thumbnail: None,
    }];
    let (mut entries, mut card, mut recent) = (Vec::new(), Vec::new(), Vec::new());
    let mut out = ctx.run_ui(Default::default(), |ui| {
        office::file_view_with(
            ui,
            &home,
            &nav,
            |id, r| entries.push((id.to_owned(), r.rect)),
            |ui| {
                eng::start_cards_with(ui, &look, &cards, |i, r| card.push((i, r.rect)));
                eng::recent_documents_with(ui, &look, &recents, "", |i, r| {
                    recent.push((i, r.rect))
                });
            },
        );
    });
    out.textures_delta.clear();
    let ids: Vec<&str> = entries.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(ids, [office::BACK, "home", "options"]);
    assert!(entries.iter().all(|(_, r)| r.is_positive()));
    assert_eq!(card.len(), 1);
    assert_eq!(card[0].1.size(), eng::START_CARD);
    assert_eq!(recent.len(), 1);
    assert!((recent[0].1.height() - eng::RECENT_ROW).abs() < 0.01);
}
