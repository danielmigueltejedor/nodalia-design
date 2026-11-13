//! The File view (Backstage) shared by the Nodalia office apps (Letra, Celdas, Lámina), so the
//! three read as one bundle: a navigation island on the left, the page in an island beside it,
//! a Home page with new-document thumbnails over the recent files, and Preferences as one
//! scrolling page of sections.
//!
//! The module only draws. Each app supplies its entries, templates and recent files, and acts
//! on what is returned (an entry id, a template index, a recent index) by running its own
//! commands. Nothing here opens files or knows about documents.
//!
//! ```no_run
//! # fn draw(ui: &mut egui::Ui) {
//! use nodalia_theme::office::{self, Entry, Look, Nav};
//! use nodalia_theme::{Accent, Mode};
//!
//! let look = Look::new(Mode::Light.palette_with(Accent::Letra));
//! let nav = Nav {
//!     back: "Back",
//!     top: vec![Entry::item("home", "Home", "home"), Entry::Separator, Entry::item("save", "Save", "save")],
//!     bottom: vec![Entry::item("options", "Preferences", "settings")],
//!     selected: Some("home"),
//! };
//! let picked = office::file_view(ui, &look, &nav, |ui| {
//!     office::page(ui, &look, "Home", |ui| {
//!         office::section(ui, &look, "Recent", |ui| {
//!             office::recents(ui, &look, &[], "No recent documents yet.", None);
//!         });
//!     });
//! });
//! # let _ = picked;
//! # }
//! ```

use egui::{
    pos2, vec2, Align2, CornerRadius, FontFamily, FontId, Frame, Margin, Rect, Response, RichText,
    Role, Sense, Stroke, StrokeKind, Ui, WidgetInfo,
};

use crate::{component, for_egui::color, icons, Palette};

/// Width of a navigation row.
pub const NAV_WIDTH: f32 = 196.0;
/// Height of a navigation row.
pub const NAV_ROW: f32 = 38.0;
/// Width of the page's content column.
pub const CONTENT_WIDTH: f32 = 760.0;
/// Gap between the islands and from the window's edges.
pub const GAP: f32 = 8.0;
/// Width of the controls in a [`setting`] row.
pub const CONTROL_WIDTH: f32 = 220.0;
/// A new-document thumbnail.
pub const THUMB: egui::Vec2 = egui::Vec2::new(150.0, 104.0);

/// Radius of the two islands.
const ISLAND_RADIUS: f32 = 16.0;
/// Radius of a navigation row.
const ROW_RADIUS: f32 = 10.0;

/// The colours and the two font families the view draws with. Apps register their own fonts, so
/// they name the families; [`Look::new`] uses egui's proportional family for both.
#[derive(Clone, Debug, PartialEq)]
pub struct Look {
    pub palette: Palette,
    pub regular: FontFamily,
    pub bold: FontFamily,
}

impl Look {
    pub fn new(palette: Palette) -> Self {
        Look {
            palette,
            regular: FontFamily::Proportional,
            bold: FontFamily::Proportional,
        }
    }

    /// Names the app's bold family (for headings and the selected entry).
    pub fn with_bold(mut self, bold: FontFamily) -> Self {
        self.bold = bold;
        self
    }

    fn font(&self, size: f32) -> FontId {
        FontId::new(size, self.regular.clone())
    }

    fn bold_font(&self, size: f32) -> FontId {
        FontId::new(size, self.bold.clone())
    }
}

/// One line of the navigation column.
#[derive(Clone, Debug, PartialEq)]
pub enum Entry<'a> {
    Item {
        /// What [`file_view`] returns when it is clicked.
        id: &'a str,
        label: &'a str,
        /// An NDL icon name; an unknown name draws no icon.
        icon: &'a str,
    },
    Separator,
}

impl<'a> Entry<'a> {
    pub fn item(id: &'a str, label: &'a str, icon: &'a str) -> Self {
        Entry::Item { id, label, icon }
    }
}

/// The navigation column: Back, the app's entries from the top, and a group held at the bottom
/// (Account, Preferences).
#[derive(Clone, Debug, PartialEq)]
pub struct Nav<'a> {
    /// The label of the first row, which leaves the view. Empty hides the row, for a start
    /// screen with nothing to go back to.
    pub back: &'a str,
    pub top: Vec<Entry<'a>>,
    /// Drawn from the bottom of the island, in the order given (top to bottom).
    pub bottom: Vec<Entry<'a>>,
    /// The entry of the page being shown.
    pub selected: Option<&'a str>,
}

/// The id [`file_view`] returns for the Back row.
pub const BACK: &str = "back";

