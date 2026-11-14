//! The pieces shared by the Nodalia engineering apps (Torno, Vortia, Forja), so the three read as
//! one suite: one typeface and type scale, one shell layout, and the panel parts every one of
//! them draws (islands, panel headers, property rows, tree rows, the ribbon).
//!
//! The specification is `docs/suites/engineering.md`. The module only draws: each app supplies
//! its labels, icons and values and acts on the responses by running its own commands.
//!
//! ```no_run
//! # fn draw(ctx: &egui::Context, ui: &mut egui::Ui) {
//! use nodalia_theme::engineering::{self as eng, Look};
//! use nodalia_theme::{Accent, Mode};
//!
//! eng::install_fonts(ctx); // once, at start-up
//! let look = Look::new(Mode::Dark.palette_with(Accent::Torno));
//! eng::island(&look).show(ui, |ui| {
//!     eng::panel_header(ui, &look, "Properties", |_| {});
//!     eng::properties(ui, "selection", |ui| {
//!         eng::property(ui, &look, "Length", |ui| eng::value(ui, &look, "12.500 mm"));
//!     });
//! });
//! # }
//! ```

use std::sync::Arc;

use egui::{
    pos2, vec2, Align2, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Frame, Margin,
    Rect, Response, RichText, Role, Sense, Stroke, Ui, WidgetInfo,
};

use crate::{for_egui::color, icons, Palette};

// --- Type -------------------------------------------------------------------------------------

/// IBM Plex Sans, variable in width and weight (SIL OFL 1.1, `fonts/ibm-plex-sans/OFL.txt`).
pub const PLEX_SANS: &[u8] = include_bytes!("../fonts/ibm-plex-sans/IBMPlexSans[wdth,wght].ttf");
/// IBM Plex Mono Regular (SIL OFL 1.1, `fonts/ibm-plex-mono/OFL.txt`).
pub const PLEX_MONO: &[u8] = include_bytes!("../fonts/ibm-plex-mono/IBMPlexMono-Regular.ttf");

/// The family of medium (500) text: tool labels, tabs, the selected tree row.
pub const MEDIUM: &str = "ndl-sans-medium";
/// The family of semibold (600) text: panel titles, dialog titles, group headings.
pub const SEMIBOLD: &str = "ndl-sans-semibold";
/// The family of bold (700) text: page titles on Home.
pub const BOLD: &str = "ndl-sans-bold";

/// The Plex Sans instances, as (font name, weight, family). The regular instance leads egui's
/// Proportional family; the others lead their named families.
const SANS: [(&str, f32, Option<&str>); 4] = [
    ("ndl-plex-sans-400", 400.0, None),
    ("ndl-plex-sans-500", 500.0, Some(MEDIUM)),
    ("ndl-plex-sans-600", 600.0, Some(SEMIBOLD)),
    ("ndl-plex-sans-700", 700.0, Some(BOLD)),
];
const MONO: &str = "ndl-plex-mono-400";

fn family(name: Option<&str>) -> FontFamily {
    name.map_or(FontFamily::Proportional, |n| FontFamily::Name(n.into()))
}

/// Puts the suite's faces first in `fonts`: Plex Sans at 400 in Proportional and at 500, 600 and
/// 700 in [`MEDIUM`], [`SEMIBOLD`] and [`BOLD`], and Plex Mono in Monospace. Whatever the
/// definitions already held (egui's faces, an app's script fallbacks) stays behind them, so a
/// glyph Plex lacks still draws. Call it on the definitions an app builds, then `set_fonts`.
pub fn add_fonts(fonts: &mut FontDefinitions) {
    let fallbacks = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    for (name, weight, fam) in SANS {
        let mut data = FontData::from_static(PLEX_SANS);
        data.tweak.coords = egui::epaint::text::VariationCoords::new([(b"wght", weight)]);
        fonts.font_data.insert(name.to_owned(), Arc::new(data));
        let list: Vec<String> = std::iter::once(name.to_owned())
            .chain(fallbacks.iter().filter(|f| *f != name).cloned())
            .collect();
        fonts.families.insert(family(fam), list);
    }
    fonts
        .font_data
        .insert(MONO.to_owned(), Arc::new(FontData::from_static(PLEX_MONO)));
    let mono = fonts.families.entry(FontFamily::Monospace).or_default();
    mono.retain(|f| f != MONO);
    mono.insert(0, MONO.to_owned());
}

