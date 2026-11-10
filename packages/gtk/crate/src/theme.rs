//! The look: NDL's stylesheets installed on the display, and which mode is showing.
//!
//! Three sheets are loaded at application priority, bottom to top: the colours of the current mode
//! (`nodalia-<mode>.css`), the metrics of the current density (`nodalia-metrics-<density>.css`) and
//! the widget sheet (`nodalia-gtk.css`), which only reads the other two through `var(--ndl-*)`.
//! Changing mode or density replaces one sheet and leaves the others alone.
//!
//! The system's preference comes from `GtkSettings:gtk-interface-color-scheme` and
//! `gtk-interface-contrast` (GTK 4.20). They are read, never written: an app's own choice is kept
//! here, so the system value is still known when the app goes back to following it.

use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::prelude::*;

const MODES: [(&str, &str); 4] = [
    ("light", include_str!("css/nodalia-light.css")),
    ("dark", include_str!("css/nodalia-dark.css")),
    ("light-hc", include_str!("css/nodalia-light-hc.css")),
    ("dark-hc", include_str!("css/nodalia-dark-hc.css")),
];

const DENSITIES: [(Density, &str); 3] = [
    (
        Density::Compact,
        include_str!("css/nodalia-metrics-compact.css"),
    ),
    (
        Density::Regular,
        include_str!("css/nodalia-metrics-regular.css"),
    ),
    (
        Density::Comfortable,
        include_str!("css/nodalia-metrics-comfortable.css"),
    ),
];

/// The widget sheet: every GTK widget and every widget in this crate, drawn from the variables.
pub const WIDGETS_CSS: &str = include_str!("css/nodalia-gtk.css");

/// The app's choice of light or dark. `Default` follows the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorScheme {
    #[default]
    Default,
    ForceLight,
    ForceDark,
}

/// How tight controls and rows are. Regular is NDL's default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Density {
    Compact,
    #[default]
    Regular,
    Comfortable,
}

/// The mode sheet for a combination, by name. Pure, so it can be tested without a display.
pub fn mode_name(dark: bool, high_contrast: bool) -> &'static str {
    match (dark, high_contrast) {
        (false, false) => "light",
        (true, false) => "dark",
        (false, true) => "light-hc",
        (true, true) => "dark-hc",
    }
}

/// The stylesheet for a mode name, or `None` for a name NDL does not have.
pub fn mode_css(name: &str) -> Option<&'static str> {
    MODES.iter().find(|(n, _)| *n == name).map(|(_, css)| *css)
}

/// The metrics stylesheet for a density.
pub fn density_css(density: Density) -> &'static str {
    DENSITIES
        .iter()
        .find(|(d, _)| *d == density)
        .map_or(DENSITIES[1].1, |(_, css)| *css)
}

mod imp {
    use super::*;
    use glib::subclass::Signal;
    use gtk::subclass::prelude::*;
    use std::sync::OnceLock;

    #[derive(Default)]
    pub struct StyleManager {
        pub scheme: Cell<ColorScheme>,
        pub density: Cell<Density>,
        pub dark: Cell<bool>,
        pub high_contrast: Cell<bool>,
        pub installed: Cell<bool>,
        pub mode: RefCell<gtk::CssProvider>,
        pub metrics: RefCell<gtk::CssProvider>,
        pub widgets: RefCell<gtk::CssProvider>,
        pub accent: RefCell<gtk::CssProvider>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StyleManager {
        const NAME: &'static str = "NdlStyleManager";
        type Type = super::StyleManager;
    }

    impl ObjectImpl for StyleManager {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            // `notify::dark` would need a GObject property; a plain signal is all callers use.
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("dark-changed").build(),
                    Signal::builder("high-contrast-changed").build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    /// Which NDL mode is on screen, and the app's say in it.
    pub struct StyleManager(ObjectSubclass<imp::StyleManager>);
}

thread_local! {
    static DEFAULT: StyleManager = glib::Object::new();
}

impl StyleManager {
    /// The manager for the default display. Installs the stylesheets the first time it is asked
    /// for once a display exists. Named like libadwaita's, which is what apps port from.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        let manager = DEFAULT.with(Clone::clone);
        manager.install();
        manager
    }