/// Draws the whole view in `ui`: the navigation island and, beside it, the page island holding a
/// vertical scroll area with `page` inside. Returns the id of the entry clicked this frame
/// ([`BACK`] for the Back row).
pub fn file_view(
    ui: &mut Ui,
    look: &Look,
    nav: &Nav,
    page: impl FnOnce(&mut Ui),
) -> Option<String> {
    let p = &look.palette;
    let mut picked = None;
    let island = |inner: Margin, outer: Margin| {
        Frame::NONE
            .fill(color(p.surface_panel))
            .inner_margin(inner)
            .outer_margin(outer)
            .corner_radius(CornerRadius::same(ISLAND_RADIUS as u8))
    };
    let g = GAP as i8;
    egui::Panel::left("ndl_office_nav")
        .exact_size(NAV_WIDTH + 16.0 + GAP)
        .resizable(false)
        .show_separator_line(false)
        .frame(island(
            Margin {
                left: 8,
                right: 8,
                top: 10,
                bottom: 10,
            },
            Margin {
                left: g,
                right: 0,
                top: 0,
                bottom: g,
            },
        ))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            if nav.back.is_empty() {
                // Nothing to go back to (a start screen): keep the row's place.
                ui.add_space(NAV_ROW);
            } else if nav_row(ui, look, "arrow-left", nav.back, false).clicked() {
                picked = Some(BACK.to_string());
            }
            ui.add_space(10.0);
            entries(ui, look, &nav.top, nav.selected, &mut picked);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                let reversed: Vec<Entry> = nav.bottom.iter().rev().cloned().collect();
                entries(ui, look, &reversed, nav.selected, &mut picked);
            });
        });
    egui::CentralPanel::default()
        .frame(island(
            Margin::ZERO,
            Margin {
                left: g,
                right: g,
                top: 0,
                bottom: g,
            },
        ))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("ndl_office_page")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    Frame::NONE
                        .inner_margin(Margin {
                            left: 44,
                            right: 32,
                            top: 26,
                            bottom: 32,
                        })
                        .show(ui, |ui| {
                            // `set_max_width` can also widen: in a window narrower than the
                            // column it would push the page past the island's right edge.
                            ui.set_max_width(ui.available_width().min(CONTENT_WIDTH));
                            page(ui);
                        });
                });
        });
    picked
}

fn entries(
    ui: &mut Ui,
    look: &Look,
    list: &[Entry],
    selected: Option<&str>,
    picked: &mut Option<String>,
) {
    for entry in list {
        match entry {
            Entry::Item { id, label, icon } => {
                if nav_row(ui, look, icon, label, selected == Some(*id)).clicked() {
                    *picked = Some((*id).to_string());
                }
            }
            Entry::Separator => separator(ui, look),
        }
    }
}

/// One navigation row: selected rows in the accent's soft tint with accent ink and a bold label.
pub fn nav_row(ui: &mut Ui, look: &Look, icon: &str, label: &str, selected: bool) -> Response {
    let p = &look.palette;
    let (rect, resp) = ui.allocate_exact_size(vec2(NAV_WIDTH, NAV_ROW), Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Button, true, selected, label));
    let painter = ui.painter();
    let ink = if selected {
        painter.rect_filled(rect, ROW_RADIUS, color(p.accent_subtle));
        color(p.accent_text)
    } else {
        if resp.hovered() {
            painter.rect_filled(rect, ROW_RADIUS, color(p.control_bg_hover));
        }
        color(p.text_primary)
    };
    if let Some(icon) = icons::get(icon) {
        let glyph =
            Rect::from_center_size(pos2(rect.left() + 22.0, rect.center().y), vec2(18.0, 18.0));
        icons::paint(painter, glyph, icon, icons::Style::bold(ink));
    }
    let font = if selected {
        look.bold_font(14.0)
    } else {
        look.font(14.0)
    };
    painter.text(
        pos2(rect.left() + 42.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        font,
        ink,
    );
    resp
}

fn separator(ui: &mut Ui, look: &Look) {
    let (rect, _) = ui.allocate_exact_size(vec2(NAV_WIDTH, 17.0), Sense::hover());
    let y = rect.center().y;
    ui.painter().line_segment(
        [pos2(rect.left() + 12.0, y), pos2(rect.right() - 12.0, y)],
        Stroke::new(1.0, color(look.palette.border_subtle)),
    );
}

/// A page: its 28 pt title, then `body`.
pub fn page<R>(ui: &mut Ui, look: &Look, title: &str, body: impl FnOnce(&mut Ui) -> R) -> R {
    ui.label(
        RichText::new(title)
            .font(look.font(28.0))
            .color(color(look.palette.text_primary)),
    );
    ui.add_space(18.0);
    body(ui)
}

