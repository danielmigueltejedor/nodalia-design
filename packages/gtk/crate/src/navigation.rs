//! Page navigation: a stack of pages, pushed and popped with a slide.

use std::cell::{Cell, RefCell};
use std::sync::OnceLock;

use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

/// The view the widget is a page of, if any.
pub(crate) fn enclosing_view(widget: &gtk::Widget) -> Option<NavigationView> {
    widget
        .ancestor(NavigationView::static_type())
        .and_downcast()
}

/// Whether a back button in this widget would have somewhere to go.
pub(crate) fn can_go_back(widget: &gtk::Widget) -> bool {
    let Some(page) = widget
        .ancestor(NavigationPage::static_type())
        .and_downcast::<NavigationPage>()
    else {
        return false;
    };
    if !page.can_pop() {
        return false;
    }
    page.parent()
        .and_then(|p| p.ancestor(NavigationView::static_type()))
        .and_downcast::<NavigationView>()
        .is_some_and(|view| view.position(&page).is_some_and(|i| i > 0))
}

mod page_imp {
    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::NavigationPage)]
    pub struct NavigationPage {
        #[property(get, nullable)]
        pub child: RefCell<Option<gtk::Widget>>,
        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set, nullable)]
        pub tag: RefCell<Option<String>>,
        #[property(get, set)]
        pub can_pop: Cell<bool>,
    }

    impl Default for NavigationPage {
        fn default() -> Self {
            Self {
                child: RefCell::default(),
                title: RefCell::default(),
                tag: RefCell::default(),
                can_pop: Cell::new(true),
            }
        }
    }

    impl NavigationPage {
        pub(super) fn set_child(&self, child: Option<gtk::Widget>) {
            if let Some(old) = self.child.take() {
                old.unparent();
            }
            if let Some(c) = &child {
                c.set_parent(&*self.obj());
            }
            self.child.replace(child);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NavigationPage {
        const NAME: &'static str = "NdlNavigationPage";
        type Type = super::NavigationPage;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.set_css_name("ndl-navigation-page");
            klass.set_accessible_role(gtk::AccessibleRole::Group);
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for NavigationPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("shown").build(),
                    Signal::builder("hidden").build(),
                ]
            })
        }

        fn dispose(&self) {
            if let Some(c) = self.child.take() {
                c.unparent();
            }
        }
    }

    impl WidgetImpl for NavigationPage {}
}

glib::wrapper! {
    /// One page of a [`NavigationView`].
    pub struct NavigationPage(ObjectSubclass<page_imp::NavigationPage>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for NavigationPage {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl NavigationPage {
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        self.imp().set_child(child.map(|w| w.as_ref().clone()));
        self.notify_child();
    }

    pub fn new(child: &impl IsA<gtk::Widget>, title: &str) -> Self {
        let page = Self::default();
        page.set_child(Some(child.as_ref()));
        page.set_title(title);
        page
    }

    pub fn with_tag(child: &impl IsA<gtk::Widget>, title: &str, tag: &str) -> Self {
        let page = Self::new(child, title);
        page.set_tag(Some(tag));
        page
    }

    pub fn builder() -> NavigationPageBuilder {
        NavigationPageBuilder::default()
    }

    pub fn connect_shown<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("shown", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }

    pub fn connect_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("hidden", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }
}

/// Builds a [`NavigationPage`].
#[derive(Default)]
#[must_use]
pub struct NavigationPageBuilder {
    child: Option<gtk::Widget>,
    title: Option<String>,
    tag: Option<String>,
}

impl NavigationPageBuilder {
    pub fn child(mut self, child: &impl IsA<gtk::Widget>) -> Self {
        self.child = Some(child.as_ref().clone());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    pub fn build(self) -> NavigationPage {
        let page = NavigationPage::default();
        if let Some(c) = self.child {
            page.set_child(Some(&c));
        }
        if let Some(t) = self.title {
            page.set_title(t);
        }
        page.set_tag(self.tag);
        page
    }
}

mod view_imp {
    use super::*;

    #[derive(Default)]
    pub struct NavigationView {
        pub stack: RefCell<Option<gtk::Stack>>,
        /// Pages added with `add`: they stay in the view when popped.
        pub pool: RefCell<Vec<super::NavigationPage>>,
        /// What is on the navigation stack, root first.
        pub pages: RefCell<Vec<super::NavigationPage>>,
        pub serial: Cell<u64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NavigationView {
        const NAME: &'static str = "NdlNavigationView";
        type Type = super::NavigationView;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.set_css_name("ndl-navigation-view");
        }
    }

    impl ObjectImpl for NavigationView {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("popped")
                        .param_types([super::NavigationPage::static_type()])
                        .build(),
                    Signal::builder("pushed").build(),
                ]
            })
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            let stack = gtk::Stack::new();
            stack.set_transition_duration(240);
            stack.set_parent(&*obj);
            self.stack.replace(Some(stack));

            // Back by keyboard and by the mouse's back button.
            let keys = gtk::ShortcutController::new();
            keys.set_scope(gtk::ShortcutScope::Local);
            for trigger in ["<Alt>Left", "Escape"] {
                if let Some(t) = gtk::ShortcutTrigger::parse_string(trigger) {
                    let action = gtk::CallbackAction::new(|w, _| {
                        let Some(view) = w.downcast_ref::<super::NavigationView>() else {
                            return glib::Propagation::Proceed;
                        };
                        if view.pop() {
                            glib::Propagation::Stop
                        } else {
                            glib::Propagation::Proceed
                        }
                    });
                    keys.add_shortcut(gtk::Shortcut::new(Some(t), Some(action)));
                }
            }
            obj.add_controller(keys);
            let click = gtk::GestureClick::new();
            click.set_button(8);
            let weak = obj.downgrade();
            click.connect_pressed(move |_, _, _, _| {
                if let Some(view) = weak.upgrade() {
                    view.pop();
                }
            });
            obj.add_controller(click);
        }

        fn dispose(&self) {
            if let Some(s) = self.stack.take() {
                s.unparent();
            }
        }
    }

    impl WidgetImpl for NavigationView {}
}

