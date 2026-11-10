#![cfg(feature = "egui")]

use nodalia_theme::for_egui::{self, color, Appearance};
use nodalia_theme::{contrast, Density, Mode};

#[test]
fn visuals_follow_the_palette_in_every_mode() {
    for mode in Mode::ALL {
        let p = mode.palette();
        let v = for_egui::visuals(mode);
        assert_eq!(v.dark_mode, mode.is_dark(), "{}", mode.name());
        assert_eq!(v.panel_fill, color(p.surface_panel));
        assert_eq!(v.widgets.inactive.bg_fill, color(p.control_bg));
        assert_eq!(
            v.widgets.noninteractive.fg_stroke.color,
            color(p.text_primary)
        );
        assert_eq!(v.selection.bg_fill, color(p.selection_bg));
        assert_eq!(v.error_fg_color, color(p.status_danger_fg));
    }
}

#[test]
fn the_colour_pairs_egui_draws_keep_their_contrast() {
    for mode in Mode::ALL {
        let p = mode.palette();
        let text = if mode.is_high_contrast() { 7.0 } else { 4.5 };
        for (name, fg, bg, minimum) in [
            ("text on control", p.text_primary, p.control_bg, text),
            (
                "text on hovered control",
                p.text_primary,
                p.control_bg_hover,
                text,
            ),
            (
                "text on pressed control",
                p.text_primary,
                p.control_bg_pressed,
                4.5,
            ),
            ("selected content", p.accent_text, p.selection_bg, 4.5),
            ("link on panel", p.text_link, p.surface_panel, 4.5),
            ("weak text on panel", p.text_secondary, p.surface_panel, 4.5),
            ("error on panel", p.status_danger_fg, p.surface_panel, 4.5),
            (
                "warning on panel",
                p.status_warning_fg,
                p.surface_panel,
                4.5,
            ),
            (
                "focus outline on panel",
                p.accent_text,
                p.surface_panel,
                3.0,
            ),
        ] {
            let ratio = contrast(fg, bg);
            assert!(
                ratio >= minimum,
                "{}: {name} is {ratio:.2}, needs {minimum}",
                mode.name()
            );
        }
    }
}

#[test]
fn style_takes_metrics_from_the_density() {
    for density in Density::ALL {
        let s = for_egui::style(&Appearance {
            density,
            ..Appearance::default()
        });
        assert_eq!(s.spacing.interact_size.y, density.control_height);
        assert_eq!(s.spacing.item_spacing.x, density.gap);
        assert_eq!(s.spacing.button_padding.x, density.control_padding_x);
        // A button is as tall as the control height: body line plus vertical padding.
        assert!(
            nodalia_theme::line_height::BODY + 2.0 * s.spacing.button_padding.y
                <= density.control_height
        );
    }
}

#[test]
fn text_sizes_and_motion_come_from_the_tokens() {
    let s = for_egui::style(&Appearance::default());
    assert_eq!(
        s.text_styles[&egui::TextStyle::Body].size,
        nodalia_theme::font_size::BODY
    );
    assert_eq!(
        s.text_styles[&egui::TextStyle::Heading].size,
        nodalia_theme::font_size::TITLE
    );
    assert!((s.animation_time - 0.16).abs() < 1e-6);
    let reduced = for_egui::style(&Appearance {
        reduced_motion: true,
        ..Appearance::default()
    });
    assert_eq!(reduced.animation_time, 0.0);
}

#[test]
fn apply_installs_the_style_in_a_context() {
    let ctx = egui::Context::default();
    for_egui::apply(
        &ctx,
        &Appearance {
            mode: Mode::Dark,
            ..Appearance::default()
        },
    );
    assert_eq!(ctx.theme(), egui::Theme::Dark);
    assert_eq!(
        ctx.global_style().visuals.panel_fill,
        color(Mode::Dark.palette().surface_panel)
    );

    for_egui::apply_system(
        &ctx,
        &Appearance {
            mode: Mode::LightHc,
            ..Appearance::default()
        },
    );
    let mut dark = None;
    ctx.style_mut_of(egui::Theme::Dark, |s| dark = Some(s.visuals.panel_fill));
    assert_eq!(dark, Some(color(Mode::DarkHc.palette().surface_panel)));
}

#[test]
fn an_app_accent_changes_the_accent_and_nothing_else() {
    use nodalia_theme::Accent;
    for mode in Mode::ALL {
        let base = for_egui::visuals(mode);
        let vortia = for_egui::visuals_with(mode, Accent::Vortia);
        assert_ne!(
            vortia.selection.bg_fill,
            base.selection.bg_fill,
            "{}",
            mode.name()
        );
        assert_ne!(vortia.hyperlink_color, base.hyperlink_color);
        assert_eq!(vortia.panel_fill, base.panel_fill);
        assert_eq!(
            vortia.widgets.inactive.bg_fill,
            base.widgets.inactive.bg_fill
        );
        assert_eq!(vortia.error_fg_color, base.error_fg_color);
    }
    let style = for_egui::style(&Appearance {
        accent: Accent::Luz,
        ..Appearance::default()
    });
    assert_eq!(
        style.visuals.selection.bg_fill,
        color(Mode::Light.palette_with(Accent::Luz).selection_bg)
    );
}
