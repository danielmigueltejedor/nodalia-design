//! Dialogs. Presented on one of this crate's windows, a dialog floats over the window's content
//! behind a scrim, the way NDL draws a modal; anywhere else it gets a small window of its own.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::OnceLock;

use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

/// The dialog a widget is inside, if any.
pub(crate) fn enclosing_dialog(widget: &gtk::Widget) -> Option<Dialog> {
    widget.ancestor(Dialog::static_type()).and_downcast()
}

pub(crate) enum Host {
    /// On a window's dialog layer: the scrim box that holds the dialog, and the content it covers.
    Layer { layer: gtk::Overlay, host: gtk::Box },
    /// In a window of its own.
    Window(gtk::Window),
}

mod imp {
    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::Dialog)]
    pub struct Dialog {
        #[property(get, nullable)]
        pub child: RefCell<Option<gtk::Widget>>,
        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set = Self::set_content_width)]
        pub content_width: Cell<i32>,
        #[property(get, set = Self::set_content_height)]
        pub content_height: Cell<i32>,
        #[property(get, set)]
        pub can_close: Cell<bool>,
        pub(crate) host: RefCell<Option<Host>>,
    }

    impl Default for Dialog {
        fn default() -> Self {
            Self {
                child: RefCell::default(),
                title: RefCell::default(),
                content_width: Cell::new(-1),
                content_height: Cell::new(-1),
                can_close: Cell::new(true),
                host: RefCell::default(),
            }
        }
    }

    impl Dialog {
        pub(super) fn set_child(&self, child: Option<gtk::Widget>) {
            if let Some(old) = self.child.take() {
                old.unparent();
            }
            if let Some(c) = &child {
                c.set_parent(&*self.obj());
            }
            self.child.replace(child);
        }

        fn set_content_width(&self, w: i32) {
            self.content_width.set(w);
            self.obj().queue_resize();
        }

        fn set_content_height(&self, h: i32) {
            self.content_height.set(h);
            self.obj().queue_resize();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Dialog {
        const NAME: &'static str = "NdlDialog";
        type Type = super::Dialog;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            // No layout manager: GTK would then skip `measure` below, and with it the
            // content size, leaving every dialog at its child's minimum.
            klass.set_css_name("ndl-dialog");
            klass.set_accessible_role(gtk::AccessibleRole::Dialog);
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for Dialog {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("closed").build(),
                    Signal::builder("close-attempt").build(),
                ]
            })
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_overflow(gtk::Overflow::Hidden);
            obj.set_halign(gtk::Align::Center);
            obj.set_valign(gtk::Align::Center);
            let keys = gtk::EventControllerKey::new();
            let weak = obj.downgrade();
            keys.connect_key_pressed(move |_, key, _, _| {
                if key == gtk::gdk::Key::Escape {
                    if let Some(d) = weak.upgrade() {
                        d.close();
                        return glib::Propagation::Stop;
                    }
                }
                glib::Propagation::Proceed
            });
            obj.add_controller(keys);
        }

        fn dispose(&self) {
            if let Some(c) = self.child.take() {
                c.unparent();
            }
        }
    }

    impl WidgetImpl for Dialog {
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let (min, nat) = self.child.borrow().as_ref().map_or((0, 0), |c| {
                let m = c.measure(orientation, for_size);
                (m.0, m.1)
            });
            let wanted = if orientation == gtk::Orientation::Horizontal {
                self.content_width.get()
            } else {
                self.content_height.get()
            };
            (min, if wanted > 0 { wanted.max(min) } else { nat }, -1, -1)
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            if let Some(child) = self.child.borrow().as_ref() {
                child.allocate(width, height, baseline, None);
            }
        }
    }
}

glib::wrapper! {
    /// A modal surface over a window.
    pub struct Dialog(ObjectSubclass<imp::Dialog>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new()
    }
}

