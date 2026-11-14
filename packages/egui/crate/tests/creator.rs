#![cfg(feature = "creator")]

use egui::{pos2, vec2, Rect};
use nodalia_theme::creator::{self as cr, layout, Look};
use nodalia_theme::{for_egui::color, Accent, Mode};

fn run(mut draw: impl FnMut(&mut egui::Ui)) {
    let ctx = egui::Context::default();
    for _ in 0..2 {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| draw(ui));
        out.textures_delta.clear();
    }
}

fn look(mode: Mode) -> Look {
    Look::new(mode.palette_with(Accent::Lienzo))
}

#[test]
fn every_part_draws_in_both_modes_at_the_suite_measures() {
    for mode in [Mode::Light, Mode::Dark] {
        let look = look(mode);
        let mut sizes = Vec::new();
        run(|ui| {
            sizes.clear();
            sizes.push(cr::primary_button(ui, &look, "OK", true).rect.size());
            sizes.push(cr::secondary_button(ui, &look, "Cancel", true).rect.size());
            sizes.push(cr::tab(ui, &look, "Tools", true).rect.size());
            ui.set_width(220.0);
            sizes.push(cr::nav_row(ui, &look, "General", true, false).rect.size());
            sizes.push(cr::home_row(ui, &look, "home", "Home", true).rect.size());
            let mut q = String::new();
            cr::search_field(ui, &look, &mut q, "Search", 200.0);
            assert_eq!(
                cr::workspace_selector(
                    ui,
                    &look,
                    "ws",
                    "Essentials",
                    &["Essentials", "Motion", "Mine"],
                    2,
                    &["New Workspace…"],
                    130.0
                ),
                None
            );
            let close = Rect::from_min_size(pos2(0.0, 0.0), vec2(layout::CLOSE, layout::CLOSE));
            assert!(!cr::close_button(
                ui,
                &look,
                close,
                egui::Id::NULL.with("close")
            ));
            cr::round_island_corners(
                ui.painter(),
                &look,
                Rect::from_min_size(pos2(0.0, 0.0), vec2(100.0, 100.0)),
            );
        });
        assert_eq!(sizes[0].y, layout::BUTTON_HEIGHT);
        assert!(
            sizes[0].x >= layout::BUTTON_MIN_WIDTH,
            "OK keeps the minimum width"
        );
        assert!(sizes[1].x >= layout::BUTTON_MIN_WIDTH);
        assert_eq!(sizes[3].y, layout::NAV_ROW);
        assert_eq!(sizes[4].y, layout::HOME_ROW);
    }
}

#[test]
fn the_primary_button_inverts_the_text_colour() {
    // Black in light themes, white in dark ones, with the panel as ink: the contrast the
    // creator suite's main actions rely on.
    for mode in [Mode::Light, Mode::Dark] {
        let p = mode.palette_with(Accent::Pluma);
        assert!(
            nodalia_theme::contrast(p.surface_panel, p.text_primary) >= 7.0,
            "{mode:?}"
        );
    }
    let light = Mode::Light.palette();
    assert!(color(light.text_primary).r() < 64, "light: a dark button");
}

#[test]
fn the_island_frame_has_the_suite_radius() {
    let frame = cr::island(&look(Mode::Light));
    assert_eq!(
        frame.corner_radius,
        egui::CornerRadius::same(layout::ISLAND_RADIUS)
    );
    assert_eq!(
        cr::field_frame(&look(Mode::Dark)).corner_radius,
        egui::CornerRadius::same(layout::FIELD_RADIUS)
    );
}
