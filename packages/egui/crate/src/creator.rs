//! The pieces shared by the Nodalia creator apps (Lienzo, Pluma and, later, Toma), so the suite
//! reads as one family: the same workspace selector, the same dialog buttons, the same Home rail
//! and Preferences navigation, the same fields and the same island corners.
//!
//! The specification is `docs/suites/creator.md`. The module only draws: each app passes its
//! labels and acts on the responses by running its own commands. Colours come from the app's
//! [`Palette`], so an app that adjusts NDL's palette (a medium-dark scheme, its accent) keeps it.
//!
//! ```no_run
//! # fn draw(ui: &mut egui::Ui) {
//! use nodalia_theme::creator::{self as cr, Look};
//! use nodalia_theme::{Accent, Mode};
//!
//! let look = Look::new(Mode::Light.palette_with(Accent::Lienzo));
//! if let Some(i) = cr::workspace_selector(ui, &look, "workspace", "Essentials", &["Essentials", "Painting"], 130.0) {
//!     // switch to workspace i
//! }
//! if cr::primary_button(ui, &look, "OK", true).clicked() {
//!     // commit
//! }
//! # }
//! ```

use egui::{
    pos2, vec2, Align2, Color32, CornerRadius, FontFamily, FontId, Frame, Id, Margin, Painter,
    Rect, Response, Role, Sense, Stroke, StrokeKind, Ui, WidgetInfo,
};

use crate::{for_egui::color, icons, Palette};

/// The suite's measures, in points.
pub mod layout {
    /// The app bar (menus, workspace selector, search, window controls).
    pub const APP_BAR: f32 = 44.0;
    /// Between islands, and between an island and the window's edge.
    pub const GAP: f32 = 8.0;
    /// An island's corners.
    pub const ISLAND_RADIUS: u8 = 16;
    /// Buttons, pills, selected rows.
    pub const CONTROL_RADIUS: u8 = 10;
    /// Text fields, dropdowns, tables.
    pub const FIELD_RADIUS: u8 = 8;
    /// Buttons, fields, the workspace selector, tabs.
    pub const CONTROL_HEIGHT: f32 = 28.0;
    /// The narrowest dialog button, so OK / Cancel / Apply line up.
    pub const BUTTON_MIN_WIDTH: f32 = 72.0;
    /// A Preferences category row.
    pub const NAV_ROW: f32 = 30.0;
    /// A Home rail row (Home, Learn).
    pub const HOME_ROW: f32 = 36.0;
    /// The round close button of a dialog.
    pub const CLOSE: f32 = 28.0;
    /// The workspace selector at its widest.
    pub const WORKSPACE_WIDTH: f32 = 130.0;
}

/// The colours and the strong face the shared parts draw with.
#[derive(Clone, Debug, PartialEq)]
pub struct Look {
    pub palette: Palette,
    /// The semibold face for buttons, titles and selected rows. Defaults to the proportional
    /// family; an app with its own semibold face names it with [`Look::strong_family`].
    pub strong: FontFamily,
}

impl Look {
    pub fn new(palette: Palette) -> Self {
        Look {
            palette,
            strong: FontFamily::Proportional,
        }
    }

    /// The app's semibold font family.
    pub fn strong_family(mut self, family: FontFamily) -> Self {
        self.strong = family;
        self
    }

    fn strong_font(&self, size: f32) -> FontId {
        FontId::new(size, self.strong.clone())
    }
}

fn radius(r: u8) -> CornerRadius {
    CornerRadius::same(r)
}

// --- Islands ----------------------------------------------------------------------------------

/// The frame of an island: the panel surface with the suite's corners, no stroke, no shadow.
pub fn island(look: &Look) -> Frame {
    Frame::NONE
        .fill(color(look.palette.surface_panel))
        .corner_radius(radius(layout::ISLAND_RADIUS))
}

/// Rounds the corners of an island something else already painted square (a dock, a canvas):
/// paints the window surface just outside `rect`'s rounded outline, clipped to `rect`.
pub fn round_island_corners(painter: &Painter, look: &Look, rect: Rect) {
    if !rect.is_positive() {
        return;
    }
    let r = f32::from(layout::ISLAND_RADIUS);
    painter.with_clip_rect(rect).rect_stroke(
        rect,
        radius(layout::ISLAND_RADIUS),
        Stroke::new(r, color(look.palette.surface_window)),
        StrokeKind::Outside,
    );
}

// --- Buttons ----------------------------------------------------------------------------------

fn button_rect(
    ui: &mut Ui,
    look: &Look,
    text: &str,
    ink: Color32,
    enabled: bool,
    min_width: f32,
) -> (Rect, Response, std::sync::Arc<egui::Galley>) {
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), look.strong_font(12.5), ink);
    let width = (galley.size().x + 32.0).max(min_width);
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(vec2(width, layout::CONTROL_HEIGHT), sense);
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, enabled && ui.is_enabled(), text));
    (rect, resp, galley)
}