/// egui's default definitions with the suite's faces in front ([`add_fonts`]).
pub fn font_definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    add_fonts(&mut fonts);
    fonts
}

/// Installs [`font_definitions`] and the suite's text styles ([`apply_text_styles`]). For an app
/// that adds no faces of its own; one that does calls [`add_fonts`] on its definitions instead.
pub fn install_fonts(ctx: &egui::Context) {
    ctx.set_fonts(font_definitions());
    apply_text_styles(ctx);
}

/// The type scale, in points. Every engineering app draws its text at these sizes.
pub mod size {
    /// Ribbon group names, status bar, hints under a field.
    pub const CAPTION: f32 = 11.0;
    /// Tree rows, property labels and values, menus, tool labels.
    pub const BODY: f32 = 12.5;
    /// Panel titles and the selected workspace tab.
    pub const PANEL_TITLE: f32 = 13.0;
    /// Dialog titles.
    pub const DIALOG_TITLE: f32 = 17.0;
    /// The title of a Home page or Preferences page.
    pub const PAGE_TITLE: f32 = 26.0;
    /// Numbers, coordinates, expressions, the command line and logs.
    pub const VALUE: f32 = 12.0;
}

/// Regular body text.
pub fn body() -> FontId {
    FontId::new(size::BODY, FontFamily::Proportional)
}
/// Medium body text: tool labels, tabs, a selected row.
pub fn label() -> FontId {
    FontId::new(size::BODY, family(Some(MEDIUM)))
}
/// Captions: ribbon group names, the status bar, hints.
pub fn caption() -> FontId {
    FontId::new(size::CAPTION, FontFamily::Proportional)
}
/// Semibold panel titles ("Browser", "Properties").
pub fn panel_title() -> FontId {
    FontId::new(size::PANEL_TITLE, family(Some(SEMIBOLD)))
}
/// Semibold dialog titles.
pub fn dialog_title() -> FontId {
    FontId::new(size::DIALOG_TITLE, family(Some(SEMIBOLD)))
}
/// Bold page titles.
pub fn page_title() -> FontId {
    FontId::new(size::PAGE_TITLE, family(Some(BOLD)))
}
/// Monospaced values: lengths, coordinates, residuals, the command line.
pub fn value_font() -> FontId {
    FontId::new(size::VALUE, FontFamily::Monospace)
}

/// Sets egui's text styles to the suite's scale: Body and Button [`body`], Small [`caption`],
/// Heading [`panel_title`], Monospace [`value_font`]. Call it after the NDL style is applied
/// (`for_egui::apply` sets sizes only); it changes every theme's style.
pub fn apply_text_styles(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        use egui::TextStyle;
        style.text_styles.insert(TextStyle::Body, body());
        style.text_styles.insert(TextStyle::Button, body());
        style.text_styles.insert(TextStyle::Small, caption());
        style.text_styles.insert(TextStyle::Heading, panel_title());
        style.text_styles.insert(TextStyle::Monospace, value_font());
    });
}

// --- Shell layout -----------------------------------------------------------------------------

/// The shell's metrics, in points (see the specification's layout diagram).
pub mod layout {
    /// The title bar: app mark, Home, File, quick access, document tabs, search, menu, controls.
    pub const TITLE_BAR: f32 = 40.0;
    /// The ribbon: workspace tabs over a row of tool groups.
    pub const RIBBON: f32 = 92.0;
    /// The workspace tab row at the top of the ribbon.
    pub const RIBBON_TABS: f32 = 28.0;
    /// A ribbon tool: 22 pt icon over a one-line label.
    pub const TOOL: egui::Vec2 = egui::Vec2::new(60.0, 48.0);
    /// The model tree island on the left.
    pub const BROWSER_WIDTH: f32 = 264.0;
    /// The properties island on the right.
    pub const PROPERTIES_WIDTH: f32 = 304.0;
    /// The status bar (coordinates, units, snaps) under everything.
    pub const STATUS_BAR: f32 = 30.0;
    /// A panel's title row.
    pub const PANEL_HEADER: f32 = 34.0;
    /// A model tree row.
    pub const TREE_ROW: f32 = 26.0;
    /// Indent of a tree level.
    pub const TREE_INDENT: f32 = 16.0;
    /// A property row.
    pub const PROPERTY_ROW: f32 = 28.0;
    /// The label column of a property grid.
    pub const PROPERTY_LABEL: f32 = 112.0;
    /// Gap between islands and from the window's edges.
    pub const GAP: f32 = 8.0;
    /// Radius of an island.
    pub const ISLAND_RADIUS: f32 = 12.0;
    /// Radius of a row, a tool or a tab highlight.
    pub const ROW_RADIUS: f32 = 7.0;
}

