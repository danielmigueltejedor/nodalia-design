use nodalia_theme::{contrast, duration, motion, Density, Mode};

#[test]
fn every_mode_keeps_text_readable_on_its_surfaces() {
    for mode in Mode::ALL {
        let p = mode.palette();
        let minimum = if mode.is_high_contrast() { 7.0 } else { 4.5 };
        for (name, fg, bg) in [
            ("primary on window", p.text_primary, p.surface_window),
            ("primary on panel", p.text_primary, p.surface_panel),
            ("primary on overlay", p.text_primary, p.surface_overlay),
            ("primary on field", p.text_primary, p.control_field_bg),
            ("on-accent on accent", p.text_on_accent, p.accent_default),
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
fn modes_report_their_scheme() {
    assert!(
        !Mode::Light.is_dark()
            && Mode::Dark.is_dark()
            && Mode::DarkHc.is_dark()
            && !Mode::LightHc.is_dark()
    );
    assert!(Mode::LightHc.is_high_contrast() && !Mode::Dark.is_high_contrast());
    assert_eq!(Mode::LightHc.name(), "light-hc");
    assert!(
        nodalia_theme::luminance(Mode::Dark.palette().surface_window)
            < nodalia_theme::luminance(Mode::Light.palette().surface_window)
    );
}

#[test]
fn densities_grow_and_respect_the_minimum_target() {
    let [compact, regular, comfortable] = Density::ALL;
    assert!(
        compact.control_height < regular.control_height
            && regular.control_height < comfortable.control_height
    );
    assert!(compact.control_height >= nodalia_theme::size::TARGET_MIN);
    assert_eq!(Density::default(), Density::REGULAR);
}

#[test]
fn reduced_motion_collapses_durations() {
    assert_eq!(motion::STATE.seconds(true), 0.0);
    assert!((motion::STATE.seconds(false) - duration::NORMAL as f32 / 1000.0).abs() < 1e-6);
}

#[test]
fn every_app_accent_keeps_the_contrast_of_the_default_one() {
    use nodalia_theme::Accent;
    assert_eq!(Accent::default(), Accent::Nodalia);
    assert_eq!(
        Mode::Dark.palette_with(Accent::Nodalia),
        *Mode::Dark.palette()
    );
    for accent in Accent::ALL {
        for mode in Mode::ALL {
            let p = mode.palette_with(accent);
            let text = if mode.is_high_contrast() { 7.0 } else { 4.5 };
            for (name, fg, bg, minimum) in [
                (
                    "on-accent on accent",
                    p.text_on_accent,
                    p.accent_default,
                    text,
                ),
                ("on-accent on hover", p.text_on_accent, p.accent_hover, text),
                ("accent text on panel", p.accent_text, p.surface_panel, text),
                (
                    "accent text on subtle",
                    p.accent_text,
                    p.accent_subtle,
                    text,
                ),
                ("text on selection", p.text_primary, p.selection_bg, text),
                ("link on window", p.text_link, p.surface_window, text),
                ("focus on panel", p.border_focus, p.surface_panel, 3.0),
            ] {
                let ratio = contrast(fg, bg);
                assert!(
                    ratio >= minimum,
                    "{} {}: {name} is {ratio:.2}, needs {minimum}",
                    accent.name(),
                    mode.name()
                );
            }
        }
    }
}