/// The dialog's main action (OK, Create, New file): filled in the primary text colour with the
/// panel colour as ink, so it is black in light themes and white in dark ones. One per dialog,
/// last in the row.
pub fn primary_button(ui: &mut Ui, look: &Look, text: &str, enabled: bool) -> Response {
    primary_button_min(ui, look, text, enabled, layout::BUTTON_MIN_WIDTH)
}

/// [`primary_button`] at least `min_width` wide.
pub fn primary_button_min(
    ui: &mut Ui,
    look: &Look,
    text: &str,
    enabled: bool,
    min_width: f32,
) -> Response {
    let p = &look.palette;
    let ink = if enabled {
        color(p.surface_panel)
    } else {
        color(p.text_disabled)
    };
    let (rect, resp, galley) = button_rect(ui, look, text, ink, enabled, min_width);
    let fill = match (enabled, resp.hovered()) {
        (false, _) => color(p.control_bg_disabled),
        (true, true) => color(p.text_primary).gamma_multiply(0.88),
        (true, false) => color(p.text_primary),
    };
    ui.painter()
        .rect_filled(rect, radius(layout::CONTROL_RADIUS), fill);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, ink);
    resp
}

/// Every other dialog action (Cancel, Apply, Open…, Reset): outlined, filled only under the
/// pointer.
pub fn secondary_button(ui: &mut Ui, look: &Look, text: &str, enabled: bool) -> Response {
    secondary_button_min(ui, look, text, enabled, layout::BUTTON_MIN_WIDTH)
}

/// [`secondary_button`] at least `min_width` wide.
pub fn secondary_button_min(
    ui: &mut Ui,
    look: &Look,
    text: &str,
    enabled: bool,
    min_width: f32,
) -> Response {
    let p = &look.palette;
    let ink = if enabled {
        color(p.text_primary)
    } else {
        color(p.text_disabled)
    };
    let (rect, resp, galley) = button_rect(ui, look, text, ink, enabled, min_width);
    if enabled && resp.hovered() {
        ui.painter().rect_filled(
            rect,
            radius(layout::CONTROL_RADIUS),
            color(p.control_bg_hover),
        );
    }
    ui.painter().rect_stroke(
        rect,
        radius(layout::CONTROL_RADIUS),
        Stroke::new(1.0, color(p.border_default)),
        StrokeKind::Inside,
    );
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, ink);
    resp
}

/// The round close button at the end of a dialog's heading, drawn in `rect`. Returns whether it
/// was clicked.
pub fn close_button(ui: &mut Ui, look: &Look, rect: Rect, id: Id) -> bool {
    let p = &look.palette;
    let resp = ui.interact(rect, id, Sense::click());
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, "Close"));
    let fill = if resp.is_pointer_button_down_on() {
        color(p.control_bg_pressed)
    } else if resp.hovered() {
        color(p.control_bg_hover)
    } else {
        color(p.control_bg)
    };
    let painter = ui.painter();
    painter.circle_filled(rect.center(), rect.width().min(rect.height()) / 2.0, fill);
    let g = Rect::from_center_size(rect.center(), vec2(9.0, 9.0));
    let ink = Stroke::new(1.6, color(p.text_primary));
    painter.line_segment([g.left_top(), g.right_bottom()], ink);
    painter.line_segment([g.right_top(), g.left_bottom()], ink);
    resp.clicked()
}

// --- Navigation -------------------------------------------------------------------------------

/// A Preferences category: a grey pill with the strong text when selected, half of it under the
/// pointer. Fills the width of `ui`.
pub fn nav_row(ui: &mut Ui, look: &Look, text: &str, selected: bool) -> Response {
    let p = &look.palette;
    let width = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(vec2(width, layout::NAV_ROW), Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, selected, text));
    let fill = if selected {
        color(p.control_bg_hover)
    } else if resp.hovered() {
        color(p.control_bg_hover).gamma_multiply(0.5)
    } else {
        Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect, radius(layout::CONTROL_RADIUS), fill);
    let (font, ink) = if selected {
        (look.strong_font(13.0), color(p.text_primary))
    } else {
        (FontId::proportional(13.0), color(p.text_primary))
    };
    ui.painter().text(
        pos2(rect.left() + 12.0, rect.center().y),
        Align2::LEFT_CENTER,
        text,
        font,
        ink,
    );
    resp
}

/// A row of the Home screen's rail (Home, Learn): an NDL icon and a label; the selected row is a
/// pill in the primary text colour, like the primary button. Fills the width of `ui`.
pub fn home_row(ui: &mut Ui, look: &Look, icon: &str, text: &str, selected: bool) -> Response {
    let p = &look.palette;
    let width = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(vec2(width, layout::HOME_ROW), Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, selected, text));
    let ink = if selected {
        ui.painter()
            .rect_filled(rect, radius(layout::CONTROL_RADIUS), color(p.text_primary));
        color(p.surface_panel)
    } else {
        if resp.hovered() {
            ui.painter().rect_filled(
                rect,
                radius(layout::CONTROL_RADIUS),
                color(p.control_bg_hover),
            );
        }
        color(p.text_primary)
    };
    let mut x = rect.left() + 12.0;
    if let Some(icon) = icons::get(icon) {
        let glyph = Rect::from_min_size(pos2(x, rect.center().y - 9.0), vec2(18.0, 18.0));
        icons::paint(ui.painter(), glyph, icon, icons::Style::regular(ink));
        x += 28.0;
    }
    ui.painter().text(
        pos2(x, rect.center().y),
        Align2::LEFT_CENTER,
        text,
        look.strong_font(13.5),
        ink,
    );
    resp
}