/// The colours the shared parts draw with.
#[derive(Clone, Debug, PartialEq)]
pub struct Look {
    pub palette: Palette,
}

impl Look {
    pub fn new(palette: Palette) -> Self {
        Look { palette }
    }
}

/// The frame of an island: the panel surface, rounded, padded, no stroke and no shadow.
pub fn island(look: &Look) -> Frame {
    Frame::NONE
        .fill(color(look.palette.surface_panel))
        .corner_radius(CornerRadius::same(layout::ISLAND_RADIUS as u8))
        .inner_margin(Margin::symmetric(8, 6))
}

/// A panel's title row: the semibold title on the left, `trailing` (a count, a filter, a menu)
/// laid out from the right.
pub fn panel_header(ui: &mut Ui, look: &Look, title: &str, trailing: impl FnOnce(&mut Ui)) {
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(width, layout::PANEL_HEADER),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(4.0);
            ui.label(
                RichText::new(title)
                    .font(panel_title())
                    .color(color(look.palette.text_primary)),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), trailing);
        },
    );
}

/// A grid for [`property`] rows, so their controls start at the same x.
pub fn properties<R>(ui: &mut Ui, id: &str, rows: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Grid::new(("ndl_eng_properties", id))
        .num_columns(2)
        .min_col_width(layout::PROPERTY_LABEL)
        .min_row_height(layout::PROPERTY_ROW)
        .spacing(vec2(10.0, 2.0))
        .show(ui, rows)
        .inner
}

/// A property row: the label right-aligned in the label column, then `control`.
pub fn property<R>(ui: &mut Ui, look: &Look, label: &str, control: impl FnOnce(&mut Ui) -> R) -> R {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(
            RichText::new(label)
                .font(body())
                .color(color(look.palette.text_secondary)),
        );
    });
    let out = control(ui);
    ui.end_row();
    out
}

/// A read-only value in the monospaced face (a length, a coordinate, a residual).
pub fn value(ui: &mut Ui, look: &Look, text: &str) -> Response {
    ui.label(
        RichText::new(text)
            .font(value_font())
            .color(color(look.palette.text_primary)),
    )
}

/// One model tree row. `depth` indents it; `expanded` draws a disclosure arrow (`None`: a leaf).
#[derive(Clone, Debug, PartialEq)]
pub struct TreeRow<'a> {
    pub depth: usize,
    pub expanded: Option<bool>,
    /// An NDL icon name; an unknown name draws no icon.
    pub icon: &'a str,
    pub label: &'a str,
    pub selected: bool,
    /// Dims the row (a hidden body, a suppressed feature).
    pub muted: bool,
}

/// What a click on a [`tree_row`] hit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeHit {
    None,
    Row,
    Disclosure,
}

/// Draws a model tree row the full width of `ui`. Selected rows take the accent's soft tint
/// with accent ink and the medium face.
pub fn tree_row(ui: &mut Ui, look: &Look, row: &TreeRow) -> (Response, TreeHit) {
    let p = &look.palette;
    let width = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(vec2(width, layout::TREE_ROW), Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::TreeItem, true, row.selected, row.label));
    let painter = ui.painter();
    let ink = if row.selected {
        painter.rect_filled(rect, layout::ROW_RADIUS, color(p.accent_subtle));
        color(p.accent_text)
    } else {
        if resp.hovered() {
            painter.rect_filled(rect, layout::ROW_RADIUS, color(p.control_bg_hover));
        }
        if row.muted {
            color(p.text_tertiary)
        } else {
            color(p.text_primary)
        }
    };
    let mut x = rect.left() + 4.0 + row.depth as f32 * layout::TREE_INDENT;
    let disclosure = Rect::from_min_size(pos2(x, rect.top()), vec2(16.0, rect.height()));
    if let Some(open) = row.expanded {
        let name = if open {
            "chevron-down"
        } else {
            "chevron-right"
        };
        if let Some(icon) = icons::get(name) {
            let glyph = Rect::from_center_size(disclosure.center(), vec2(12.0, 12.0));
            icons::paint(
                painter,
                glyph,
                icon,
                icons::Style::bold(color(p.text_secondary)),
            );
        }
    }
    x += 18.0;
    if let Some(icon) = icons::get(row.icon) {
        let glyph = Rect::from_min_size(pos2(x, rect.center().y - 8.0), vec2(16.0, 16.0));
        icons::paint(painter, glyph, icon, icons::Style::regular(ink));
        x += 22.0;
    }
    let font = if row.selected { label() } else { body() };
    painter.text(
        pos2(x, rect.center().y),
        Align2::LEFT_CENTER,
        row.label,
        font,
        ink,
    );
    let hit = if !resp.clicked() {
        TreeHit::None
    } else if row.expanded.is_some()
        && resp
            .interact_pointer_pos()
            .is_some_and(|pos| disclosure.contains(pos))
    {
        TreeHit::Disclosure
    } else {
        TreeHit::Row
    };
    (resp, hit)
}

