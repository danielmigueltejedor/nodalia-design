//! Bars and the view that stacks them around content.

use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod toolbar_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::ToolbarView)]
    pub struct ToolbarView {
        #[property(get, nullable)]
        pub content: RefCell<Option<gtk::Widget>>,
        pub top: RefCell<Option<gtk::Box>>,
        pub bottom: RefCell<Option<gtk::Box>>,
        pub holder: RefCell<Option<gtk::Box>>,
    }

    impl ToolbarView {
        pub(super) fn set_content(&self, content: Option<gtk::Widget>) {
            let Some(holder) = self.holder.borrow().clone() else {
                return;
            };
            if let Some(old) = self.content.take() {
                holder.remove(&old);
            }
            if let Some(c) = &content {
                c.set_vexpand(true);
                holder.append(c);
            }
            self.content.replace(content);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ToolbarView {
        const NAME: &'static str = "NdlToolbarView";
        type Type = super::ToolbarView;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BoxLayout>();
            klass.set_css_name("ndl-toolbar-view");
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ToolbarView {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            if let Some(layout) = obj.layout_manager().and_downcast::<gtk::BoxLayout>() {
                layout.set_orientation(gtk::Orientation::Vertical);
            }
            let top = gtk::Box::new(gtk::Orientation::Vertical, 0);
            top.add_css_class("top-bar");
            let holder = gtk::Box::new(gtk::Orientation::Vertical, 0);
            holder.set_vexpand(true);
            let bottom = gtk::Box::new(gtk::Orientation::Vertical, 0);
            bottom.add_css_class("bottom-bar");
            top.set_parent(&*obj);
            holder.set_parent(&*obj);
            bottom.set_parent(&*obj);
            self.top.replace(Some(top));
            self.holder.replace(Some(holder));
            self.bottom.replace(Some(bottom));
        }

        fn dispose(&self) {
            for w in [
                self.top.take().map(|b| b.upcast::<gtk::Widget>()),
                self.holder.take().map(Cast::upcast),
                self.bottom.take().map(Cast::upcast),
            ]
            .into_iter()
            .flatten()
            {
                w.unparent();
            }
        }
    }

    impl WidgetImpl for ToolbarView {}
}

glib::wrapper! {
    /// Content with bars above and below it.
    pub struct ToolbarView(ObjectSubclass<toolbar_imp::ToolbarView>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ToolbarView {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolbarView {
    pub fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
        self.imp().set_content(content.map(|w| w.as_ref().clone()));
        self.notify_content();
    }

    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> ToolbarViewBuilder {
        ToolbarViewBuilder::default()
    }

    pub fn add_top_bar(&self, bar: &impl IsA<gtk::Widget>) {
        if let Some(top) = self.imp().top.borrow().as_ref() {
            top.append(bar);
        }
    }

    pub fn add_bottom_bar(&self, bar: &impl IsA<gtk::Widget>) {
        if let Some(bottom) = self.imp().bottom.borrow().as_ref() {
            bottom.append(bar);
        }
    }

    pub fn remove(&self, bar: &impl IsA<gtk::Widget>) {
        let imp = self.imp();
        for b in [imp.top.borrow().clone(), imp.bottom.borrow().clone()]
            .into_iter()
            .flatten()
        {
            if bar.as_ref().parent().as_ref() == Some(b.upcast_ref()) {
                b.remove(bar);
            }
        }
    }

    pub fn set_reveal_top_bars(&self, reveal: bool) {
        if let Some(top) = self.imp().top.borrow().as_ref() {
            top.set_visible(reveal);
        }
    }

    pub fn set_reveal_bottom_bars(&self, reveal: bool) {
        if let Some(bottom) = self.imp().bottom.borrow().as_ref() {
            bottom.set_visible(reveal);
        }
    }
}

/// Builds a [`ToolbarView`].
#[derive(Default)]
#[must_use]
pub struct ToolbarViewBuilder {
    content: Option<gtk::Widget>,
}

impl ToolbarViewBuilder {
    pub fn content(mut self, content: &impl IsA<gtk::Widget>) -> Self {
        self.content = Some(content.as_ref().clone());
        self
    }

    pub fn build(self) -> ToolbarView {
        let view = ToolbarView::new();
        view.set_content(self.content.as_ref());
        view
    }
}

mod header_imp {
    use super::*;

    #[derive(Default)]
    pub struct HeaderBar {
        pub bar: RefCell<Option<gtk::HeaderBar>>,
        pub start_controls: RefCell<Option<gtk::WindowControls>>,
        pub end_controls: RefCell<Option<gtk::WindowControls>>,
        pub back: RefCell<Option<gtk::Button>>,
        pub close: RefCell<Option<gtk::Button>>,
        pub show_start: Cell<bool>,
        pub show_end: Cell<bool>,
        pub show_back: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HeaderBar {
        const NAME: &'static str = "NdlHeaderBar";
        type Type = super::HeaderBar;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.set_css_name("ndl-header-bar");
            klass.set_accessible_role(gtk::AccessibleRole::Group);
        }
    }

    impl ObjectImpl for HeaderBar {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            self.show_start.set(true);
            self.show_end.set(true);
            self.show_back.set(true);
            let bar = gtk::HeaderBar::new();
            bar.set_show_title_buttons(false);
            let start = gtk::WindowControls::new(gtk::PackType::Start);
            let end = gtk::WindowControls::new(gtk::PackType::End);
            let back = gtk::Button::from_icon_name("go-previous-symbolic");
            back.add_css_class("back");
            back.set_tooltip_text(Some("Back"));
            back.set_visible(false);
            let close = gtk::Button::from_icon_name("window-close-symbolic");
            close.add_css_class("close");
            close.add_css_class("circular");
            close.set_tooltip_text(Some("Close"));
            close.set_visible(false);
            bar.pack_start(&start);
            bar.pack_start(&back);
            bar.pack_end(&end);
            bar.pack_end(&close);
            back.connect_clicked(|button| {
                if let Some(view) = crate::navigation::enclosing_view(button.upcast_ref()) {
                    view.pop();
                }
            });
            close.connect_clicked(|button| {
                if let Some(dialog) = crate::dialog::enclosing_dialog(button.upcast_ref()) {
                    dialog.close();
                }
            });
            bar.set_parent(&*obj);
            self.bar.replace(Some(bar));
            self.start_controls.replace(Some(start));
            self.end_controls.replace(Some(end));
            self.back.replace(Some(back));
            self.close.replace(Some(close));
        }

        fn dispose(&self) {
            if let Some(bar) = self.bar.take() {
                bar.unparent();
            }
        }
    }

    impl WidgetImpl for HeaderBar {
        fn map(&self) {
            self.obj().sync_controls();
            self.parent_map();
        }
    }
}

glib::wrapper! {
    /// A header bar that carries the window's controls, a back button inside a navigation page
    /// that is not the first, and a close button inside a dialog.
    pub struct HeaderBar(ObjectSubclass<header_imp::HeaderBar>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for HeaderBar {
    fn default() -> Self {
        Self::new()
    }
}

impl HeaderBar {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> HeaderBarBuilder {
        HeaderBarBuilder::default()
    }

    fn bar(&self) -> Option<gtk::HeaderBar> {
        self.imp().bar.borrow().clone()
    }

    pub fn pack_start(&self, child: &impl IsA<gtk::Widget>) {
        if let Some(bar) = self.bar() {
            bar.pack_start(child);
        }
    }

    pub fn pack_end(&self, child: &impl IsA<gtk::Widget>) {
        if let Some(bar) = self.bar() {
            bar.pack_end(child);
        }
    }

    pub fn remove(&self, child: &impl IsA<gtk::Widget>) {
        if let Some(bar) = self.bar() {
            bar.remove(child);
        }
    }

    pub fn set_title_widget(&self, widget: Option<&impl IsA<gtk::Widget>>) {
        if let Some(bar) = self.bar() {
            bar.set_title_widget(widget);
        }
    }

    pub fn title_widget(&self) -> Option<gtk::Widget> {
        self.bar().and_then(|b| b.title_widget())
    }

    pub fn set_show_start_title_buttons(&self, show: bool) {
        self.imp().show_start.set(show);
        self.sync_controls();
    }

    pub fn set_show_end_title_buttons(&self, show: bool) {
        self.imp().show_end.set(show);
        self.sync_controls();
    }

    pub fn set_show_back_button(&self, show: bool) {
        self.imp().show_back.set(show);
        self.sync_controls();
    }

    pub fn set_show_title(&self, show: bool) {
        if let Some(t) = self.title_widget() {
            t.set_visible(show);
        }
    }

    /// Which buttons show depends on where the bar is: a dialog gets a close button instead of
    /// the window's controls, a pushed navigation page gets a back button.
    pub(crate) fn sync_controls(&self) {
        let imp = self.imp();
        let in_dialog = crate::dialog::enclosing_dialog(self.upcast_ref());
        let can_go_back = crate::navigation::can_go_back(self.upcast_ref());
        if let Some(start) = imp.start_controls.borrow().as_ref() {
            start.set_visible(in_dialog.is_none() && imp.show_start.get());
        }
        if let Some(end) = imp.end_controls.borrow().as_ref() {
            end.set_visible(in_dialog.is_none() && imp.show_end.get());
        }
        if let Some(close) = imp.close.borrow().as_ref() {
            close.set_visible(
                in_dialog.as_ref().is_some_and(crate::Dialog::can_close) && imp.show_end.get(),
            );
        }
        if let Some(back) = imp.back.borrow().as_ref() {
            back.set_visible(can_go_back && imp.show_back.get());
        }
    }
}

/// Builds a [`HeaderBar`].
#[derive(Default)]
#[must_use]
pub struct HeaderBarBuilder {
    css_classes: Vec<String>,
    show_start: Option<bool>,
    show_end: Option<bool>,
    title_widget: Option<gtk::Widget>,
}

impl HeaderBarBuilder {
    pub fn css_classes(mut self, classes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.css_classes = classes.into_iter().map(Into::into).collect();
        self
    }
    pub fn show_start_title_buttons(mut self, show: bool) -> Self {
        self.show_start = Some(show);
        self
    }
    pub fn show_end_title_buttons(mut self, show: bool) -> Self {
        self.show_end = Some(show);
        self
    }
    pub fn title_widget(mut self, widget: &impl IsA<gtk::Widget>) -> Self {
        self.title_widget = Some(widget.as_ref().clone());
        self
    }
    pub fn build(self) -> HeaderBar {
        let bar = HeaderBar::new();
        for c in &self.css_classes {
            bar.add_css_class(c);
        }
        if let Some(s) = self.show_start {
            bar.set_show_start_title_buttons(s);
        }
        if let Some(s) = self.show_end {
            bar.set_show_end_title_buttons(s);
        }
        if let Some(t) = self.title_widget {
            bar.set_title_widget(Some(&t));
        }
        bar
    }
}

mod title_imp {
    use super::*;

    #[derive(Default)]
    pub struct WindowTitle {
        pub title: RefCell<Option<gtk::Label>>,
        pub subtitle: RefCell<Option<gtk::Label>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WindowTitle {
        const NAME: &'static str = "NdlWindowTitle";
        type Type = super::WindowTitle;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BoxLayout>();
            klass.set_css_name("ndl-window-title");
        }
    }

    impl ObjectImpl for WindowTitle {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            if let Some(layout) = obj.layout_manager().and_downcast::<gtk::BoxLayout>() {
                layout.set_orientation(gtk::Orientation::Vertical);
            }
            obj.set_valign(gtk::Align::Center);
            let title = gtk::Label::new(None);
            title.add_css_class("title");
            title.set_ellipsize(gtk::pango::EllipsizeMode::End);
            title.set_single_line_mode(true);
            let subtitle = gtk::Label::new(None);
            subtitle.add_css_class("subtitle");
            subtitle.set_ellipsize(gtk::pango::EllipsizeMode::End);
            subtitle.set_single_line_mode(true);
            subtitle.set_visible(false);
            title.set_parent(&*obj);
            subtitle.set_parent(&*obj);
            self.title.replace(Some(title));
            self.subtitle.replace(Some(subtitle));
        }

        fn dispose(&self) {
            for l in [self.title.take(), self.subtitle.take()]
                .into_iter()
                .flatten()
            {
                l.unparent();
            }
        }
    }

    impl WidgetImpl for WindowTitle {}
}

glib::wrapper! {
    /// A title and an optional subtitle, centred in a header bar.
    pub struct WindowTitle(ObjectSubclass<title_imp::WindowTitle>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for WindowTitle {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl WindowTitle {
    pub fn new(title: &str, subtitle: &str) -> Self {
        let this = Self::default();
        this.set_title(title);
        this.set_subtitle(subtitle);
        this
    }

    pub fn set_title(&self, title: &str) {
        if let Some(l) = self.imp().title.borrow().as_ref() {
            l.set_label(title);
        }
    }

    pub fn title(&self) -> glib::GString {
        self.imp()
            .title
            .borrow()
            .as_ref()
            .map(gtk::Label::label)
            .unwrap_or_default()
    }

    pub fn set_subtitle(&self, subtitle: &str) {
        if let Some(l) = self.imp().subtitle.borrow().as_ref() {
            l.set_label(subtitle);
            l.set_visible(!subtitle.is_empty());
        }
    }
}
