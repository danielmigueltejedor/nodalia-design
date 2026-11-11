//! Windows without a title bar of their own, a container that reacts to its size, and the layer
//! dialogs are shown in.
//!
//! [`ApplicationWindow`] and [`Window`] take their header bar inside the content (a [`HeaderBar`]
//! in a [`ToolbarView`]), the way NDL lays out a window. Their child is a [`BreakpointBin`] under
//! an overlay: the bin holds the content and the window's breakpoints, the overlay holds the
//! dialogs presented on the window.
//!
//! [`HeaderBar`]: crate::HeaderBar
//! [`ToolbarView`]: crate::ToolbarView

use std::cell::RefCell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::breakpoint::{Breakpoint, Breakpoints};

/// Re-evaluates the breakpoints of whichever container `widget` is.
pub(crate) fn evaluate_breakpoints(widget: &gtk::Widget) {
    if let Some(bin) = widget.downcast_ref::<BreakpointBin>() {
        bin.imp().breakpoints.evaluate();
    }
}

mod bin_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::BreakpointBin)]
    pub struct BreakpointBin {
        #[property(get, nullable)]
        pub child: RefCell<Option<gtk::Widget>>,
        pub(crate) breakpoints: Breakpoints,
    }

    impl BreakpointBin {
        pub(super) fn set_child(&self, child: Option<gtk::Widget>) {
            if let Some(old) = self.child.take() {
                old.unparent();
            }
            if let Some(child) = &child {
                child.set_parent(&*self.obj());
            }
            self.child.replace(child);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BreakpointBin {
        const NAME: &'static str = "NdlBreakpointBin";
        type Type = super::BreakpointBin;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("ndl-breakpoint-bin");
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for BreakpointBin {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().set_overflow(gtk::Overflow::Hidden);
        }

        fn dispose(&self) {
            if let Some(child) = self.child.take() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for BreakpointBin {
        // The minimum is left to the size request, so the bin can be made smaller than its
        // content needs: that is the point of a breakpoint. The content is then given what it
        // needs and clipped, until a breakpoint makes it fit.
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let natural = self
                .child
                .borrow()
                .as_ref()
                .map_or(0, |c| c.measure(orientation, for_size).1);
            (0, natural, -1, -1)
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            let obj = self.obj();
            self.breakpoints
                .size_changed(obj.upcast_ref(), width, height);
            if let Some(child) = self.child.borrow().as_ref() {
                let min_w = child.measure(gtk::Orientation::Horizontal, -1).0;
                let w = width.max(min_w);
                let min_h = child.measure(gtk::Orientation::Vertical, w).0;
                let h = height.max(min_h);
                child.allocate(w, h, baseline, None);
            }
        }

        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::ConstantSize
        }
    }
}

glib::wrapper! {
    /// A container that applies [`Breakpoint`]s according to the size it is given.
    pub struct BreakpointBin(ObjectSubclass<bin_imp::BreakpointBin>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for BreakpointBin {
    fn default() -> Self {
        Self::new()
    }
}

impl BreakpointBin {
    pub fn set_child(&self, child: Option<&impl IsA<gtk::Widget>>) {
        self.imp().set_child(child.map(|w| w.as_ref().clone()));
        self.notify_child();
    }

    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn add_breakpoint(&self, breakpoint: Breakpoint) {
        self.imp().breakpoints.add(breakpoint);
        self.queue_allocate();
    }
}

/// What both window types share: the bin, the dialog layer, and an empty title bar.
fn build_frame(window: &gtk::Window) -> (BreakpointBin, gtk::Overlay) {
    // An empty, hidden title bar keeps client-side decorations without GTK drawing a bar of
    // its own: the header bar lives in the content.
    let titlebar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    titlebar.set_visible(false);
    window.set_titlebar(Some(&titlebar));
    let bin = BreakpointBin::new();
    let layer = gtk::Overlay::new();
    layer.set_child(Some(&bin));
    window.set_child(Some(&layer));
    (bin, layer)
}

macro_rules! window_type {
    ($name:ident, $imp:ident, $parent:ty, $gname:literal, [$($extends:ty),*], $doc:literal) => {
        mod $imp {
            use super::*;

            #[derive(Default)]
            pub struct $name {
                pub bin: RefCell<Option<BreakpointBin>>,
                pub layer: RefCell<Option<gtk::Overlay>>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for $name {
                const NAME: &'static str = $gname;
                type Type = super::$name;
                type ParentType = $parent;
            }

            impl ObjectImpl for $name {
                fn constructed(&self) {
                    self.parent_constructed();
                    let obj = self.obj();
                    obj.add_css_class("ndl-window");
                    let (bin, layer) = build_frame(obj.upcast_ref());
                    self.bin.replace(Some(bin));
                    self.layer.replace(Some(layer));
                }
            }

            impl WidgetImpl for $name {}
            impl WindowImpl for $name {}
        }

        glib::wrapper! {
            #[doc = $doc]
            pub struct $name(ObjectSubclass<$imp::$name>)
                @extends $($extends),*, gtk::Widget,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager, gtk::gio::ActionGroup, gtk::gio::ActionMap;
        }

        impl $name {
            /// The window's content, under any header bar it carries.
            pub fn set_content(&self, content: Option<&impl IsA<gtk::Widget>>) {
                if let Some(bin) = self.imp().bin.borrow().as_ref() {
                    bin.set_child(content);
                }
            }

            pub fn content(&self) -> Option<gtk::Widget> {
                self.imp().bin.borrow().as_ref().and_then(BreakpointBin::child)
            }

            pub fn add_breakpoint(&self, breakpoint: Breakpoint) {
                if let Some(bin) = self.imp().bin.borrow().as_ref() {
                    bin.add_breakpoint(breakpoint);
                }
            }

            pub(crate) fn dialog_layer(&self) -> Option<gtk::Overlay> {
                self.imp().layer.borrow().clone()
            }

            /// The dialog on top, if one is showing over this window.
            pub fn visible_dialog(&self) -> Option<crate::Dialog> {
                let layer = self.dialog_layer()?;
                let mut child = layer.last_child();
                while let Some(c) = child {
                    if c.has_css_class("ndl-dialog-host") {
                        if let Some(d) = c.first_child().and_downcast::<crate::Dialog>() {
                            return Some(d);
                        }
                    }
                    child = c.prev_sibling();
                }
                None
            }
        }

        impl Default for $name {
            fn default() -> Self {
                glib::Object::new()
            }
        }
    };
}

window_type!(
    ApplicationWindow,
    app_window_imp,
    gtk::ApplicationWindow,
    "NdlApplicationWindow",
    [gtk::ApplicationWindow, gtk::Window],
    "An application window whose header bar is part of its content."
);

impl app_window_imp::ApplicationWindow {}
impl gtk::subclass::application_window::ApplicationWindowImpl
    for app_window_imp::ApplicationWindow
{
}

window_type!(
    Window,
    window_imp,
    gtk::Window,
    "NdlWindow",
    [gtk::Window],
    "A window whose header bar is part of its content."
);

impl ApplicationWindow {
    pub fn new(app: &impl IsA<gtk::Application>) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}

impl Window {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }
}

/// Builds a [`Window`].
#[derive(Default)]
#[must_use]
pub struct WindowBuilder {
    title: Option<String>,
    content: Option<gtk::Widget>,
    default_width: Option<i32>,
    default_height: Option<i32>,
    modal: Option<bool>,
    transient_for: Option<gtk::Window>,
}

impl WindowBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn content(mut self, content: &impl IsA<gtk::Widget>) -> Self {
        self.content = Some(content.as_ref().clone());
        self
    }
    pub fn default_width(mut self, width: i32) -> Self {
        self.default_width = Some(width);
        self
    }
    pub fn default_height(mut self, height: i32) -> Self {
        self.default_height = Some(height);
        self
    }
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }
    pub fn transient_for(mut self, parent: &impl IsA<gtk::Window>) -> Self {
        self.transient_for = Some(parent.as_ref().clone());
        self
    }
    pub fn build(self) -> Window {
        let window = Window::new();
        if let Some(t) = self.title {
            window.set_title(Some(&t));
        }
        if let Some(c) = self.content {
            window.set_content(Some(&c));
        }
        if let Some(w) = self.default_width {
            window.set_default_width(w);
        }
        if let Some(h) = self.default_height {
            window.set_default_height(h);
        }
        if let Some(m) = self.modal {
            window.set_modal(m);
        }
        if let Some(p) = self.transient_for {
            window.set_transient_for(Some(&p));
        }
        window
    }
}

/// The dialog layer of the window `widget` is in, if that window is one of ours.
pub(crate) fn dialog_layer_of(widget: &gtk::Widget) -> Option<gtk::Overlay> {
    let root = widget.root()?;
    if let Some(w) = root.downcast_ref::<ApplicationWindow>() {
        return w.dialog_layer();
    }
    if let Some(w) = root.downcast_ref::<Window>() {
        return w.dialog_layer();
    }
    None
}