/// A workspace tab of the ribbon: medium text, the accent's underline when selected, no box.
pub fn ribbon_tab(ui: &mut Ui, look: &Look, text: &str, selected: bool) -> Response {
    let p = &look.palette;
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), label(), color(p.text_primary));
    let size = vec2(galley.size().x + 20.0, layout::RIBBON_TABS);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, selected, text));
    let ink = if selected {
        color(p.accent_text)
    } else if resp.hovered() {
        color(p.text_primary)
    } else {
        color(p.text_secondary)
    };
    let painter = ui.painter();
    painter.text(rect.center(), Align2::CENTER_CENTER, text, label(), ink);
    if selected {
        let y = rect.bottom() - 2.0;
        painter.line_segment(
            [pos2(rect.left() + 8.0, y), pos2(rect.right() - 8.0, y)],
            Stroke::new(2.0, color(p.accent_default)),
        );
    }
    resp
}

/// A ribbon tool: a 22 pt CAD icon (accent parts tinted) over a one-line label.
pub fn ribbon_tool(ui: &mut Ui, look: &Look, icon: &str, text: &str, active: bool) -> Response {
    let p = &look.palette;
    let (rect, resp) = ui.allocate_exact_size(layout::TOOL, Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Button, true, active, text));
    let painter = ui.painter();
    if active {
        painter.rect_filled(rect, layout::ROW_RADIUS, color(p.accent_subtle));
    } else if resp.hovered() {
        painter.rect_filled(rect, layout::ROW_RADIUS, color(p.control_bg_hover));
    }
    if let Some(glyph) = icons::get(icon) {
        let r = Rect::from_center_size(pos2(rect.center().x, rect.top() + 16.0), vec2(22.0, 22.0));
        let style = icons::Style {
            accent: Some(color(p.accent_default)),
            ..icons::Style::regular(color(p.text_primary))
        };
        icons::paint(painter, r, glyph, style);
    }
    painter.text(
        pos2(rect.center().x, rect.bottom() - 9.0),
        Align2::CENTER_CENTER,
        text,
        FontId::new(size::CAPTION + 0.5, FontFamily::Proportional),
        color(p.text_primary),
    );
    resp
}

/// A ribbon group: its tools in a row with the group's name under them as a caption, and a thin
/// divider after it. Returns what `tools` returned.
pub fn ribbon_group<R>(
    ui: &mut Ui,
    look: &Look,
    name: &str,
    tools: impl FnOnce(&mut Ui) -> R,
) -> R {
    let p = &look.palette;
    let out = ui
        .vertical(|ui| {
            ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
            let out = ui.horizontal(tools).inner;
            ui.add_space(2.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(name)
                        .font(caption())
                        .color(color(p.text_secondary)),
                );
            });
            out
        })
        .inner;
    let (rect, _) = ui.allocate_exact_size(vec2(9.0, layout::TOOL.y + 14.0), Sense::hover());
    ui.painter().line_segment(
        [
            pos2(rect.center().x, rect.top() + 6.0),
            pos2(rect.center().x, rect.bottom() - 4.0),
        ],
        Stroke::new(1.0, color(p.border_subtle)),
    );
    out
}

// --- Title bar --------------------------------------------------------------------------------

/// An open document's tab in the [`title_bar`].
#[derive(Clone, Debug, PartialEq)]
pub struct Document<'a> {
    pub name: &'a str,
    /// Unsaved changes: a dot before the close button.
    pub modified: bool,
}

