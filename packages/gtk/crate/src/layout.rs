//! Two layouts that slide a pane over or beside the content: a sidebar ([`OverlaySplitView`])
//! and a sheet that rises from a bar at the bottom ([`BottomSheet`]).

use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::animation::{CallbackAnimationTarget, Easing, TimedAnimation};

const SLIDE_MS: u32 = 240;

fn reparent(slot: &RefCell<Option<gtk::Widget>>, holder: &gtk::Box, child: Option<gtk::Widget>) {
    if let Some(old) = slot.take() {
        holder.remove(&old);
    }
    if let Some(c) = &child {
        holder.append(c);
    }
    slot.replace(child);
}

fn holder(class: &str) -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
    b.add_css_class(class);
    b
}

fn scrim() -> gtk::Box {
    let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
    b.add_css_class("ndl-scrim");
    b.set_can_target(true);
    b
}

mod split_imp {
    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::OverlaySplitView)]
    pub struct OverlaySplitView {
        #[property(get = Self::show_sidebar_value, set = Self::set_show_sidebar)]
        pub show_sidebar: Cell<bool>,
        #[property(get, set = Self::set_collapsed)]
        pub collapsed: Cell<bool>,
        #[property(get, set = Self::set_min)]
        pub min_sidebar_width: Cell<f64>,
        #[property(get, set = Self::set_max)]
        pub max_sidebar_width: Cell<f64>,
        #[property(get, set = Self::set_fraction)]
        pub sidebar_width_fraction: Cell<f64>,
        pub sidebar: RefCell<Option<gtk::Widget>>,
        pub content: RefCell<Option<gtk::Widget>>,
        pub sidebar_box: gtk::Box,
        pub content_box: gtk::Box,
        pub scrim: gtk::Box,
        pub progress: Cell<f64>,
        pub anim: RefCell<Option<TimedAnimation>>,
    }

    impl Default for OverlaySplitView {
        fn default() -> Self {
            Self {
                show_sidebar: Cell::new(true),
                collapsed: Cell::new(false),
                min_sidebar_width: Cell::new(180.0),
                max_sidebar_width: Cell::new(280.0),
                sidebar_width_fraction: Cell::new(0.25),
                sidebar: RefCell::default(),
                content: RefCell::default(),
                sidebar_box: holder("ndl-sidebar-pane"),
                content_box: holder("ndl-content-pane"),
                scrim: scrim(),
                progress: Cell::new(1.0),
                anim: RefCell::default(),
            }
        }
    }

    impl OverlaySplitView {
        fn show_sidebar_value(&self) -> bool {
            self.show_sidebar.get()
        }

        fn set_show_sidebar(&self, show: bool) {
            if self.show_sidebar.replace(show) == show {
                return;
            }
            self.animate(if show { 1.0 } else { 0.0 });
        }

        fn set_collapsed(&self, collapsed: bool) {
            if self.collapsed.replace(collapsed) == collapsed {
                return;
            }
            let obj = self.obj();
            if collapsed {
                obj.add_css_class("collapsed");
            } else {
                obj.remove_css_class("collapsed");
            }
            obj.queue_allocate();
        }

        fn set_min(&self, v: f64) {
            self.min_sidebar_width
                .set(if v.is_finite() { v.max(0.0) } else { 0.0 });
            self.obj().queue_resize();
        }

        fn set_max(&self, v: f64) {
            self.max_sidebar_width
                .set(if v.is_finite() { v.max(0.0) } else { 0.0 });
            self.obj().queue_resize();
        }

        fn set_fraction(&self, v: f64) {
            self.sidebar_width_fraction.set(if v.is_finite() {
                v.clamp(0.0, 1.0)
            } else {
                0.25
            });
            self.obj().queue_resize();
        }

        fn animate(&self, to: f64) {
            let obj = self.obj();
            if let Some(anim) = self.anim.borrow().as_ref() {
                anim.set_value_from(self.progress.get());
                anim.set_value_to(to);
                anim.play();
                return;
            }
            let weak = obj.downgrade();
            let anim = TimedAnimation::new(
                &*obj,
                self.progress.get(),
                to,
                SLIDE_MS,
                CallbackAnimationTarget::new(move |v| {
                    if let Some(this) = weak.upgrade() {
                        this.imp().progress.set(v);
                        this.queue_allocate();
                    }
                }),
            );
            anim.set_easing(Easing::EaseOutCubic);
            anim.play();
            self.anim.replace(Some(anim));
        }

        pub fn sidebar_width(&self, width: i32) -> i32 {
            let (min, max) = (
                self.min_sidebar_width.get(),
                self.max_sidebar_width
                    .get()
                    .max(self.min_sidebar_width.get()),
            );
            let want = (f64::from(width) * self.sidebar_width_fraction.get()).clamp(min, max);
            let natural = self.sidebar_box.measure(gtk::Orientation::Horizontal, -1).0;
            (want as i32).max(natural).min(width.max(0))
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OverlaySplitView {
        const NAME: &'static str = "NdlOverlaySplitView";
        type Type = super::OverlaySplitView;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("ndl-overlay-split-view");
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for OverlaySplitView {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_overflow(gtk::Overflow::Hidden);
            // Sibling order is paint order: content, then the scrim, then the sidebar on top.
            self.content_box.set_parent(&*obj);
            self.scrim.set_parent(&*obj);
            self.sidebar_box.set_parent(&*obj);
            let click = gtk::GestureClick::new();
            let weak = obj.downgrade();
            click.connect_released(move |_, _, _, _| {
                if let Some(this) = weak.upgrade() {
                    this.set_show_sidebar(false);
                }
            });
            self.scrim.add_controller(click);
            let keys = gtk::EventControllerKey::new();
            let weak = obj.downgrade();
            keys.connect_key_pressed(move |_, key, _, _| {
                if let Some(this) = weak.upgrade() {
                    if key == gtk::gdk::Key::Escape && this.collapsed() && this.show_sidebar() {
                        this.set_show_sidebar(false);
                        return glib::Propagation::Stop;
                    }
                }
                glib::Propagation::Proceed
            });
            obj.add_controller(keys);
        }

        fn dispose(&self) {
            self.content_box.unparent();
            self.scrim.unparent();
            self.sidebar_box.unparent();
        }
    }

    impl WidgetImpl for OverlaySplitView {
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let (cmin, cnat, _, _) = self.content_box.measure(orientation, for_size);
            let (smin, snat, _, _) = self.sidebar_box.measure(orientation, -1);
            if orientation == gtk::Orientation::Vertical {
                return (cmin.max(smin), cnat.max(snat), -1, -1);
            }
            if self.collapsed.get() {
                (cmin.max(smin), cnat, -1, -1)
            } else {
                let side = self.min_sidebar_width.get() as i32;
                (cmin + smin.max(side), cnat + snat.max(side), -1, -1)
            }
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let p = self.progress.get().clamp(0.0, 1.0);
            let sw = self.sidebar_width(width);
            let shown = (f64::from(sw) * p) as i32;
            let offset = shown - sw;
            let visible = p > 0.0;
            self.sidebar_box.set_child_visible(visible);
            let place = |w: &gtk::Box, x: i32, wd: i32| {
                let t =
                    gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(x as f32, 0.0));
                w.allocate(wd.max(0), height, baseline, Some(t));
            };
            if self.collapsed.get() {
                place(&self.content_box, 0, width);
                self.scrim.set_child_visible(visible);
                self.scrim.set_opacity(p);
                place(&self.scrim, 0, width);
            } else {
                self.scrim.set_child_visible(false);
                let content_w = width - shown;
                let content_min = self.content_box.measure(gtk::Orientation::Horizontal, -1).0;
                place(&self.content_box, shown, content_w.max(content_min));
            }
            place(&self.sidebar_box, offset, sw);
        }
    }
}