/// One tab of a small tab row inside a dialog or panel (Tools / Menu Commands): a grey pill with
/// the strong text when selected.
pub fn tab(ui: &mut Ui, look: &Look, text: &str, selected: bool) -> Response {
    let p = &look.palette;
    let ink = if selected {
        color(p.text_primary)
    } else {
        color(p.text_secondary)
    };
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), look.strong_font(12.5), ink);
    let size = vec2(galley.size().x + 20.0, layout::CONTROL_HEIGHT - 2.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, selected, text));
    let fill = if selected {
        color(p.control_bg_hover)
    } else if resp.hovered() {
        color(p.control_bg_hover).gamma_multiply(0.5)
    } else {
        Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect, radius(layout::CONTROL_RADIUS), fill);
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, ink);
    resp
}

// --- Fields -----------------------------------------------------------------------------------

/// The frame of a text field, dropdown or table: the field surface, a hairline, the field radius.
pub fn field_frame(look: &Look) -> Frame {
    Frame::NONE
        .fill(color(look.palette.control_field_bg))
        .stroke(Stroke::new(1.0, color(look.palette.border_default)))
        .corner_radius(radius(layout::FIELD_RADIUS))
}

/// A search field `width` wide with a magnifier and `hint`. Returns the text edit's response.
pub fn search_field(
    ui: &mut Ui,
    look: &Look,
    text: &mut String,
    hint: &str,
    width: f32,
) -> Response {
    let p = &look.palette;
    field_frame(look)
        .inner_margin(Margin::symmetric(8, 4))
        .show(ui, |ui| {
            ui.set_width(width - 18.0);
            ui.horizontal(|ui| {
                let (glyph, _) = ui.allocate_exact_size(vec2(14.0, 14.0), Sense::hover());
                if let Some(icon) = icons::get("search") {
                    icons::paint(
                        ui.painter(),
                        glyph,
                        icon,
                        icons::Style::regular(color(p.text_secondary)),
                    );
                }
                ui.add(
                    egui::TextEdit::singleline(text)
                        .frame(Frame::NONE)
                        .hint_text(hint)
                        .desired_width(ui.available_width()),
                )
            })
            .inner
        })
        .inner
}

/// The app bar's workspace selector: the current workspace in a field-shaped pill with a chevron,
/// opening the list of `options` (presets, then saved workspaces) with a check by `current`.
/// Returns the index picked.
pub fn workspace_selector(
    ui: &mut Ui,
    look: &Look,
    id: &str,
    current: &str,
    options: &[&str],
    width: f32,
) -> Option<usize> {
    let p = &look.palette;
    let (rect, resp) = ui.allocate_exact_size(vec2(width, layout::CONTROL_HEIGHT), Sense::click());
    let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
    resp.widget_info(|| WidgetInfo::labeled(Role::ComboBox, true, current));
    let popup_id = Id::NULL.with(("ndl_creator_workspace", id));
    let open = egui::Popup::is_id_open(ui.ctx(), popup_id);
    let fill = if open || resp.hovered() {
        color(p.control_bg_hover)
    } else {
        color(p.control_field_bg)
    };
    let painter = ui.painter();
    painter.rect(
        rect,
        radius(layout::FIELD_RADIUS),
        fill,
        Stroke::new(1.0, color(p.border_subtle)),
        StrokeKind::Inside,
    );
    let chevron =
        Rect::from_center_size(pos2(rect.right() - 16.0, rect.center().y), vec2(12.0, 12.0));
    if let Some(icon) = icons::get("chevron-down") {
        icons::paint(
            painter,
            chevron,
            icon,
            icons::Style::bold(color(p.text_secondary)),
        );
    }
    let mut job = egui::text::LayoutJob::simple_singleline(
        current.to_owned(),
        FontId::proportional(13.0),
        color(p.text_primary),
    );
    job.wrap = egui::text::TextWrapping::truncate_at_width(rect.width() - 40.0);
    let galley = painter.layout_job(job);
    painter.galley(
        pos2(rect.left() + 12.0, rect.center().y - galley.size().y / 2.0),
        galley,
        color(p.text_primary),
    );
    let mut picked = None;
    egui::Popup::menu(&resp).id(popup_id).show(|ui| {
        ui.set_min_width(width.max(180.0));
        for (i, name) in options.iter().enumerate() {
            let row = ui.horizontal(|ui| {
                let (mark, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                if *name == current {
                    if let Some(icon) = icons::get("check") {
                        icons::paint(
                            ui.painter(),
                            mark,
                            icon,
                            icons::Style::bold(color(p.text_primary)),
                        );
                    }
                }
                ui.selectable_label(false, *name)
            });
            if row.inner.clicked() {
                picked = Some(i);
                ui.close();
            }
        }
    });
    picked
}