/// The accessible labels (and tooltips-free names) of the title bar's controls, in the app's
/// language.
#[derive(Clone, Debug, PartialEq)]
pub struct TitleLabels<'a> {
    pub home: &'a str,
    pub file: &'a str,
    pub save: &'a str,
    pub undo: &'a str,
    pub redo: &'a str,
    pub new_document: &'a str,
    /// Prefix of a tab's close button label, followed by the document's name ("Close Bracket").
    pub close_document: &'a str,
    /// The placeholder of the command search field ("Search commands").
    pub search: &'a str,
    pub menu: &'a str,
}

/// What the [`title_bar`] shows. The app's mark and name lead it; window controls are the app's
/// (`window_controls::show` on Linux, native elsewhere) in the space `controls_width` reserves.
#[derive(Clone, Debug, PartialEq)]
pub struct TitleBar<'a> {
    pub app: &'a str,
    pub documents: &'a [Document<'a>],
    pub active: Option<usize>,
    pub labels: TitleLabels<'a>,
    /// Width kept free at the right edge for window controls (0 for native controls).
    pub controls_width: f32,
    /// Width kept free at the left edge (macOS traffic lights; 0 elsewhere).
    pub leading_width: f32,
}

/// What a click in the [`title_bar`] asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TitleAction {
    Home,
    File,
    Save,
    Undo,
    Redo,
    Document(usize),
    CloseDocument(usize),
    NewDocument,
    Search,
    Menu,
}

/// What the [`title_bar`] drew.
pub struct TitleOutput {
    pub action: Option<TitleAction>,
    /// Where the app draws its window controls.
    pub controls: Rect,
    /// The empty part of the bar (what `background` returned): drag the window with it,
    /// double-click to maximize.
    pub drag: Response,
}

fn icon_button(ui: &mut Ui, look: &Look, icon: &str, label: &str, size: f32) -> Response {
    let p = &look.palette;
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, ui.is_enabled(), label));
    let painter = ui.painter();
    if resp.hovered() {
        painter.rect_filled(rect, layout::ROW_RADIUS, color(p.control_bg_hover));
    }
    if let Some(glyph) = icons::get(icon) {
        let r = Rect::from_center_size(rect.center(), vec2(18.0, 18.0));
        icons::paint(painter, r, glyph, icons::Style::bold(color(p.text_primary)));
    }
    resp
}

fn text_button(ui: &mut Ui, look: &Look, text: &str) -> Response {
    let p = &look.palette;
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), label(), color(p.text_primary));
    let (rect, resp) = ui.allocate_exact_size(vec2(galley.size().x + 20.0, 30.0), Sense::click());
    resp.widget_info(|| WidgetInfo::labeled(Role::Button, ui.is_enabled(), text));
    if resp.hovered() {
        ui.painter()
            .rect_filled(rect, layout::ROW_RADIUS, color(p.control_bg_hover));
    }
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        label(),
        color(p.text_primary),
    );
    resp
}

/// The widest a document tab gets; a longer name ends in "…".
const TAB_MAX: f32 = 240.0;

fn document_tab(
    ui: &mut Ui,
    look: &Look,
    doc: &Document,
    active: bool,
    close: &str,
) -> (Response, bool) {
    let p = &look.palette;
    let ink = if active {
        p.text_primary
    } else {
        p.text_secondary
    };
    // 12 of padding before the name, 30 for the close button after it.
    let mut job =
        egui::text::LayoutJob::simple_singleline(doc.name.to_owned(), label(), color(ink));
    job.wrap = egui::text::TextWrapping::truncate_at_width(TAB_MAX - 42.0);
    let galley = ui.ctx().fonts_mut(|f| f.layout_job(job));
    let width = (galley.size().x + 52.0).clamp(96.0, TAB_MAX);
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 30.0), Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Tab, true, active, doc.name));
    let painter = ui.painter();
    if active {
        painter.rect_filled(rect, layout::ROW_RADIUS, color(p.surface_panel));
    } else if resp.hovered() {
        painter.rect_filled(rect, layout::ROW_RADIUS, color(p.control_bg_hover));
    }
    let at = pos2(rect.left() + 12.0, rect.center().y - galley.size().y / 2.0);
    painter.galley(at, galley, color(ink));
    let x = Rect::from_center_size(pos2(rect.right() - 16.0, rect.center().y), vec2(20.0, 20.0));
    let close_resp = ui.interact(x, resp.id.with("close"), Sense::click());
    close_resp
        .widget_info(|| WidgetInfo::labeled(Role::Button, true, format!("{close} {}", doc.name)));
    if close_resp.hovered() {
        painter.rect_filled(x, 6.0, color(p.control_bg_pressed));
    }
    if doc.modified && !close_resp.hovered() {
        painter.circle_filled(x.center(), 3.5, color(p.text_secondary));
    } else if let Some(glyph) = icons::get("close") {
        let r = Rect::from_center_size(x.center(), vec2(12.0, 12.0));
        icons::paint(painter, r, glyph, icons::Style::bold(color(ink)));
    }
    (resp, close_resp.clicked())
}