impl Dialog {
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        self.imp().set_child(child.map(|w| w.as_ref().clone()));
        self.notify_child();
    }

    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> DialogBuilder {
        DialogBuilder::default()
    }

    /// Shows the dialog over `parent`'s window.
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        if self.imp().host.borrow().is_some() {
            return;
        }
        let parent = parent.map(|p| p.as_ref().clone());
        if let Some(layer) = parent.as_ref().and_then(crate::window::dialog_layer_of) {
            let host = gtk::Box::new(gtk::Orientation::Vertical, 0);
            host.add_css_class("ndl-dialog-host");
            host.set_hexpand(true);
            host.set_vexpand(true);
            host.append(self);
            let click = gtk::GestureClick::new();
            let weak = self.downgrade();
            click.connect_released(move |gesture, _, x, y| {
                let Some(dialog) = weak.upgrade() else { return };
                let Some(host) = gesture.widget() else { return };
                // Only a click on the scrim itself, outside the dialog.
                if host.pick(x, y, gtk::PickFlags::DEFAULT).as_ref() == Some(&host) {
                    dialog.close();
                }
            });
            host.add_controller(click);
            if let Some(content) = layer.child() {
                content.set_can_target(false);
                content.set_can_focus(false);
            }
            layer.add_overlay(&host);
            self.imp().host.replace(Some(Host::Layer { layer, host }));
            self.child_focus(gtk::DirectionType::TabForward);
        } else {
            let window = gtk::Window::new();
            window.set_modal(true);
            window.set_title(Some(&self.title()));
            if let Some(root) = parent
                .as_ref()
                .and_then(|p| p.root())
                .and_downcast::<gtk::Window>()
            {
                window.set_transient_for(Some(&root));
            }
            let w = self.content_width();
            let h = self.content_height();
            window.set_default_size(w, h);
            window.set_child(Some(self));
            let weak = self.downgrade();
            window.connect_close_request(move |_| match weak.upgrade() {
                Some(dialog) if !dialog.close() => glib::Propagation::Stop,
                _ => glib::Propagation::Proceed,
            });
            window.present();
            self.imp().host.replace(Some(Host::Window(window)));
        }
    }

    /// Closes the dialog if it can be closed; otherwise reports the attempt. True if closed.
    pub fn close(&self) -> bool {
        if !self.can_close() {
            self.emit_by_name::<()>("close-attempt", &[]);
            return false;
        }
        self.force_close();
        true
    }

    /// Closes the dialog whatever `can-close` says.
    pub fn force_close(&self) {
        let Some(host) = self.imp().host.take() else {
            return;
        };
        match host {
            Host::Layer { layer, host } => {
                host.remove(self);
                layer.remove_overlay(&host);
                if let Some(content) = layer.child() {
                    content.set_can_target(true);
                    content.set_can_focus(true);
                }
            }
            Host::Window(window) => {
                window.set_child(None::<&gtk::Widget>);
                window.destroy();
            }
        }
        self.emit_by_name::<()>("closed", &[]);
    }

    pub fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("closed", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }

    pub fn connect_close_attempt<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("close-attempt", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }
}

/// Builds a [`Dialog`].
#[derive(Default)]
#[must_use]
pub struct DialogBuilder {
    title: Option<String>,
    child: Option<gtk::Widget>,
    content_width: Option<i32>,
    content_height: Option<i32>,
    can_close: Option<bool>,
}

impl DialogBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn child(mut self, child: &impl IsA<gtk::Widget>) -> Self {
        self.child = Some(child.as_ref().clone());
        self
    }
    pub fn content_width(mut self, w: i32) -> Self {
        self.content_width = Some(w);
        self
    }
    pub fn content_height(mut self, h: i32) -> Self {
        self.content_height = Some(h);
        self
    }
    pub fn can_close(mut self, can: bool) -> Self {
        self.can_close = Some(can);
        self
    }
    pub fn build(self) -> Dialog {
        let d = Dialog::new();
        if let Some(t) = self.title {
            d.set_title(t);
        }
        if let Some(c) = self.child {
            d.set_child(Some(&c));
        }
        if let Some(w) = self.content_width {
            d.set_content_width(w);
        }
        if let Some(h) = self.content_height {
            d.set_content_height(h);
        }
        if let Some(c) = self.can_close {
            d.set_can_close(c);
        }
        d
    }
}

/// How a response button looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResponseAppearance {
    #[default]
    Default,
    Suggested,
    Destructive,
}

type ResponseHandler = Box<dyn Fn(&AlertDialog, &str)>;

struct AlertInner {
    dialog: Dialog,
    heading: gtk::Label,
    body: gtk::Label,
    extra: gtk::Box,
    buttons: gtk::Box,
    responses: RefCell<Vec<(String, gtk::Button)>>,
    default_response: RefCell<Option<String>>,
    close_response: RefCell<String>,
    handlers: RefCell<Vec<(Option<String>, ResponseHandler)>>,
}

/// A short question with a row of answers.
#[derive(Clone)]
pub struct AlertDialog(Rc<AlertInner>);