glib::wrapper! {
    /// A sidebar beside the content, or over it with a scrim once `collapsed`.
    pub struct OverlaySplitView(ObjectSubclass<split_imp::OverlaySplitView>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for OverlaySplitView {
    fn default() -> Self {
        Self::new()
    }
}

impl OverlaySplitView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_sidebar(&self, sidebar: Option<&impl IsA<gtk::Widget>>) {
        let imp = self.imp();
        let sidebar = sidebar.map(|s| s.as_ref().clone());
        // The pane is as tall as the window; a child left at its natural height
        // collapses a ScrolledWindow inside it to nothing.
        if let Some(s) = &sidebar {
            s.set_vexpand(true);
        }
        reparent(&imp.sidebar, &imp.sidebar_box, sidebar);
    }

    pub fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
        let imp = self.imp();
        let content = content.map(|c| c.as_ref().clone());
        if let Some(c) = &content {
            c.set_hexpand(true);
            c.set_vexpand(true);
        }
        reparent(&imp.content, &imp.content_box, content);
    }

    pub fn shows_sidebar(&self) -> bool {
        self.show_sidebar()
    }

    pub fn is_collapsed(&self) -> bool {
        self.collapsed()
    }
}

mod sheet_imp {
    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::BottomSheet)]
    pub struct BottomSheet {
        #[property(get = Self::open_value, set = Self::set_open)]
        pub open: Cell<bool>,
        #[property(get = Self::reveal_value, set = Self::set_reveal)]
        pub reveal_bottom_bar: Cell<bool>,
        #[property(get)]
        pub bottom_bar_height: Cell<i32>,
        #[property(get, set)]
        pub modal: Cell<bool>,
        #[property(get, set)]
        pub full_width: Cell<bool>,
        #[property(get, set = Self::set_handle)]
        pub show_drag_handle: Cell<bool>,
        pub content: RefCell<Option<gtk::Widget>>,
        pub bar: RefCell<Option<gtk::Widget>>,
        pub sheet: RefCell<Option<gtk::Widget>>,
        pub content_box: gtk::Box,
        pub bar_box: gtk::Box,
        pub sheet_box: gtk::Box,
        pub handle: gtk::Box,
        pub scrim: gtk::Box,
        pub progress: Cell<f64>,
        pub bar_progress: Cell<f64>,
        pub anim: RefCell<Option<TimedAnimation>>,
        pub bar_anim: RefCell<Option<TimedAnimation>>,
        pub reported: Cell<i32>,
    }

    impl Default for BottomSheet {
        fn default() -> Self {
            let handle = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            handle.add_css_class("ndl-drag-handle");
            handle.set_halign(gtk::Align::Center);
            Self {
                open: Cell::new(false),
                reveal_bottom_bar: Cell::new(true),
                bottom_bar_height: Cell::new(0),
                modal: Cell::new(true),
                full_width: Cell::new(true),
                show_drag_handle: Cell::new(true),
                content: RefCell::default(),
                bar: RefCell::default(),
                sheet: RefCell::default(),
                content_box: holder("ndl-sheet-content"),
                bar_box: holder("ndl-bottom-bar"),
                sheet_box: holder("ndl-sheet"),
                handle,
                scrim: scrim(),
                progress: Cell::new(0.0),
                bar_progress: Cell::new(1.0),
                anim: RefCell::default(),
                bar_anim: RefCell::default(),
                reported: Cell::new(-1),
            }
        }
    }

    impl BottomSheet {
        fn open_value(&self) -> bool {
            self.open.get()
        }

        fn reveal_value(&self) -> bool {
            self.reveal_bottom_bar.get()
        }

        fn set_open(&self, open: bool) {
            if self.open.replace(open) == open {
                return;
            }
            let obj = self.obj();
            if open {
                obj.add_css_class("open");
            } else {
                obj.remove_css_class("open");
            }
            self.run(&self.anim, &self.progress, if open { 1.0 } else { 0.0 });
            if open {
                self.sheet_box.grab_focus();
            }
        }

        fn set_reveal(&self, reveal: bool) {
            if self.reveal_bottom_bar.replace(reveal) == reveal {
                return;
            }
            self.run(
                &self.bar_anim,
                &self.bar_progress,
                if reveal { 1.0 } else { 0.0 },
            );
        }

        fn set_handle(&self, show: bool) {
            self.show_drag_handle.set(show);
            self.handle.set_visible(show);
        }

        fn run(&self, slot: &RefCell<Option<TimedAnimation>>, value: &Cell<f64>, to: f64) {
            let obj = self.obj();
            if let Some(anim) = slot.borrow().as_ref() {
                anim.set_value_from(value.get());
                anim.set_value_to(to);
                anim.play();
                return;
            }
            let weak = obj.downgrade();
            let is_bar = std::ptr::eq(value, &self.bar_progress);
            let anim = TimedAnimation::new(
                &*obj,
                value.get(),
                to,
                SLIDE_MS,
                CallbackAnimationTarget::new(move |v| {
                    if let Some(this) = weak.upgrade() {
                        let imp = this.imp();
                        if is_bar {
                            imp.bar_progress.set(v);
                        } else {
                            imp.progress.set(v);
                        }
                        this.queue_allocate();
                    }
                }),
            );
            anim.set_easing(Easing::EaseOutCubic);
            anim.play();
            slot.replace(Some(anim));
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BottomSheet {
        const NAME: &'static str = "NdlBottomSheet";
        type Type = super::BottomSheet;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("ndl-bottom-sheet");
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BottomSheet {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_overflow(gtk::Overflow::Hidden);
            self.sheet_box.set_focusable(true);
            self.sheet_box.append(&self.handle);
            for w in [
                &self.content_box,
                &self.bar_box,
                &self.scrim,
                &self.sheet_box,
            ] {
                w.set_parent(&*obj);
            }
            let close = |w: &gtk::Box| {
                let click = gtk::GestureClick::new();
                let weak = w.downgrade();
                click.connect_released(move |_, _, _, _| {
                    if let Some(sheet) = weak
                        .upgrade()
                        .and_then(|w| w.parent())
                        .and_downcast::<super::BottomSheet>()
                    {
                        sheet.set_open(false);
                    }
                });
                w.add_controller(click);
            };
            close(&self.scrim);
            let handle_click = gtk::GestureClick::new();
            let weak = obj.downgrade();
            handle_click.connect_released(move |_, _, _, _| {
                if let Some(this) = weak.upgrade() {
                    this.set_open(false);
                }
            });
            self.handle.add_controller(handle_click);
            // A click on the bar that no button inside it took opens the sheet: a button claims
            // the sequence, which cancels this gesture before it can report a release.
            let open_click = gtk::GestureClick::new();
            let weak = obj.downgrade();
            open_click.connect_released(move |_, _, _, _| {
                if let Some(this) = weak.upgrade() {
                    this.set_open(true);
                }
            });
            self.bar_box.add_controller(open_click);
            let keys = gtk::EventControllerKey::new();
            let weak = obj.downgrade();
            keys.connect_key_pressed(move |_, key, _, _| {
                if let Some(this) = weak.upgrade() {
                    if key == gtk::gdk::Key::Escape && this.open() {
                        this.set_open(false);
                        return glib::Propagation::Stop;
                    }
                }
                glib::Propagation::Proceed
            });
            obj.add_controller(keys);
        }

        fn dispose(&self) {
            for w in [
                &self.content_box,
                &self.bar_box,
                &self.scrim,
                &self.sheet_box,
            ] {
                w.unparent();
            }
        }
    }

    impl WidgetImpl for BottomSheet {
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let (cmin, cnat, _, _) = self.content_box.measure(orientation, for_size);
            let (bmin, bnat, _, _) = self.bar_box.measure(orientation, for_size);
            if orientation == gtk::Orientation::Horizontal {
                (cmin.max(bmin), cnat.max(bnat), -1, -1)
            } else {
                (cmin, cnat, -1, -1)
            }
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let at = |w: &gtk::Box, y: i32, h: i32| {
                let t =
                    gtk::gsk::Transform::new().translate(&gtk::graphene::Point::new(0.0, y as f32));
                w.allocate(width, h.max(0), baseline, Some(t));
            };
            // The content keeps the whole area; the bar is drawn over its bottom edge, and
            // `bottom-bar-height` says how much it covers so the app can inset its scrollables.
            at(&self.content_box, 0, height);

            let r = self.bar_progress.get().clamp(0.0, 1.0);
            let bar_h = self.bar_box.measure(gtk::Orientation::Vertical, width).1;
            let shown_bar = (f64::from(bar_h) * r) as i32;
            self.bar_box.set_child_visible(r > 0.0);
            at(&self.bar_box, height - shown_bar, bar_h);

            let p = self.progress.get().clamp(0.0, 1.0);
            let (sheet_min, sheet_nat, _, _) =
                self.sheet_box.measure(gtk::Orientation::Vertical, width);
            let sheet_h = sheet_nat.max(sheet_min).min(height);
            let shown_sheet = (f64::from(sheet_h) * p) as i32;
            self.sheet_box.set_child_visible(p > 0.0);
            self.scrim.set_child_visible(p > 0.0 && self.modal.get());
            self.scrim.set_opacity(p);
            at(&self.scrim, 0, height);
            at(&self.sheet_box, height - shown_sheet, sheet_h);

            // Told after the allocation, not during it: whoever listens will change layout.
            if self.reported.replace(shown_bar) != shown_bar {
                let weak = self.obj().downgrade();
                glib::idle_add_local_once(move || {
                    if let Some(this) = weak.upgrade() {
                        let imp = this.imp();
                        let h = imp.reported.get();
                        if imp.bottom_bar_height.replace(h) != h {
                            this.notify_bottom_bar_height();
                        }
                    }
                });
            }
        }
    }
}

glib::wrapper! {
    /// A bar at the bottom that opens into a sheet over the content.
    pub struct BottomSheet(ObjectSubclass<sheet_imp::BottomSheet>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for BottomSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl BottomSheet {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
        let imp = self.imp();
        let content = content.map(|c| c.as_ref().clone());
        if let Some(c) = &content {
            c.set_vexpand(true);
        }
        reparent(&imp.content, &imp.content_box, content);
    }

    pub fn set_bottom_bar(&self, bar: Option<&impl IsA<gtk::Widget>>) {
        let imp = self.imp();
        reparent(&imp.bar, &imp.bar_box, bar.map(|b| b.as_ref().clone()));
    }

    pub fn set_sheet(&self, sheet: Option<&impl IsA<gtk::Widget>>) {
        let imp = self.imp();
        let sheet = sheet.map(|s| s.as_ref().clone());
        if let Some(s) = &sheet {
            s.set_vexpand(true);
        }
        reparent(&imp.sheet, &imp.sheet_box, sheet);
    }

    pub fn is_open(&self) -> bool {
        self.open()
    }
}