/// The window's title bar ([`layout::TITLE_BAR`] high): app name, Home, File, quick access
/// (save, undo, redo), document tabs with a new-document button, then from the right the
/// window controls' space, the menu and the command search.
pub fn title_bar(ui: &mut Ui, look: &Look, bar: &TitleBar) -> TitleOutput {
    title_bar_with(ui, look, bar, |ui, rect| {
        ui.interact(
            rect,
            ui.scope_id().with("ndl_eng_title_drag"),
            Sense::click_and_drag(),
        )
    })
}

/// [`title_bar`] with the app's own handling of the empty area: `background` is called with
/// the bar's rect before any control is laid out, so the controls win the pointer over it.
/// Use it to plug in a platform's window-move logic (native drag on macOS, double-click
/// actions) instead of a plain drag response.
pub fn title_bar_with(
    ui: &mut Ui,
    look: &Look,
    bar: &TitleBar,
    background: impl FnOnce(&mut Ui, Rect) -> Response,
) -> TitleOutput {
    let p = &look.palette;
    let full = ui.available_rect_before_wrap();
    let rect = Rect::from_min_size(full.min, vec2(full.width(), layout::TITLE_BAR));
    let drag = background(ui, rect);
    let mut action = None;
    let controls = Rect::from_min_max(
        pos2(rect.right() - bar.controls_width, rect.top()),
        rect.right_bottom(),
    );
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(vec2(8.0, 0.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let ui = &mut child;
    ui.spacing_mut().item_spacing.x = 2.0;
    ui.add_space(bar.leading_width);
    ui.label(
        RichText::new(bar.app)
            .font(panel_title())
            .color(color(p.text_primary)),
    );
    ui.add_space(10.0);
    let l = &bar.labels;
    if icon_button(ui, look, "home", l.home, 30.0).clicked() {
        action = Some(TitleAction::Home);
    }
    if text_button(ui, look, l.file).clicked() {
        action = Some(TitleAction::File);
    }
    for (icon, name, a) in [
        ("save", l.save, TitleAction::Save),
        ("undo", l.undo, TitleAction::Undo),
        ("redo", l.redo, TitleAction::Redo),
    ] {
        if icon_button(ui, look, icon, name, 30.0).clicked() {
            action = Some(a);
        }
    }
    ui.add_space(8.0);
    for (i, doc) in bar.documents.iter().enumerate() {
        let (tab, closed) = document_tab(ui, look, doc, bar.active == Some(i), l.close_document);
        if closed {
            action = Some(TitleAction::CloseDocument(i));
        } else if tab.clicked() {
            action = Some(TitleAction::Document(i));
        }
    }
    if icon_button(ui, look, "plus", l.new_document, 28.0).clicked() {
        action = Some(TitleAction::NewDocument);
    }
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.add_space(bar.controls_width);
        if icon_button(ui, look, "menu", l.menu, 30.0).clicked() {
            action = Some(TitleAction::Menu);
        }
        ui.add_space(4.0);
        let (r, search) = ui.allocate_exact_size(vec2(200.0, 28.0), Sense::click());
        search.widget_info(|| WidgetInfo::labeled(Role::SearchInput, true, l.search));
        let painter = ui.painter();
        let fill = if search.hovered() {
            p.control_bg_hover
        } else {
            p.control_field_bg
        };
        painter.rect_filled(r, 14.0, color(fill));
        if let Some(glyph) = icons::get("search") {
            let g = Rect::from_center_size(pos2(r.left() + 16.0, r.center().y), vec2(14.0, 14.0));
            icons::paint(
                painter,
                g,
                glyph,
                icons::Style::bold(color(p.text_secondary)),
            );
        }
        painter.text(
            pos2(r.left() + 30.0, r.center().y),
            Align2::LEFT_CENTER,
            l.search,
            body(),
            color(p.text_tertiary),
        );
        if search.clicked() {
            action = Some(TitleAction::Search);
        }
    });
    TitleOutput {
        action,
        controls,
        drag,
    }
}