impl AlertDialog {
    pub fn new(heading: Option<&str>, body: Option<&str>) -> Self {
        let dialog = Dialog::new();
        dialog.add_css_class("ndl-alert");
        dialog.set_content_width(360);
        let column = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let text = gtk::Box::new(gtk::Orientation::Vertical, 8);
        text.add_css_class("ndl-alert-text");
        let heading_label = gtk::Label::new(heading);
        heading_label.add_css_class("title-3");
        heading_label.set_wrap(true);
        heading_label.set_justify(gtk::Justification::Center);
        heading_label.set_visible(heading.is_some());
        let body_label = gtk::Label::new(body);
        body_label.set_wrap(true);
        body_label.set_justify(gtk::Justification::Center);
        body_label.add_css_class("body");
        body_label.set_visible(body.is_some());
        let extra = gtk::Box::new(gtk::Orientation::Vertical, 0);
        extra.set_visible(false);
        text.append(&heading_label);
        text.append(&body_label);
        text.append(&extra);
        let buttons = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        buttons.add_css_class("ndl-alert-buttons");
        buttons.set_homogeneous(true);
        column.append(&text);
        column.append(&buttons);
        dialog.set_child(Some(&column));
        let this = Self(Rc::new(AlertInner {
            dialog,
            heading: heading_label,
            body: body_label,
            extra,
            buttons,
            responses: RefCell::default(),
            default_response: RefCell::default(),
            close_response: RefCell::new("close".into()),
            handlers: RefCell::default(),
        }));
        let weak = Rc::downgrade(&this.0);
        this.0.dialog.connect_close_attempt(move |_| {
            if let Some(inner) = weak.upgrade() {
                let me = AlertDialog(inner);
                let id = me.0.close_response.borrow().clone();
                me.respond(&id);
            }
        });
        // Escape and the scrim go through `close`, which answers with the close response.
        this.0.dialog.set_can_close(false);
        this
    }

    pub fn set_heading(&self, heading: Option<&str>) {
        self.0.heading.set_label(heading.unwrap_or_default());
        self.0.heading.set_visible(heading.is_some());
    }

    pub fn set_body(&self, body: &str) {
        self.0.body.set_label(body);
        self.0.body.set_visible(!body.is_empty());
    }

    pub fn set_extra_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        while let Some(c) = self.0.extra.first_child() {
            self.0.extra.remove(&c);
        }
        if let Some(c) = child {
            self.0.extra.append(c);
        }
        self.0.extra.set_visible(child.is_some());
    }

    pub fn add_response(&self, id: &str, label: &str) {
        let button = gtk::Button::with_mnemonic(label);
        let weak = Rc::downgrade(&self.0);
        let owned = id.to_owned();
        button.connect_clicked(move |_| {
            if let Some(inner) = weak.upgrade() {
                AlertDialog(inner).respond(&owned);
            }
        });
        self.0.buttons.append(&button);
        self.0.responses.borrow_mut().push((id.to_owned(), button));
    }

    pub fn set_response_appearance(&self, id: &str, appearance: ResponseAppearance) {
        if let Some((_, b)) = self.0.responses.borrow().iter().find(|(r, _)| r == id) {
            b.remove_css_class("suggested-action");
            b.remove_css_class("destructive-action");
            match appearance {
                ResponseAppearance::Suggested => b.add_css_class("suggested-action"),
                ResponseAppearance::Destructive => b.add_css_class("destructive-action"),
                ResponseAppearance::Default => {}
            }
        }
    }

    pub fn set_response_enabled(&self, id: &str, enabled: bool) {
        if let Some((_, b)) = self.0.responses.borrow().iter().find(|(r, _)| r == id) {
            b.set_sensitive(enabled);
        }
    }

    pub fn set_default_response(&self, id: Option<&str>) {
        self.0.default_response.replace(id.map(str::to_owned));
    }

    pub fn set_close_response(&self, id: &str) {
        self.0.close_response.replace(id.to_owned());
    }

    /// Calls `f` with the response id; with `Some(detail)`, only for that response.
    pub fn connect_response<F: Fn(&Self, &str) + 'static>(&self, detail: Option<&str>, f: F) {
        self.0
            .handlers
            .borrow_mut()
            .push((detail.map(str::to_owned), Box::new(f)));
    }

    fn respond(&self, id: &str) {
        self.0.dialog.force_close();
        for (detail, f) in self.0.handlers.borrow().iter() {
            if detail.as_deref().is_none_or(|d| d == id) {
                f(self, id);
            }
        }
    }

    /// Triggers the default response, as Enter in a field would.
    pub fn activate_default(&self) {
        if let Some(id) = self.0.default_response.borrow().clone() {
            let enabled = self
                .0
                .responses
                .borrow()
                .iter()
                .any(|(r, b)| *r == id && b.is_sensitive());
            if enabled {
                self.respond(&id);
            }
        }
    }

    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.0.dialog.present(parent);
        if let Some(id) = self.0.default_response.borrow().as_ref() {
            if let Some((_, b)) = self.0.responses.borrow().iter().find(|(r, _)| r == id) {
                b.grab_focus();
            }
        }
    }

    /// The dialog under the alert, for anything not wrapped here.
    pub fn dialog(&self) -> &Dialog {
        &self.0.dialog
    }
}