/// A section of a page: a 15 pt bold heading, `body`, and the space before the next section.
pub fn section<R>(ui: &mut Ui, look: &Look, heading: &str, body: impl FnOnce(&mut Ui) -> R) -> R {
    ui.label(
        RichText::new(heading)
            .font(look.bold_font(15.0))
            .color(color(look.palette.text_primary)),
    );
    ui.add_space(8.0);
    let out = body(ui);
    ui.add_space(18.0);
    out
}

/// Secondary text under a setting or as an empty state.
pub fn hint(ui: &mut Ui, look: &Look, text: &str) {
    ui.label(
        RichText::new(text)
            .font(look.font(12.5))
            .color(color(look.palette.text_secondary)),
    );
}

/// A Preferences row: the label on the left, `control` in a column [`CONTROL_WIDTH`] wide.
/// Rows in a [`settings`] grid line their controls up.
pub fn setting<R>(ui: &mut Ui, look: &Look, label: &str, control: impl FnOnce(&mut Ui) -> R) -> R {
    ui.label(
        RichText::new(label)
            .font(look.font(13.5))
            .color(color(look.palette.text_primary)),
    );
    let out = ui
        .allocate_ui(vec2(CONTROL_WIDTH, ui.spacing().interact_size.y), |ui| {
            ui.set_width(CONTROL_WIDTH);
            ui.spacing_mut().combo_width = CONTROL_WIDTH;
            control(ui)
        })
        .inner;
    ui.end_row();
    out
}

/// A two-column grid for [`setting`] rows, so their controls start at the same x.
pub fn settings<R>(ui: &mut Ui, id: &str, rows: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Grid::new(("ndl_office_settings", id))
        .num_columns(2)
        .spacing(vec2(24.0, 8.0))
        .show(ui, rows)
        .inner
}

/// A new-document thumbnail the app draws itself (a page, a sheet, a slide) in its accent.
pub struct Template<'a> {
    pub name: &'a str,
    /// Paints the thumbnail inside the given rect (already inset from the border).
    pub draw: &'a dyn Fn(&egui::Painter, Rect),
}

/// A row of new-document thumbnails, each [`THUMB`] with its name under it, wrapping to the
/// width. Returns the index of the one clicked.
pub fn templates(ui: &mut Ui, look: &Look, list: &[Template]) -> Option<usize> {
    let p = &look.palette;
    let mut picked = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(18.0, 14.0);
        for (i, template) in list.iter().enumerate() {
            let (rect, resp) =
                ui.allocate_exact_size(vec2(THUMB.x, THUMB.y + 36.0), Sense::click());
            resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, template.name));
            let painter = ui.painter();
            let thumb = Rect::from_min_size(rect.min, THUMB);
            painter.rect_filled(thumb, 6.0, color(p.surface_raised));
            (template.draw)(
                &painter.with_clip_rect(thumb.shrink(1.0)),
                thumb.shrink(10.0),
            );
            let edge = if resp.hovered() {
                Stroke::new(2.0, color(p.accent_default))
            } else {
                Stroke::new(1.0, color(p.border_subtle))
            };
            painter.rect_stroke(thumb, 6.0, edge, StrokeKind::Inside);
            let ink = color(p.text_primary);
            let galley = painter.layout(
                template.name.to_string(),
                look.font(12.5),
                ink,
                rect.width(),
            );
            painter.galley(pos2(rect.left(), thumb.bottom() + 6.0), galley, ink);
            if resp.clicked() {
                picked = Some(i);
            }
        }
    });
    picked
}

/// A recent file: its name, the folder it is in, and when it was last changed (already in words).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Recent<'a> {
    pub name: &'a str,
    pub folder: &'a str,
    pub when: &'a str,
    /// Shown on hover: the full path.
    pub path: &'a str,
}

