//! The creator suite's Home screen: one island with the rail on the left (New file, Open…, Home,
//! Learn and the links at the foot) and the page beside it (the app's mark and name, Recent Files
//! as cards or a list, and whatever the app adds below). Lienzo, Pluma and Toma draw it with this
//! code, so it is the same screen in all three.
//!
//! The module only draws: each app passes its labels and its recent files (with a thumbnail when
//! it has one) and acts on what is picked by running its own commands.
//!
//! ```no_run
//! # fn draw(ui: &mut egui::Ui, rect: egui::Rect) {
//! use nodalia_theme::creator::{home, Look};
//! use nodalia_theme::{Accent, Mode};
//!
//! let look = Look::new(Mode::Light.palette_with(Accent::Pluma));
//! let rail = home::Rail {
//!     new_file: "New file",
//!     open: "Open…",
//!     home: "Home",
//!     learn: Some("Learn"),
//!     links: &[home::Link { icon: "globe", label: "Website", url: "https://getnodalia.com/pluma" }],
//! };
//! let mark = |_: &egui::Painter, _: egui::Rect| {};
//! let picked = home::view(ui, &look, rect, "Pluma", &mark, &rail, |ui| {
//!     home::section_heading(ui, &look, "Quickly start a new file");
//! });
//! # let _ = picked;
//! # }
//! ```

use egui::{
    pos2, vec2, Align2, Color32, CornerRadius, FontId, Galley, Painter, Rect, Response, Role,
    Sense, Stroke, StrokeKind, TextureId, Ui, Vec2, WidgetInfo,
};
use std::sync::Arc;

use super::{home_row, layout, primary_button_min, radius, secondary_button_min, Look};
use crate::{for_egui::color, icons};

/// The Home screen's measures, in points.
pub mod measures {
    /// Narrower than this, the rail folds into a row of buttons above the page.
    pub const NARROW: f32 = 640.0;
    /// The rail's width.
    pub const RAIL: f32 = 170.0;
    /// New file and Open… in the rail.
    pub const RAIL_BUTTON: f32 = 130.0;
    /// Between the rail and the page.
    pub const RAIL_GAP: f32 = 36.0;
    /// The screen's side and top margins, wide and narrow.
    pub const PAD: (f32, f32) = (36.0, 54.0);
    pub const PAD_NARROW: (f32, f32) = (16.0, 24.0);
    /// A recent file's card.
    pub const CARD: (f32, f32) = (184.0, 236.0);
    /// The preview well at the top of a card.
    pub const PREVIEW: f32 = 164.0;
    /// A recent file's row in the list.
    pub const ROW: f32 = 36.0;
    /// Between cards.
    pub const CARD_GAP: f32 = 16.0;
}

/// A link at the rail's foot.
#[derive(Clone, Debug, PartialEq)]
pub struct Link<'a> {
    /// An NDL icon name.
    pub icon: &'a str,
    pub label: &'a str,
    /// Shown on hover; the app opens it.
    pub url: &'a str,
}

/// The rail's labels. `learn: None` hides the Learn row.
#[derive(Clone, Debug, PartialEq)]
pub struct Rail<'a> {
    pub new_file: &'a str,
    pub open: &'a str,
    pub home: &'a str,
    pub learn: Option<&'a str>,
    pub links: &'a [Link<'a>],
}

/// What was clicked in the rail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pick {
    NewFile,
    Open,
    /// The Home row: back to the open documents, when there are any.
    Home,
    Learn,
    /// The index into [`Rail::links`].
    Link(usize),
}