    fn install(&self) {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        let imp = self.imp();
        if imp.installed.get() {
            return;
        }
        let Some(display) = gtk::gdk::Display::default() else {
            return;
        };
        imp.installed.set(true);
        let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
        // Above GTK's own theme and below anything the app adds at APPLICATION + n.
        for (provider, offset) in [
            (&imp.mode, 0),
            (&imp.metrics, 0),
            (&imp.widgets, 0),
            (&imp.accent, 0),
        ] {
            gtk::style_context_add_provider_for_display(
                &display,
                &*provider.borrow(),
                priority + offset,
            );
        }
        imp.widgets.borrow().load_from_string(WIDGETS_CSS);
        imp.metrics
            .borrow()
            .load_from_string(density_css(imp.density.get()));

        {
            let settings = gtk::Settings::for_display(&display);
            let this = self.downgrade();
            settings.connect_gtk_interface_color_scheme_notify(move |_| {
                if let Some(this) = this.upgrade() {
                    this.refresh();
                }
            });
            let this = self.downgrade();
            settings.connect_gtk_interface_contrast_notify(move |_| {
                if let Some(this) = this.upgrade() {
                    this.refresh();
                }
            });
        }
        self.refresh();
    }

    fn system_dark() -> bool {
        gtk::Settings::default()
            .is_some_and(|s| s.gtk_interface_color_scheme() == gtk::InterfaceColorScheme::Dark)
    }

    fn system_high_contrast() -> bool {
        gtk::Settings::default()
            .is_some_and(|s| s.gtk_interface_contrast() == gtk::InterfaceContrast::More)
    }

    fn refresh(&self) {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        let imp = self.imp();
        let dark = match imp.scheme.get() {
            ColorScheme::ForceDark => true,
            ColorScheme::ForceLight => false,
            ColorScheme::Default => Self::system_dark(),
        };
        let high_contrast = Self::system_high_contrast();
        // GTK's own theme draws whatever no NDL rule covers; keep it on the same side. The
        // replacement, gtk-interface-color-scheme, is the system's value, which this must not
        // overwrite (see the module header).
        if let Some(settings) = gtk::Settings::default() {
            #[allow(deprecated)]
            settings.set_gtk_application_prefer_dark_theme(dark);
        }
        if let Some(css) = mode_css(mode_name(dark, high_contrast)) {
            imp.mode.borrow().load_from_string(css);
        }
        let dark_changed = imp.dark.replace(dark) != dark;
        let contrast_changed = imp.high_contrast.replace(high_contrast) != high_contrast;
        if dark_changed {
            self.emit_by_name::<()>("dark-changed", &[]);
        }
        if contrast_changed {
            self.emit_by_name::<()>("high-contrast-changed", &[]);
        }
    }

    pub fn set_color_scheme(&self, scheme: ColorScheme) {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        self.imp().scheme.set(scheme);
        self.refresh();
    }

    pub fn color_scheme(&self) -> ColorScheme {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        self.imp().scheme.get()
    }

    pub fn set_density(&self, density: Density) {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        let imp = self.imp();
        imp.density.set(density);
        imp.metrics.borrow().load_from_string(density_css(density));
    }

    /// Replace NDL's accent with the app's own, or go back to NDL's with `None`.
    /// `(background, foreground)`; the foreground must meet contrast on the background.
    pub fn set_accent(&self, colors: Option<(&str, &str)>) {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        let css = match colors {
            Some((bg, fg)) => format!(
                ":root {{ --ndl-color-accent-default: {bg}; --ndl-color-accent-hover: shade({bg}, 0.92); \
                 --ndl-color-accent-pressed: shade({bg}, 0.84); --ndl-color-accent-text: {bg}; \
                 --ndl-color-text-on-accent: {fg}; --ndl-color-border-focus: {bg}; \
                 --ndl-color-accent-subtle: alpha({bg}, 0.16); \
                 --accent-bg-color: {bg}; --accent-fg-color: {fg}; --accent-color: {bg}; }} \
                 @define-color accent_bg_color {bg}; @define-color accent_fg_color {fg}; \
                 @define-color accent_color {bg};"
            ),
            None => String::new(),
        };
        self.imp().accent.borrow().load_from_string(&css);
    }

    pub fn is_dark(&self) -> bool {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        self.imp().dark.get()
    }

    pub fn is_high_contrast(&self) -> bool {
        use gtk::subclass::prelude::ObjectSubclassIsExt;
        self.imp().high_contrast.get()
    }

    pub fn connect_dark_notify<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("dark-changed", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }

    pub fn connect_high_contrast_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("high-contrast-changed", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_mode_has_a_sheet() {
        for dark in [false, true] {
            for hc in [false, true] {
                let css = mode_css(mode_name(dark, hc)).expect("sheet");
                assert!(css.contains("--ndl-color-surface-window"));
            }
        }
        assert!(mode_css("sepia").is_none());
    }

    #[test]
    fn densities_differ_in_control_height() {
        assert!(density_css(Density::Compact).contains("--ndl-density-control-height: 24px;"));
        assert!(density_css(Density::Regular).contains("--ndl-density-control-height: 32px;"));
        assert!(density_css(Density::Comfortable).contains("--ndl-density-control-height: 40px;"));
    }
}