glib::wrapper! {
    /// Pages on a stack: push to go deeper, pop to come back.
    pub struct NavigationView(ObjectSubclass<view_imp::NavigationView>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for NavigationView {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationView {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn stack(&self) -> Option<gtk::Stack> {
        self.imp().stack.borrow().clone()
    }

    fn ensure_child(&self, page: &NavigationPage) {
        let Some(stack) = self.stack() else { return };
        if page.parent().as_ref() != Some(stack.upcast_ref()) {
            let n = self.imp().serial.get().wrapping_add(1);
            self.imp().serial.set(n);
            stack.add_named(page, Some(&format!("page-{n}")));
        }
    }

    fn show(&self, page: &NavigationPage, transition: gtk::StackTransitionType) {
        let Some(stack) = self.stack() else { return };
        let previous = stack.visible_child().and_downcast::<NavigationPage>();
        stack.set_transition_type(transition);
        stack.set_visible_child(page);
        if let Some(prev) = previous.filter(|p| p != page) {
            prev.emit_by_name::<()>("hidden", &[]);
        }
        page.emit_by_name::<()>("shown", &[]);
        // The page's header bar decides on its back button when mapped; a page that is shown
        // again after a pop is already mapped, so tell it.
        let mut child = page.first_child();
        while let Some(c) = child {
            sync_header_bars(&c, 6);
            child = c.next_sibling();
        }
    }

    /// Adds a page to the view without showing it. The first page added becomes the root.
    pub fn add(&self, page: &impl IsA<NavigationPage>) {
        let page = page.as_ref().clone();
        self.ensure_child(&page);
        self.imp().pool.borrow_mut().push(page.clone());
        if self.imp().pages.borrow().is_empty() {
            self.imp().pages.borrow_mut().push(page.clone());
            self.show(&page, gtk::StackTransitionType::None);
        }
    }

    pub fn remove(&self, page: &impl IsA<NavigationPage>) {
        let page = page.as_ref();
        self.imp().pool.borrow_mut().retain(|p| p != page);
        if !self.imp().pages.borrow().contains(page) {
            if let Some(stack) = self.stack() {
                if page.parent().as_ref() == Some(stack.upcast_ref()) {
                    stack.remove(page);
                }
            }
        }
    }

    pub fn push(&self, page: &impl IsA<NavigationPage>) {
        let page = page.as_ref().clone();
        if self.imp().pages.borrow().contains(&page) {
            return;
        }
        self.ensure_child(&page);
        let first = self.imp().pages.borrow().is_empty();
        self.imp().pages.borrow_mut().push(page.clone());
        self.show(
            &page,
            if first {
                gtk::StackTransitionType::None
            } else {
                gtk::StackTransitionType::SlideLeft
            },
        );
        self.emit_by_name::<()>("pushed", &[]);
    }

    pub fn push_by_tag(&self, tag: &str) {
        if let Some(page) = self.find_page(tag) {
            self.push(&page);
        }
    }

    /// Pops the visible page. False when only the root is left, or the page cannot be popped.
    pub fn pop(&self) -> bool {
        let popped = {
            let pages = self.imp().pages.borrow();
            if pages.len() < 2 || pages.last().is_some_and(|p| !p.can_pop()) {
                return false;
            }
            pages.last().cloned()
        };
        let Some(popped) = popped else { return false };
        self.pop_to_index(self.imp().pages.borrow().len() - 2);
        let _ = popped;
        true
    }

    pub fn pop_to_page(&self, page: &impl IsA<NavigationPage>) -> bool {
        let Some(i) = self.position(page.as_ref()) else {
            return false;
        };
        if i + 1 == self.imp().pages.borrow().len() {
            return false;
        }
        self.pop_to_index(i);
        true
    }

    pub fn pop_to_tag(&self, tag: &str) -> bool {
        let found = self
            .imp()
            .pages
            .borrow()
            .iter()
            .position(|p| p.tag().as_deref() == Some(tag));
        match found {
            Some(i) if i + 1 < self.imp().pages.borrow().len() => {
                self.pop_to_index(i);
                true
            }
            _ => false,
        }
    }

    fn pop_to_index(&self, index: usize) {
        let removed: Vec<NavigationPage> =
            self.imp().pages.borrow_mut().drain(index + 1..).collect();
        let Some(target) = self.imp().pages.borrow().get(index).cloned() else {
            return;
        };
        self.show(&target, gtk::StackTransitionType::SlideRight);
        let duration = self.stack().map_or(0, |s| s.transition_duration());
        for page in removed.into_iter().rev() {
            self.emit_by_name::<()>("popped", &[&page]);
            if self.imp().pool.borrow().contains(&page) {
                continue;
            }
            // Removed once the slide is over, so it stays visible while leaving.
            let weak = self.downgrade();
            glib::timeout_add_local_once(
                std::time::Duration::from_millis(u64::from(duration) + 20),
                move || {
                    let Some(view) = weak.upgrade() else { return };
                    if view.imp().pages.borrow().contains(&page) {
                        return;
                    }
                    if let Some(stack) = view.stack() {
                        if page.parent().as_ref() == Some(stack.upcast_ref()) {
                            stack.remove(&page);
                        }
                    }
                },
            );
        }
    }

    /// Replaces the whole stack; the last page is shown.
    pub fn replace(&self, pages: &[NavigationPage]) {
        let old: Vec<NavigationPage> = self.imp().pages.replace(Vec::new());
        for p in pages {
            self.ensure_child(p);
        }
        self.imp().pages.replace(pages.to_vec());
        if let Some(last) = pages.last() {
            self.show(last, gtk::StackTransitionType::None);
        }
        if let Some(stack) = self.stack() {
            for p in old {
                if !pages.contains(&p)
                    && !self.imp().pool.borrow().contains(&p)
                    && p.parent().as_ref() == Some(stack.upcast_ref())
                {
                    stack.remove(&p);
                }
            }
        }
    }

    pub fn replace_with_tags(&self, tags: &[&str]) {
        let pages: Vec<NavigationPage> = tags.iter().filter_map(|t| self.find_page(t)).collect();
        self.replace(&pages);
    }

    pub fn find_page(&self, tag: &str) -> Option<NavigationPage> {
        let imp = self.imp();
        imp.pool
            .borrow()
            .iter()
            .chain(imp.pages.borrow().iter())
            .find(|p| p.tag().as_deref() == Some(tag))
            .cloned()
    }

    pub fn visible_page(&self) -> Option<NavigationPage> {
        self.imp().pages.borrow().last().cloned()
    }

    pub fn previous_page(&self, page: &impl IsA<NavigationPage>) -> Option<NavigationPage> {
        let i = self.position(page.as_ref())?;
        i.checked_sub(1)
            .and_then(|j| self.imp().pages.borrow().get(j).cloned())
    }

    /// The navigation stack, root first.
    pub fn navigation_stack(&self) -> Vec<NavigationPage> {
        self.imp().pages.borrow().clone()
    }

    pub(crate) fn position(&self, page: &NavigationPage) -> Option<usize> {
        self.imp().pages.borrow().iter().position(|p| p == page)
    }

    pub fn connect_popped<F: Fn(&Self, &NavigationPage) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("popped", false, move |args| {
            let this = args.first().and_then(|v| v.get::<Self>().ok());
            let page = args.get(1).and_then(|v| v.get::<NavigationPage>().ok());
            if let (Some(this), Some(page)) = (this, page) {
                f(&this, &page);
            }
            None
        })
    }

    pub fn connect_pushed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("pushed", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }
}

/// Re-syncs the header bars under `widget`. Bars sit near the top of a page, so the walk stops
/// `depth` levels down rather than visiting every row of a long list.
fn sync_header_bars(widget: &gtk::Widget, depth: u32) {
    if let Some(bar) = widget.downcast_ref::<crate::HeaderBar>() {
        bar.sync_controls();
        return;
    }
    if depth == 0 {
        return;
    }
    let mut child = widget.first_child();
    while let Some(c) = child {
        sync_header_bars(&c, depth - 1);
        child = c.next_sibling();
    }
}