/// Draws the Home screen over `rect`: the island, the rail and the page with the app's mark (a
/// painter callback, 28 points square) and name, then `body` (Recent Files, the app's own
/// sections). Narrower than [`measures::NARROW`], the rail's buttons sit above the page and the
/// whole screen scrolls.
pub fn view(
    ui: &mut Ui,
    look: &Look,
    rect: Rect,
    app: &str,
    mark: &dyn Fn(&Painter, Rect),
    rail: &Rail,
    body: impl FnOnce(&mut Ui),
) -> Option<Pick> {
    let p = &look.palette;
    ui.painter()
        .rect_filled(rect, radius(layout::ISLAND_RADIUS), color(p.surface_panel));
    let mut picked = None;
    let narrow = rect.width() < measures::NARROW;
    let (pad_x, pad_y) = if narrow {
        measures::PAD_NARROW
    } else {
        measures::PAD
    };
    let inner = rect.shrink2(vec2(pad_x, 0.0));
    let top_down = egui::Layout::top_down(egui::Align::Min);
    if narrow {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner).layout(top_down));
        egui::ScrollArea::vertical()
            .id_salt("ndl_creator_home")
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                ui.add_space(pad_y);
                ui.horizontal(|ui| actions(ui, look, rail, &mut picked));
                ui.add_space(20.0);
                page(ui, look, app, mark, body);
            });
        return picked;
    }
    let rail_rect = Rect::from_min_size(
        pos2(inner.left(), rect.top() + pad_y),
        vec2(measures::RAIL, (rect.height() - 2.0 * pad_y).max(0.0)),
    );
    let page_rect = Rect::from_min_max(
        pos2(rail_rect.right() + measures::RAIL_GAP, rect.top()),
        pos2(inner.right(), rect.bottom() - 16.0),
    );
    let mut rail_ui = ui.new_child(egui::UiBuilder::new().max_rect(rail_rect).layout(top_down));
    side_rail(&mut rail_ui, look, rail, &mut picked);
    let mut page_ui = ui.new_child(egui::UiBuilder::new().max_rect(page_rect).layout(top_down));
    egui::ScrollArea::vertical()
        .id_salt("ndl_creator_home")
        .auto_shrink([false, false])
        .show(&mut page_ui, |ui| {
            ui.add_space(pad_y);
            page(ui, look, app, mark, body);
            ui.add_space(24.0);
        });
    picked
}

fn actions(ui: &mut Ui, look: &Look, rail: &Rail, picked: &mut Option<Pick>) {
    let w = measures::RAIL_BUTTON;
    if primary_button_min(ui, look, rail.new_file, true, w)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        *picked = Some(Pick::NewFile);
    }
    ui.add_space(8.0);
    if secondary_button_min(ui, look, rail.open, true, w)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        *picked = Some(Pick::Open);
    }
}

fn side_rail(ui: &mut Ui, look: &Look, rail: &Rail, picked: &mut Option<Pick>) {
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.add_space(8.0);
    actions(ui, look, rail, picked);
    ui.add_space(16.0);
    separator(ui, look, measures::RAIL);
    ui.add_space(14.0);
    if home_row(ui, look, "home", rail.home, true)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        *picked = Some(Pick::Home);
    }
    if let Some(learn) = rail.learn {
        ui.add_space(4.0);
        if home_row(ui, look, "sparkle", learn, false)
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
        {
            *picked = Some(Pick::Learn);
        }
    }
    let foot = 9.0 + rail.links.len() as f32 * 30.0;
    ui.add_space((ui.available_height() - foot).max(24.0));
    separator(ui, look, measures::RAIL);
    ui.add_space(8.0);
    for (i, link) in rail.links.iter().enumerate() {
        if rail_link(ui, look, link).clicked() {
            *picked = Some(Pick::Link(i));
        }
        ui.add_space(2.0);
    }
}

