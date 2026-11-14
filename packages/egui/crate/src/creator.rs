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
//! let names = ["Essentials", "Painting"];
//! if let Some(cr::WorkspacePick::Workspace(i)) =
//!     cr::workspace_selector(ui, &look, "workspace", "Essentials", &names, names.len(), &[], 130.0)
//! {
//!     // switch to names[i]
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

pub mod home;

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
    /// Fields, the workspace selector, tabs.
    pub const CONTROL_HEIGHT: f32 = 28.0;
    /// Dialog buttons (OK, Cancel, New file, Open…).
    pub const BUTTON_HEIGHT: f32 = 30.0;
    /// The narrowest dialog button, so OK / Cancel / Apply line up.
    pub const BUTTON_MIN_WIDTH: f32 = 72.0;
    /// A Preferences category row.
    pub const NAV_ROW: f32 = 28.0;
    /// A Home rail row (Home, Learn).
    pub const HOME_ROW: f32 = 32.0;
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
    /// The medium face for button labels and tabs. Defaults to the proportional family; an app
    /// with its own medium face names it with [`Look::medium_family`].
    pub medium: FontFamily,
}

impl Look {
    pub fn new(palette: Palette) -> Self {
        Look {
            palette,
            strong: FontFamily::Proportional,
            medium: FontFamily::Proportional,
        }
    }

    /// The app's semibold font family.
    pub fn strong_family(mut self, family: FontFamily) -> Self {
        self.strong = family;
        self
    }

    /// The app's medium font family.
    pub fn medium_family(mut self, family: FontFamily) -> Self {
        self.medium = family;
        self
    }

    fn medium_font(&self, size: f32) -> FontId {
        FontId::new(size, self.medium.clone())
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
        .layout_no_wrap(text.to_owned(), look.medium_font(13.0), ink);
    let width = (galley.size().x + 28.0).max(min_width);
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(vec2(width, layout::BUTTON_HEIGHT), sense);
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

/// `component.window.close`: the close button's coral under the pointer, in every mode.
const WINDOW_CLOSE: Color32 = Color32::from_rgb(0xff, 0x5c, 0x4d);

/// The round close button at the end of a dialog's heading, drawn in `rect`: the window close
/// button's disc and × (NDL's `window-close` on its 24 pt grid, 2 pt strokes with round ends)
/// scaled to `rect`, coral under the pointer. Returns whether it was clicked.
pub fn close_button(ui: &mut Ui, look: &Look, rect: Rect, id: Id) -> bool {
    let p = &look.palette;
    let resp = ui.interact(rect, id, Sense::click());
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, "Close"));
    let pressed = resp.is_pointer_button_down_on() && resp.hovered();
    let (mut fill, ink) = if resp.hovered() || pressed {
        (WINDOW_CLOSE, Color32::WHITE)
    } else {
        (color(p.control_bg), color(p.text_secondary))
    };
    if pressed {
        fill = fill.gamma_multiply(0.82).to_opaque();
    }
    let k = rect.height().min(rect.width()) / 28.0;
    let ppp = ui.ctx().pixels_per_point();
    let centre = pos2(
        (rect.center().x * ppp).round() / ppp,
        (rect.center().y * ppp).round() / ppp,
    );
    // The antialiased rim falls just outside `rect`, where a dialog clips its content.
    let painter = ui
        .painter()
        .with_clip_rect(ui.clip_rect().union(rect.expand(2.0)));
    painter.circle_filled(centre, 14.0 * k, fill);
    let at = |x: f32, y: f32| centre + vec2(x - 12.0, y - 12.0) * k;
    let stroke = Stroke::new(2.0 * k, ink);
    for (a, b) in [
        (at(8.0, 8.0), at(16.0, 16.0)),
        (at(16.0, 8.0), at(8.0, 16.0)),
    ] {
        painter.line_segment([a, b], stroke);
        painter.circle_filled(a, k, ink);
        painter.circle_filled(b, k, ink);
    }
    resp.clicked()
}

// --- Navigation -------------------------------------------------------------------------------

/// A Preferences category: a grey pill when selected, half of it under the pointer, the label
/// in the primary text when selected and the secondary one otherwise (tertiary when `muted`: a
/// section with nothing to show). A label wider than the row is elided, with the whole of it as
/// the tooltip. Fills the width of `ui`.
pub fn nav_row(ui: &mut Ui, look: &Look, text: &str, selected: bool, muted: bool) -> Response {
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
    let ink = if selected || resp.hovered() {
        color(p.text_primary)
    } else if muted {
        color(p.text_tertiary)
    } else {
        color(p.text_secondary)
    };
    let mut job =
        egui::text::LayoutJob::simple_singleline(text.to_owned(), look.medium_font(13.0), ink);
    job.wrap = egui::text::TextWrapping::truncate_at_width(rect.width() - 20.0);
    let galley = ui.painter().layout_job(job);
    let elided = galley.elided;
    ui.painter().galley(
        pos2(rect.left() + 12.0, rect.center().y - galley.size().y / 2.0),
        galley,
        ink,
    );
    if elided {
        resp.on_hover_text(text)
    } else {
        resp
    }
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
            color(p.text_primary)
        } else {
            color(p.text_secondary)
        }
    };
    if let Some(icon) = icons::get(icon) {
        let glyph =
            Rect::from_center_size(pos2(rect.left() + 20.0, rect.center().y), vec2(16.0, 16.0));
        icons::paint(ui.painter(), glyph, icon, icons::Style::regular(ink));
    }
    ui.painter().text(
        pos2(rect.left() + 34.0, rect.center().y),
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

/// What the [`workspace_selector`] list was used for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkspacePick {
    /// The workspace at this index of `workspaces`.
    Workspace(usize),
    /// The action at this index of `actions` (Reset, New Workspace…, Manage Workspaces…).
    Action(usize),
}

/// The app bar's workspace selector: the current workspace in a field-shaped pill with a chevron.
/// Its list shows `workspaces` (presets, then the user's own from `saved_from`, after a line) with
/// a check by `current`, then `actions` after a line.
#[allow(clippy::too_many_arguments)]
pub fn workspace_selector(
    ui: &mut Ui,
    look: &Look,
    id: &str,
    current: &str,
    workspaces: &[&str],
    saved_from: usize,
    actions: &[&str],
    width: f32,
) -> Option<WorkspacePick> {
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
        ui.set_min_width(width.max(200.0));
        let row = |ui: &mut Ui, text: &str, checked: bool| {
            ui.horizontal(|ui| {
                let (mark, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                if checked {
                    if let Some(icon) = icons::get("check") {
                        icons::paint(
                            ui.painter(),
                            mark,
                            icon,
                            icons::Style::bold(color(p.text_primary)),
                        );
                    }
                }
                ui.add(egui::Button::new(text).frame(false))
            })
            .inner
            .clicked()
        };
        for (i, name) in workspaces.iter().enumerate() {
            if i == saved_from && i > 0 {
                ui.separator();
            }
            if row(ui, name, *name == current) {
                picked = Some(WorkspacePick::Workspace(i));
                ui.close();
            }
        }
        if !actions.is_empty() {
            ui.separator();
        }
        for (i, name) in actions.iter().enumerate() {
            if row(ui, name, false) {
                picked = Some(WorkspacePick::Action(i));
                ui.close();
            }
        }
    });
    picked
}