// --- Status bar and view bar ------------------------------------------------------------------

/// The status bar ([`layout::STATUS_BAR`] high, caption type): `left` holds what is the app's
/// (history, layout tabs, the run state), `right` is laid out from the right edge (coordinates
/// and units with [`status_value`], then [`status_toggle`]s).
pub fn status_bar(ui: &mut Ui, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(width, layout::STATUS_BAR),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.style_mut().override_font_id = Some(caption());
            ui.add_space(8.0);
            left(ui);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                right(ui);
            });
        },
    );
}

/// A value in the status bar: Plex Mono at caption size (coordinates, units, residuals).
pub fn status_value(ui: &mut Ui, look: &Look, text: &str) -> Response {
    ui.label(
        RichText::new(text)
            .font(FontId::new(size::CAPTION + 0.5, FontFamily::Monospace))
            .color(color(look.palette.text_primary)),
    )
}

/// A drafting aid in the status bar (grid, snap, ortho…): an icon, accent-tinted when on.
pub fn status_toggle(ui: &mut Ui, look: &Look, icon: &str, name: &str, on: bool) -> Response {
    let p = &look.palette;
    let (rect, resp) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::click());
    resp.widget_info(|| WidgetInfo::selected(Role::Button, true, on, name));
    let painter = ui.painter();
    if on {
        painter.rect_filled(rect, 6.0, color(p.accent_subtle));
    } else if resp.hovered() {
        painter.rect_filled(rect, 6.0, color(p.control_bg_hover));
    }
    let ink = if on { p.accent_text } else { p.text_secondary };
    if let Some(glyph) = icons::get(icon) {
        let r = Rect::from_center_size(rect.center(), vec2(16.0, 16.0));
        icons::paint(painter, r, glyph, icons::Style::regular(color(ink)));
    }
    resp
}

/// One item of the [`view_bar`].
#[derive(Clone, Debug, PartialEq)]
pub enum ViewItem<'a> {
    Tool {
        icon: &'a str,
        name: &'a str,
        active: bool,
    },
    Separator,
}

/// The floating view bar: a pill-shaped island of view tools (orbit, pan, zoom, fit, visual
/// style…) over the viewport. The app places it (an `egui::Area` anchored at the bottom centre
/// of the viewport). Returns the index of the item clicked this frame.
pub fn view_bar(ui: &mut Ui, look: &Look, items: &[ViewItem]) -> Option<usize> {
    let p = &look.palette;
    let mut clicked = None;
    Frame::NONE
        .fill(color(p.surface_raised))
        .stroke(Stroke::new(1.0, color(p.border_subtle)))
        .corner_radius(CornerRadius::same(20))
        .inner_margin(Margin::symmetric(6, 4))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                for (i, item) in items.iter().enumerate() {
                    match item {
                        ViewItem::Tool { icon, name, active } => {
                            let (rect, resp) =
                                ui.allocate_exact_size(vec2(30.0, 30.0), Sense::click());
                            resp.widget_info(|| {
                                WidgetInfo::selected(Role::Button, true, *active, *name)
                            });
                            let painter = ui.painter();
                            if *active {
                                painter.rect_filled(rect, 15.0, color(p.accent_subtle));
                            } else if resp.hovered() {
                                painter.rect_filled(rect, 15.0, color(p.control_bg_hover));
                            }
                            let ink = if *active {
                                p.accent_text
                            } else {
                                p.text_primary
                            };
                            if let Some(glyph) = icons::get(icon) {
                                let r = Rect::from_center_size(rect.center(), vec2(18.0, 18.0));
                                icons::paint(painter, r, glyph, icons::Style::regular(color(ink)));
                            }
                            if resp.clicked() {
                                clicked = Some(i);
                            }
                        }
                        ViewItem::Separator => {
                            let (rect, _) = ui.allocate_exact_size(vec2(9.0, 30.0), Sense::hover());
                            ui.painter().line_segment(
                                [
                                    pos2(rect.center().x, rect.top() + 7.0),
                                    pos2(rect.center().x, rect.bottom() - 7.0),
                                ],
                                Stroke::new(1.0, color(p.border_subtle)),
                            );
                        }
                    }
                }
            });
        });
    clicked
}