/// A link at the rail's foot: secondary ink, a soft pill under the pointer, its address on hover.
fn rail_link(ui: &mut Ui, look: &Look, link: &Link) -> Response {
    let p = &look.palette;
    let (rect, resp) = ui.allocate_exact_size(
        vec2(ui.available_width(), layout::CONTROL_HEIGHT),
        Sense::click(),
    );
    resp.widget_info(|| WidgetInfo::labeled(Role::Link, true, link.label));
    let ink = if resp.hovered() {
        ui.painter().rect_filled(
            rect,
            radius(layout::CONTROL_RADIUS),
            color(p.control_bg_hover),
        );
        color(p.text_primary)
    } else {
        color(p.text_secondary)
    };
    if let Some(icon) = icons::get(link.icon) {
        let glyph =
            Rect::from_center_size(pos2(rect.left() + 20.0, rect.center().y), vec2(14.0, 14.0));
        icons::paint(ui.painter(), glyph, icon, icons::Style::regular(ink));
    }
    ui.painter().text(
        pos2(rect.left() + 34.0, rect.center().y),
        Align2::LEFT_CENTER,
        link.label,
        look.medium_font(12.5),
        ink,
    );
    resp.on_hover_text(link.url)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn separator(ui: &mut Ui, look: &Look, width: f32) {
    let (r, _) = ui.allocate_exact_size(vec2(width.min(ui.available_width()), 1.0), Sense::hover());
    ui.painter().hline(
        r.x_range(),
        r.center().y,
        Stroke::new(1.0, color(look.palette.border_subtle)),
    );
}

fn page(
    ui: &mut Ui,
    look: &Look,
    app: &str,
    mark: &dyn Fn(&Painter, Rect),
    body: impl FnOnce(&mut Ui),
) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(vec2(28.0, 28.0), Sense::hover());
        mark(ui.painter(), rect);
        ui.add_space(10.0);
        let ink = color(look.palette.text_primary);
        let galley = ui
            .painter()
            .layout_no_wrap(app.to_owned(), look.strong_font(24.0), ink);
        let (rect, _) = ui.allocate_exact_size(galley.size(), Sense::hover());
        ui.painter().galley(rect.min, galley, ink);
    });
    ui.add_space(18.0);
    separator(ui, look, ui.available_width());
    ui.add_space(26.0);
    body(ui);
}

/// A section's heading on the page (Recent Files, the app's own sections).
pub fn section_heading(ui: &mut Ui, look: &Look, text: &str) -> Response {
    let ink = color(look.palette.text_primary);
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), look.strong_font(16.0), ink);
    let (rect, resp) = ui.allocate_exact_size(
        vec2(galley.size().x, layout::CONTROL_HEIGHT),
        Sense::hover(),
    );
    ui.painter().galley(
        pos2(rect.left(), rect.center().y - galley.size().y / 2.0),
        galley,
        ink,
    );
    resp.widget_info(|| WidgetInfo::labeled(Role::Label, true, text));
    resp
}

// --- Recent files -----------------------------------------------------------------------------

/// A recent file: its name, its folder, the full path (shown on hover) and a thumbnail when the
/// app has one (its texture and size in pixels).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Recent<'a> {
    pub name: &'a str,
    pub folder: &'a str,
    pub path: &'a str,
    pub thumb: Option<(TextureId, Vec2)>,
}

/// The words the recent files need, in the app's language.
#[derive(Clone, Debug, PartialEq)]
pub struct RecentLabels<'a> {
    /// The section's heading ("Recent Files").
    pub heading: &'a str,
    pub grid: &'a str,
    pub list: &'a str,
    /// The card's × and the context menu ("Remove from Recent").
    pub remove: &'a str,
    /// The context menu ("Clear Recent Files").
    pub clear: &'a str,
    /// A file whose folder is unknown ("On your computer").
    pub no_folder: &'a str,
    /// The drop card closing the grid, and the drop zone when there are no files: "Drop an image
    /// anywhere to open it." Its click is [`RecentPick::OpenDialog`].
    pub drop: &'a str,
    /// The drop card's icon (an NDL icon name).
    pub drop_icon: &'a str,
}

/// What was picked among the recent files.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecentPick {
    Open(usize),
    Remove(usize),
    Clear,
    /// The drop card or zone: show the Open dialog.
    OpenDialog,
}

/// Recent Files: the heading with the grid / list switch, then the files as cards (closed by a
/// drop card) or as rows. With no files, a drop zone in their place. `grid` is the app's saved
/// choice; the switch changes it.
pub fn recents(
    ui: &mut Ui,
    look: &Look,
    labels: &RecentLabels,
    list: &[Recent],
    grid: &mut bool,
) -> Option<RecentPick> {
    let width = ui.available_width();
    let mut picked = None;
    if list.is_empty() {
        section_heading(ui, look, labels.heading);
        ui.add_space(14.0);
        if drop_zone(ui, look, labels, width) {
            picked = Some(RecentPick::OpenDialog);
        }
        return picked;
    }
    ui.horizontal(|ui| {
        section_heading(ui, look, labels.heading);
        ui.add_space((ui.available_width() - 64.0).max(0.0));
        view_switch(ui, look, labels, grid);
    });
    ui.add_space(14.0);
    if *grid {
        let gap = measures::CARD_GAP;
        let cols = ((width + gap) / (measures::CARD.0 + gap)).floor().max(1.0) as usize;
        let count = list.len() + 1;
        for row in 0..count.div_ceil(cols) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for i in row * cols..((row + 1) * cols).min(count) {
                    match list.get(i) {
                        Some(file) => {
                            if let Some(pick) = card(ui, look, labels, file, i) {
                                picked = Some(pick);
                            }
                        }
                        None => {
                            if drop_card(ui, look, labels) {
                                picked = Some(RecentPick::OpenDialog);
                            }
                        }
                    }
                }
            });
            ui.add_space(gap);
        }
    } else {
        for (i, file) in list.iter().enumerate() {
            if let Some(pick) = row(ui, look, labels, file, i, width) {
                picked = Some(pick);
            }
        }
    }
    picked
}

