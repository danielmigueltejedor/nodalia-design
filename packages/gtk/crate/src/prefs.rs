//! Preferences: groups of rows, pages of groups, and the dialog that holds the pages.

use std::cell::{OnceCell, RefCell};
use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::dialog::Dialog;
use crate::misc::Clamp;
use crate::toolbar::{HeaderBar, ToolbarView};

mod group_imp {
    use super::*;

    #[derive(Default)]
    pub struct PreferencesGroup {
        pub title: OnceCell<gtk::Label>,
        pub description: OnceCell<gtk::Label>,
        pub list: OnceCell<gtk::ListBox>,
        pub extra: OnceCell<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesGroup {
        const NAME: &'static str = "NdlPreferencesGroup";
        type Type = super::PreferencesGroup;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.set_css_name("ndl-preferences-group");
        }
    }

    impl ObjectImpl for PreferencesGroup {
        fn constructed(&self) {
            self.parent_constructed();
            let column = gtk::Box::new(gtk::Orientation::Vertical, 12);
            let header = gtk::Box::new(gtk::Orientation::Vertical, 4);
            let title = gtk::Label::builder()
                .xalign(0.0)
                .wrap(true)
                .visible(false)
                .css_classes(["heading"])
                .build();
            let description = gtk::Label::builder()
                .xalign(0.0)
                .wrap(true)
                .use_markup(true)
                .visible(false)
                .css_classes(["dim-label", "caption"])
                .build();
            header.append(&title);
            header.append(&description);
            let list = gtk::ListBox::builder()
                .selection_mode(gtk::SelectionMode::None)
                .visible(false)
                .css_classes(["boxed-list"])
                .build();
            let extra = gtk::Box::new(gtk::Orientation::Vertical, 12);
            extra.set_visible(false);
            column.append(&header);
            column.append(&list);
            column.append(&extra);
            column.set_parent(&*self.obj());
            let _ = self.title.set(title);
            let _ = self.description.set(description);
            let _ = self.list.set(list);
            let _ = self.extra.set(extra);
        }

        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for PreferencesGroup {}
}

glib::wrapper! {
    /// A titled block of rows, drawn as one card.
    pub struct PreferencesGroup(ObjectSubclass<group_imp::PreferencesGroup>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for PreferencesGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl PreferencesGroup {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> PreferencesGroupBuilder {
        PreferencesGroupBuilder::default()
    }

    pub fn set_title(&self, title: &str) {
        if let Some(l) = self.imp().title.get() {
            l.set_label(title);
            l.set_visible(!title.is_empty());
        }
    }

    pub fn set_description(&self, description: Option<&str>) {
        if let Some(l) = self.imp().description.get() {
            let text = description.unwrap_or_default();
            l.set_label(text);
            l.set_visible(!text.is_empty());
        }
    }

    /// Rows go into the card; anything else goes below it.
    pub fn add(&self, child: &impl IsA<gtk::Widget>) {
        let imp = self.imp();
        if child.as_ref().is::<gtk::ListBoxRow>() {
            if let Some(list) = imp.list.get() {
                list.append(child);
                list.set_visible(true);
            }
        } else if let Some(extra) = imp.extra.get() {
            extra.append(child);
            extra.set_visible(true);
        }
    }

    pub fn remove(&self, child: &impl IsA<gtk::Widget>) {
        let imp = self.imp();
        if let Some(list) = imp.list.get() {
            if child.as_ref().parent().as_ref() == Some(list.upcast_ref()) {
                list.remove(child);
                list.set_visible(list.first_child().is_some());
                return;
            }
        }
        if let Some(extra) = imp.extra.get() {
            if child.as_ref().parent().as_ref() == Some(extra.upcast_ref()) {
                extra.remove(child);
                extra.set_visible(extra.first_child().is_some());
            }
        }
    }
}

#[derive(Default)]
#[must_use]
pub struct PreferencesGroupBuilder {
    title: Option<String>,
    description: Option<String>,
}

impl PreferencesGroupBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> PreferencesGroup {
        let group = PreferencesGroup::new();
        if let Some(t) = &self.title {
            group.set_title(t);
        }
        group.set_description(self.description.as_deref());
        group
    }
}

mod page_imp {
    use super::*;

    #[derive(Default)]
    pub struct PreferencesPage {
        pub column: OnceCell<gtk::Box>,
        pub title: RefCell<String>,
        pub icon_name: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesPage {
        const NAME: &'static str = "NdlPreferencesPage";
        type Type = super::PreferencesPage;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.set_css_name("ndl-preferences-page");
        }
    }

    impl ObjectImpl for PreferencesPage {
        fn constructed(&self) {
            self.parent_constructed();
            let column = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(24)
                .margin_top(24)
                .margin_bottom(24)
                .margin_start(12)
                .margin_end(12)
                .build();
            let clamp = Clamp::builder().maximum_size(600).child(&column).build();
            let scroller = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .vexpand(true)
                .child(&clamp)
                .build();
            scroller.set_parent(&*self.obj());
            let _ = self.column.set(column);
        }

        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for PreferencesPage {}
}

glib::wrapper! {
    /// A scrolling column of [`PreferencesGroup`]s.
    pub struct PreferencesPage(ObjectSubclass<page_imp::PreferencesPage>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for PreferencesPage {
    fn default() -> Self {
        Self::new()
    }
}

impl PreferencesPage {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn add(&self, group: &PreferencesGroup) {
        if let Some(c) = self.imp().column.get() {
            c.append(group);
        }
    }

    pub fn remove(&self, group: &PreferencesGroup) {
        if let Some(c) = self.imp().column.get() {
            c.remove(group);
        }
    }

    pub fn set_title(&self, title: &str) {
        self.imp().title.replace(title.to_owned());
    }

    pub fn title(&self) -> String {
        self.imp().title.borrow().clone()
    }

    pub fn set_icon_name(&self, icon: Option<&str>) {
        self.imp().icon_name.replace(icon.map(str::to_owned));
    }
}

/// A dialog of preference pages; with more than one, a switcher in the header picks between them.
#[derive(Clone)]
pub struct PreferencesDialog(Rc<PrefsInner>);

struct PrefsInner {
    dialog: Dialog,
    stack: gtk::Stack,
    switcher: gtk::StackSwitcher,
    title: gtk::Label,
}

impl Default for PreferencesDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl PreferencesDialog {
    pub fn new() -> Self {
        let stack = gtk::Stack::builder()
            .transition_type(gtk::StackTransitionType::Crossfade)
            .vexpand(true)
            .build();
        let switcher = gtk::StackSwitcher::builder()
            .stack(&stack)
            .visible(false)
            .build();
        let title = gtk::Label::builder().css_classes(["title"]).build();
        let titles = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        titles.append(&title);
        titles.append(&switcher);
        let header = HeaderBar::new();
        header.set_title_widget(Some(&titles));
        let view = ToolbarView::new();
        view.add_top_bar(&header);
        view.set_content(Some(&stack));
        let dialog = Dialog::builder()
            .child(&view)
            .content_width(640)
            .content_height(640)
            .build();
        dialog.add_css_class("preferences");
        Self(Rc::new(PrefsInner {
            dialog,
            stack,
            switcher,
            title,
        }))
    }

    pub fn set_title(&self, title: &str) {
        self.0.title.set_label(title);
        self.0.dialog.set_title(title);
    }

    pub fn add(&self, page: &PreferencesPage) {
        let inner = &self.0;
        let stack_page = inner.stack.add_child(page);
        stack_page.set_title(&page.title());
        let many = inner.stack.pages().n_items() > 1;
        inner.switcher.set_visible(many);
        inner.title.set_visible(!many);
    }

    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.0.dialog.present(parent);
    }

    pub fn close(&self) -> bool {
        self.0.dialog.close()
    }

    pub fn dialog(&self) -> &Dialog {
        &self.0.dialog
    }
}