/// Recent files, newest first. `empty` is shown when there are none; `modified` heads the date
/// column (`None` hides the column). Returns the index of the one clicked.
pub fn recents(
    ui: &mut Ui,
    look: &Look,
    list: &[Recent],
    empty: &str,
    modified: Option<&str>,
) -> Option<usize> {
    let p = &look.palette;
    if list.is_empty() {
        hint(ui, look, empty);
        return None;
    }
    let width = ui.available_width().min(CONTENT_WIDTH);
    let column = if modified.is_some() && width >= 420.0 {
        150.0
    } else {
        0.0
    };
    if let (Some(head), true) = (modified, column > 0.0) {
        let (rect, _) = ui.allocate_exact_size(vec2(width, 20.0), Sense::hover());
        ui.painter().text(
            pos2(rect.right() - column, rect.center().y),
            Align2::LEFT_CENTER,
            head,
            look.font(11.5),
            color(p.text_secondary),
        );
    }
    let mut picked = None;
    for (i, recent) in list.iter().enumerate() {
        let (rect, resp) = ui.allocate_exact_size(vec2(width, 44.0), Sense::click());
        resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, recent.name));
        let painter = ui.painter();
        if resp.hovered() {
            painter.rect_filled(rect, component::CONTROL_RADIUS, color(p.control_bg_hover));
        }
        if let Some(icon) = icons::get("file-text") {
            let glyph =
                Rect::from_center_size(pos2(rect.left() + 20.0, rect.center().y), vec2(20.0, 20.0));
            icons::paint(
                painter,
                glyph,
                icon,
                icons::Style::bold(color(p.accent_text)),
            );
        }
        let room = if column > 0.0 {
            rect.right() - column - 14.0
        } else {
            rect.right()
        };
        let text = painter.with_clip_rect(
            Rect::from_min_max(rect.min, pos2(room, rect.bottom())).intersect(painter.clip_rect()),
        );
        text.text(
            pos2(rect.left() + 42.0, rect.center().y - 8.0),
            Align2::LEFT_CENTER,
            recent.name,
            look.font(13.5),
            color(p.text_primary),
        );
        text.text(
            pos2(rect.left() + 42.0, rect.center().y + 9.0),
            Align2::LEFT_CENTER,
            recent.folder,
            look.font(11.5),
            color(p.text_secondary),
        );
        if column > 0.0 {
            painter.text(
                pos2(rect.right() - column, rect.center().y),
                Align2::LEFT_CENTER,
                recent.when,
                look.font(12.0),
                color(p.text_secondary),
            );
        }
        let resp = if recent.path.is_empty() {
            resp
        } else {
            resp.on_hover_text(recent.path)
        };
        if resp.clicked() {
            picked = Some(i);
        }
    }
    picked
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Accent, Mode};

    fn run(f: impl FnMut(&mut Ui)) {
        let ctx = egui::Context::default();
        let mut f = f;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1200.0, 800.0))),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| f(ui));
        output.textures_delta.clear();
    }

    #[test]
    fn the_view_draws_every_entry_and_its_page() {
        let look = Look::new(Mode::Dark.palette_with(Accent::Celdas));
        let nav = Nav {
            back: "Back",
            top: vec![
                Entry::item("home", "Home", "home"),
                Entry::Separator,
                Entry::item("x", "Unknown icon", "no-such-icon"),
            ],
            bottom: vec![
                Entry::item("account", "Account", "user"),
                Entry::item("options", "Preferences", "settings"),
            ],
            selected: Some("options"),
        };
        let mut drawn = false;
        run(|ui| {
            let picked = file_view(ui, &look, &nav, |ui| {
                page(ui, &look, "Preferences", |ui| {
                    section(ui, &look, "General", |ui| {
                        settings(ui, "general", |ui| {
                            setting(ui, &look, "Display theme", |ui| ui.label("Light"));
                        });
                    });
                    let blank = |p: &egui::Painter, r: Rect| {
                        p.rect_filled(r, 0.0, egui::Color32::WHITE);
                    };
                    assert_eq!(
                        templates(
                            ui,
                            &look,
                            &[Template {
                                name: "Blank",
                                draw: &blank
                            }]
                        ),
                        None
                    );
                    let list = [Recent {
                        name: "a.docx",
                        folder: "/tmp",
                        when: "now",
                        path: "/tmp/a.docx",
                    }];
                    assert_eq!(
                        recents(ui, &look, &list, "None yet.", Some("Modified")),
                        None
                    );
                    assert_eq!(recents(ui, &look, &[], "None yet.", None), None);
                });
                drawn = true;
            });
            assert_eq!(picked, None);
        });
        assert!(drawn);
    }

    #[test]
    fn a_narrow_window_keeps_the_page_inside_and_an_empty_back_hides_the_row() {
        let look = Look::new(Mode::Light.palette_with(Accent::Lamina));
        let nav = Nav {
            back: "",
            top: vec![Entry::item("home", "Home", "home")],
            bottom: vec![],
            selected: Some("home"),
        };
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(700.0, 600.0))),
            ..Default::default()
        };
        let mut page_right = 0.0;
        let mut picked = None;
        let mut output = ctx.run_ui(input, |ui| {
            picked = file_view(ui, &look, &nav, |ui| {
                let (r, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 10.0), Sense::hover());
                page_right = r.right();
            });
        });
        output.textures_delta.clear();
        assert!(
            page_right <= 700.0 - GAP,
            "the page column ends at {page_right}, past the island"
        );
        assert_eq!(picked, None, "no Back row to click");
    }

    #[test]
    fn the_icons_the_office_apps_name_exist() {
        for name in [
            "arrow-left",
            "home",
            "file-text",
            "folder",
            "info",
            "save",
            "save-as",
            "print",
            "export",
            "share",
            "close",
            "user",
            "settings",
        ] {
            assert!(icons::get(name).is_some(), "{name}");
        }
    }
}