/// The grid / list switch: two icon tabs in a field.
fn view_switch(ui: &mut Ui, look: &Look, labels: &RecentLabels, grid: &mut bool) {
    let p = &look.palette;
    let (frame, _) = ui.allocate_exact_size(vec2(64.0, layout::CONTROL_HEIGHT), Sense::hover());
    ui.painter().rect_filled(
        frame,
        radius(layout::CONTROL_RADIUS),
        color(p.control_field_bg),
    );
    for (i, (icon, label, on)) in [
        ("grid-view", labels.grid, true),
        ("list-view", labels.list, false),
    ]
    .into_iter()
    .enumerate()
    {
        let r = Rect::from_min_size(
            frame.min + vec2(2.0 + i as f32 * 30.0, 2.0),
            vec2(30.0, layout::CONTROL_HEIGHT - 4.0),
        );
        let resp = ui.interact(r, ui.scope_id().with(("ndl_home_view", i)), Sense::click());
        let selected = *grid == on;
        resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, selected, label));
        if selected {
            ui.painter().rect_filled(
                r,
                radius(layout::CONTROL_RADIUS - 2),
                color(p.surface_raised),
            );
            ui.painter().rect_stroke(
                r,
                radius(layout::CONTROL_RADIUS - 2),
                Stroke::new(1.0, color(p.border_subtle)),
                StrokeKind::Inside,
            );
        }
        let ink = if selected || resp.hovered() {
            color(p.text_primary)
        } else {
            color(p.text_secondary)
        };
        if let Some(icon) = icons::get(icon) {
            icons::paint(
                ui.painter(),
                Rect::from_center_size(r.center(), vec2(15.0, 15.0)),
                icon,
                icons::Style::regular(ink),
            );
        }
        if resp
            .on_hover_text(label)
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
        {
            *grid = on;
        }
    }
}

fn elided(ui: &Ui, text: &str, font: FontId, ink: Color32, width: f32) -> Arc<Galley> {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, ink);
    job.wrap = egui::text::TextWrapping::truncate_at_width(width.max(1.0));
    ui.painter().layout_job(job)
}

/// The thumbnail fitted inside `bounds`, or the file glyph when there is none.
fn preview(ui: &Ui, look: &Look, file: &Recent, bounds: Rect, glyph: f32) {
    match file.thumb {
        Some((id, size)) if size.x > 0.0 && size.y > 0.0 => {
            let scale = (bounds.width() / size.x).min(bounds.height() / size.y);
            ui.painter().image(
                id,
                Rect::from_center_size(bounds.center(), size * scale),
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        _ => {
            if let Some(icon) = icons::get("file") {
                icons::paint(
                    ui.painter(),
                    Rect::from_center_size(bounds.center(), vec2(glyph, glyph)),
                    icon,
                    icons::Style::regular(color(look.palette.text_tertiary)),
                );
            }
        }
    }
}

/// The round × that removes a file from the list, shown while the pointer is over it.
fn remove_button(
    ui: &mut Ui,
    look: &Look,
    labels: &RecentLabels,
    rect: Rect,
    id: egui::Id,
) -> bool {
    let p = &look.palette;
    let resp = ui.interact(rect, id, Sense::click());
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, labels.remove));
    let (fill, ink) = if resp.hovered() {
        (color(p.text_primary), color(p.surface_panel))
    } else {
        (color(p.surface_raised), color(p.text_secondary))
    };
    ui.painter()
        .circle_filled(rect.center(), rect.width() / 2.0, fill);
    if let Some(icon) = icons::get("close") {
        icons::paint(
            ui.painter(),
            rect.shrink(rect.width() * 0.25),
            icon,
            icons::Style::bold(ink),
        );
    }
    resp.on_hover_text(labels.remove).clicked()
}

fn context_menu(resp: &Response, labels: &RecentLabels, i: usize, picked: &mut Option<RecentPick>) {
    resp.context_menu(|ui| {
        if ui.button(labels.remove).clicked() {
            *picked = Some(RecentPick::Remove(i));
            ui.close();
        }
        if ui.button(labels.clear).clicked() {
            *picked = Some(RecentPick::Clear);
            ui.close();
        }
    });
}

fn card(
    ui: &mut Ui,
    look: &Look,
    labels: &RecentLabels,
    file: &Recent,
    i: usize,
) -> Option<RecentPick> {
    let p = &look.palette;
    let (rect, resp) =
        ui.allocate_exact_size(vec2(measures::CARD.0, measures::CARD.1), Sense::click());
    let hovered = ui.rect_contains_pointer(rect);
    let r = layout::CONTROL_RADIUS;
    ui.painter()
        .rect_filled(rect, radius(r), color(p.surface_raised));
    let well = Rect::from_min_size(rect.min, vec2(rect.width(), measures::PREVIEW));
    ui.painter().rect_filled(
        well,
        CornerRadius {
            nw: r,
            ne: r,
            sw: 0,
            se: 0,
        },
        color(p.surface_sunken),
    );
    preview(ui, look, file, well.shrink(10.0), 36.0);
    let edge = if hovered {
        Stroke::new(1.5, color(p.text_primary))
    } else {
        Stroke::new(1.0, color(p.border_subtle))
    };
    ui.painter()
        .rect_stroke(rect, radius(r), edge, StrokeKind::Inside);
    let x = rect.left() + 14.0;
    let room = rect.right() - x - 12.0;
    let ink = color(p.text_primary);
    let name = elided(ui, file.name, look.strong_font(13.0), ink, room);
    ui.painter()
        .galley(pos2(x, well.bottom() + 12.0), name, ink);
    let dim = color(p.text_secondary);
    let folder = if file.folder.is_empty() {
        labels.no_folder
    } else {
        file.folder
    };
    let folder = elided(ui, folder, FontId::proportional(11.5), dim, room);
    ui.painter()
        .galley(pos2(x, well.bottom() + 34.0), folder, dim);
    let mut picked = None;
    if hovered {
        let close = Rect::from_center_size(
            pos2(rect.right() - 18.0, rect.top() + 18.0),
            vec2(22.0, 22.0),
        );
        if remove_button(ui, look, labels, close, resp.id.with("remove")) {
            picked = Some(RecentPick::Remove(i));
        }
    }
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, file.name));
    let resp = resp
        .on_hover_text(file.path)
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    if resp.clicked() && picked.is_none() {
        picked = Some(RecentPick::Open(i));
    }
    context_menu(&resp, labels, i, &mut picked);
    picked
}

fn row(
    ui: &mut Ui,
    look: &Look,
    labels: &RecentLabels,
    file: &Recent,
    i: usize,
    width: f32,
) -> Option<RecentPick> {
    let p = &look.palette;
    let (rect, resp) = ui.allocate_exact_size(vec2(width, measures::ROW), Sense::click());
    let hovered = ui.rect_contains_pointer(rect);
    if hovered {
        ui.painter().rect_filled(
            rect,
            radius(layout::CONTROL_RADIUS),
            color(p.control_bg_hover),
        );
    }
    let thumb = Rect::from_center_size(pos2(rect.left() + 20.0, rect.center().y), vec2(26.0, 26.0));
    ui.painter()
        .rect_filled(thumb, radius(6), color(p.surface_sunken));
    preview(ui, look, file, thumb.shrink(2.0), 14.0);
    let close =
        Rect::from_center_size(pos2(rect.right() - 18.0, rect.center().y), vec2(22.0, 22.0));
    let right = if hovered {
        close.left() - 8.0
    } else {
        rect.right() - 12.0
    };
    let x = rect.left() + 42.0;
    let ink = color(p.text_primary);
    let name = elided(
        ui,
        file.name,
        look.strong_font(13.0),
        ink,
        (right - x).min(260.0),
    );
    let name_w = name.size().x;
    ui.painter()
        .galley(pos2(x, rect.center().y - name.size().y / 2.0), name, ink);
    let folder_room = right - (x + name_w + 16.0);
    if folder_room > 24.0 {
        let dim = color(p.text_secondary);
        let folder = if file.folder.is_empty() {
            labels.no_folder
        } else {
            file.folder
        };
        let folder = elided(ui, folder, FontId::proportional(12.0), dim, folder_room);
        ui.painter().galley(
            pos2(
                right - folder.size().x,
                rect.center().y - folder.size().y / 2.0,
            ),
            folder,
            dim,
        );
    }
    let mut picked = None;
    if hovered && remove_button(ui, look, labels, close, resp.id.with("remove")) {
        picked = Some(RecentPick::Remove(i));
    }
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, file.name));
    let resp = resp
        .on_hover_text(file.path)
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    if resp.clicked() && picked.is_none() {
        picked = Some(RecentPick::Open(i));
    }
    context_menu(&resp, labels, i, &mut picked);
    picked
}

/// The dashed outline that says files can be dropped, with the icon over the words.
fn drop_target(ui: &mut Ui, look: &Look, labels: &RecentLabels, rect: Rect, resp: &Response) {
    let p = &look.palette;
    let hovered = resp.hovered();
    if hovered {
        ui.painter().rect_filled(
            rect,
            radius(layout::CONTROL_RADIUS),
            color(p.control_bg_hover),
        );
    }
    let edge = if hovered {
        p.text_secondary
    } else {
        p.border_default
    };
    ui.painter().add(egui::Shape::dashed_line(
        &rounded_outline(rect.shrink(0.5), f32::from(layout::CONTROL_RADIUS)),
        Stroke::new(1.0, color(edge)),
        5.0,
        4.0,
    ));
    let ink = if hovered {
        color(p.text_primary)
    } else {
        color(p.text_secondary)
    };
    if let Some(icon) = icons::get(labels.drop_icon) {
        icons::paint(
            ui.painter(),
            Rect::from_center_size(rect.center() - vec2(0.0, 18.0), vec2(30.0, 30.0)),
            icon,
            icons::Style::regular(ink),
        );
    }
    let job = egui::text::LayoutJob::simple(
        labels.drop.to_owned(),
        look.medium_font(12.5),
        ink,
        rect.width() - 28.0,
    );
    let mut job = job;
    job.halign = egui::Align::Center;
    let galley = ui.painter().layout_job(job);
    ui.painter()
        .galley(pos2(rect.center().x, rect.center().y + 10.0), galley, ink);
}

/// The points of a rounded rectangle's outline, closed.
fn rounded_outline(rect: Rect, r: f32) -> Vec<egui::Pos2> {
    let r = r.min(rect.width() / 2.0).min(rect.height() / 2.0);
    let corners = [
        (pos2(rect.right() - r, rect.top() + r), -90.0_f32),
        (pos2(rect.right() - r, rect.bottom() - r), 0.0),
        (pos2(rect.left() + r, rect.bottom() - r), 90.0),
        (pos2(rect.left() + r, rect.top() + r), 180.0),
    ];
    let mut points = Vec::with_capacity(4 * 7 + 1);
    for (center, start) in corners {
        for step in 0..=6 {
            let a = (start + step as f32 * 15.0).to_radians();
            points.push(center + vec2(a.cos(), a.sin()) * r);
        }
    }
    if let Some(first) = points.first().copied() {
        points.push(first);
    }
    points
}

fn drop_card(ui: &mut Ui, look: &Look, labels: &RecentLabels) -> bool {
    let (rect, resp) =
        ui.allocate_exact_size(vec2(measures::CARD.0, measures::CARD.1), Sense::click());
    drop_target(ui, look, labels, rect, &resp);
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, labels.drop));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn drop_zone(ui: &mut Ui, look: &Look, labels: &RecentLabels, width: f32) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(width.min(540.0), 180.0), Sense::click());
    drop_target(ui, look, labels, rect, &resp);
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, true, labels.drop));
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}
