//! List rows: a title with an optional subtitle, things before it and things after it.
//!
//! Every row is its own `GtkListBoxRow` subclass sharing one layout ([`Parts`]) rather than a
//! subclass of [`ActionRow`]: subclassing a Rust-defined GObject type needs an `unsafe impl`, and
//! the rows have too little behaviour in common to be worth it.

use std::cell::{Cell, OnceCell, RefCell};
use std::rc::Rc;
use std::sync::OnceLock;

use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

/// The layout every row shares: `[prefixes] title/subtitle [suffixes]`.
#[derive(Debug)]
pub(crate) struct Parts {
    #[allow(dead_code)]
    pub header: gtk::Box,
    pub prefixes: gtk::Box,
    pub titles: gtk::Box,
    pub title: gtk::Label,
    pub subtitle: gtk::Label,
    pub suffixes: gtk::Box,
}

impl Parts {
    pub(crate) fn new(row: &gtk::ListBoxRow) -> Self {
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        header.add_css_class("header");
        let prefixes = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        prefixes.add_css_class("prefixes");
        prefixes.set_visible(false);
        let titles = gtk::Box::new(gtk::Orientation::Vertical, 2);
        titles.add_css_class("title-box");
        titles.set_hexpand(true);
        titles.set_valign(gtk::Align::Center);
        let title = gtk::Label::builder()
            .xalign(0.0)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .use_markup(true)
            .css_classes(["title"])
            .build();
        let subtitle = gtk::Label::builder()
            .xalign(0.0)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .use_markup(true)
            .visible(false)
            .css_classes(["subtitle"])
            .build();
        titles.append(&title);
        titles.append(&subtitle);
        let suffixes = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        suffixes.add_css_class("suffixes");
        suffixes.set_visible(false);
        header.append(&prefixes);
        header.append(&titles);
        header.append(&suffixes);
        row.set_child(Some(&header));
        row.add_css_class("ndl-row");
        Self {
            header,
            prefixes,
            titles,
            title,
            subtitle,
            suffixes,
        }
    }

    pub(crate) fn set_title(&self, title: &str) {
        self.title.set_label(title);
        self.title.set_visible(!title.is_empty());
    }

    pub(crate) fn set_subtitle(&self, subtitle: &str) {
        self.subtitle.set_label(subtitle);
        self.subtitle.set_visible(!subtitle.is_empty());
    }

    pub(crate) fn add_prefix(&self, widget: &impl IsA<gtk::Widget>) {
        self.prefixes.append(widget);
        self.prefixes.set_visible(true);
    }

    pub(crate) fn add_suffix(&self, widget: &impl IsA<gtk::Widget>) {
        self.suffixes.append(widget);
        self.suffixes.set_visible(true);
    }

    pub(crate) fn remove(&self, widget: &impl IsA<gtk::Widget>) {
        for holder in [&self.prefixes, &self.suffixes] {
            if widget.as_ref().parent().as_ref() == Some(holder.upcast_ref()) {
                holder.remove(widget);
                holder.set_visible(holder.first_child().is_some());
            }
        }
    }
}

/// Calls `f` whenever the list the row sits in activates it. `GtkListBox` reports activation on
/// the list, not the row, so the row listens to whichever list it is in now.
pub(crate) fn on_row_activated(row: &gtk::ListBoxRow, f: impl Fn() + 'static) {
    let f = Rc::new(f);
    let current: RefCell<Option<(glib::WeakRef<gtk::ListBox>, glib::SignalHandlerId)>> =
        RefCell::default();
    row.connect_parent_notify(move |row| {
        if let Some((list, id)) = current.take() {
            if let Some(list) = list.upgrade() {
                list.disconnect(id);
            }
        }
        if let Some(list) = row.parent().and_downcast::<gtk::ListBox>() {
            let me = row.downgrade();
            let f = f.clone();
            let id = list.connect_row_activated(move |_, activated| {
                if me.upgrade().as_ref() == Some(activated) {
                    f();
                }
            });
            current.replace(Some((list.downgrade(), id)));
        }
    });
}

/// The `activated` signal, as a static of the calling class. A `Signal` registers once, so a
/// single static shared by every row type made the second type to load panic.
macro_rules! activated_signal {
    () => {{
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| vec![Signal::builder("activated").build()])
    }};
}

/// The methods every row has, written once.
macro_rules! row_common {
    ($ty:ty) => {
        impl $ty {
            fn parts(&self) -> &Parts {
                self.imp()
                    .parts
                    .get_or_init(|| Parts::new(self.upcast_ref()))
            }

            pub fn set_title(&self, title: &str) {
                self.parts().set_title(title);
            }

            pub fn title(&self) -> glib::GString {
                self.parts().title.label()
            }

            pub fn set_subtitle(&self, subtitle: &str) {
                self.parts().set_subtitle(subtitle);
            }

            pub fn subtitle(&self) -> glib::GString {
                self.parts().subtitle.label()
            }

            pub fn add_prefix(&self, widget: &impl IsA<gtk::Widget>) {
                self.parts().add_prefix(widget);
            }

            pub fn add_suffix(&self, widget: &impl IsA<gtk::Widget>) {
                self.parts().add_suffix(widget);
            }

            pub fn remove(&self, widget: &impl IsA<gtk::Widget>) {
                self.parts().remove(widget);
            }

            pub fn connect_activated<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
                self.connect_local("activated", false, move |args| {
                    if let Some(row) = args.first().and_then(|v| v.get::<Self>().ok()) {
                        f(&row);
                    }
                    None
                })
            }
        }
    };
}

/// Builder fields every row takes.
macro_rules! row_builder {
    ($builder:ident, $ty:ty, { $($field:ident : $fty:ty),* $(,)? }, |$row:ident, $b:ident| $extra:block) => {
        #[derive(Default)]
        #[must_use]
        pub struct $builder {
            title: Option<String>,
            subtitle: Option<String>,
            sensitive: Option<bool>,
            css_classes: Vec<String>,
            $($field: Option<$fty>,)*
        }

        impl $builder {
            pub fn title(mut self, title: impl Into<String>) -> Self {
                self.title = Some(title.into());
                self
            }

            pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
                self.subtitle = Some(subtitle.into());
                self
            }

            pub fn sensitive(mut self, sensitive: bool) -> Self {
                self.sensitive = Some(sensitive);
                self
            }

            pub fn css_classes(mut self, classes: impl IntoIterator<Item = impl Into<String>>) -> Self {
                self.css_classes = classes.into_iter().map(Into::into).collect();
                self
            }

            $(
                pub fn $field(mut self, value: impl Into<$fty>) -> Self {
                    self.$field = Some(value.into());
                    self
                }
            )*

            pub fn build(self) -> $ty {
                let $row = <$ty>::new();
                if let Some(t) = &self.title {
                    $row.set_title(t);
                }
                if let Some(s) = &self.subtitle {
                    $row.set_subtitle(s);
                }
                if let Some(s) = self.sensitive {
                    $row.set_sensitive(s);
                }
                for class in &self.css_classes {
                    $row.add_css_class(class);
                }
                let $b = self;
                $extra
                $row
            }
        }
    };
}

// ---------------------------------------------------------------------------------------------
// ActionRow

mod action_imp {
    use super::*;

    #[derive(Default)]
    pub struct ActionRow {
        pub(crate) parts: OnceCell<Parts>,
        pub activatable_widget: RefCell<Option<glib::WeakRef<gtk::Widget>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ActionRow {
        const NAME: &'static str = "NdlActionRow";
        type Type = super::ActionRow;
        type ParentType = gtk::ListBoxRow;
    }

    impl ObjectImpl for ActionRow {
        fn signals() -> &'static [Signal] {
            activated_signal!()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_activatable(false);
            obj.add_css_class("ndl-action-row");
            let weak = obj.downgrade();
            on_row_activated(obj.upcast_ref(), move || {
                if let Some(row) = weak.upgrade() {
                    row.activate_row();
                }
            });
        }
    }

    impl WidgetImpl for ActionRow {}
    impl ListBoxRowImpl for ActionRow {}
}

glib::wrapper! {
    /// A row with a title, a subtitle, and widgets before and after them.
    pub struct ActionRow(ObjectSubclass<action_imp::ActionRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

row_common!(ActionRow);
row_builder!(ActionRowBuilder, ActionRow, { activatable: bool }, |row, b| {
    if let Some(a) = b.activatable {
        row.set_activatable(a);
    }
});

impl Default for ActionRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> ActionRowBuilder {
        ActionRowBuilder::default()
    }

    /// The widget activating the row activates, the way a label activates its mnemonic target.
    pub fn set_activatable_widget(&self, widget: Option<&impl IsA<gtk::Widget>>) {
        self.imp()
            .activatable_widget
            .replace(widget.map(|w| w.as_ref().downgrade()));
        if widget.is_some() {
            self.set_activatable(true);
        }
    }

    fn activate_row(&self) {
        let target = self
            .imp()
            .activatable_widget
            .borrow()
            .as_ref()
            .and_then(|w| w.upgrade());
        if let Some(target) = target {
            target.mnemonic_activate(false);
        }
        self.emit_by_name::<()>("activated", &[]);
    }
}

// ---------------------------------------------------------------------------------------------
// SwitchRow

mod switch_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::SwitchRow)]
    pub struct SwitchRow {
        pub(crate) parts: OnceCell<Parts>,
        #[property(get, set)]
        pub active: Cell<bool>,
        pub switch: OnceCell<gtk::Switch>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwitchRow {
        const NAME: &'static str = "NdlSwitchRow";
        type Type = super::SwitchRow;
        type ParentType = gtk::ListBoxRow;
    }

    #[glib::derived_properties]
    impl ObjectImpl for SwitchRow {
        fn signals() -> &'static [Signal] {
            activated_signal!()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.add_css_class("ndl-switch-row");
            let switch = gtk::Switch::builder()
                .valign(gtk::Align::Center)
                .can_focus(false)
                .build();
            obj.bind_property("active", &switch, "active")
                .bidirectional()
                .sync_create()
                .build();
            obj.add_suffix(&switch);
            let _ = self.switch.set(switch);
            let weak = obj.downgrade();
            on_row_activated(obj.upcast_ref(), move || {
                if let Some(row) = weak.upgrade() {
                    row.set_active(!row.active());
                    row.emit_by_name::<()>("activated", &[]);
                }
            });
        }
    }

    impl WidgetImpl for SwitchRow {}
    impl ListBoxRowImpl for SwitchRow {}
}

glib::wrapper! {
    /// A row holding an on/off switch. Activating the row flips it.
    pub struct SwitchRow(ObjectSubclass<switch_imp::SwitchRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

row_common!(SwitchRow);
row_builder!(SwitchRowBuilder, SwitchRow, { active: bool }, |row, b| {
    if let Some(a) = b.active {
        row.set_active(a);
    }
});

impl Default for SwitchRow {
    fn default() -> Self {
        Self::new()
    }
}

impl SwitchRow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> SwitchRowBuilder {
        SwitchRowBuilder::default()
    }

    pub fn is_active(&self) -> bool {
        self.active()
    }
}

// ---------------------------------------------------------------------------------------------
// ComboRow

mod combo_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::ComboRow)]
    pub struct ComboRow {
        pub(crate) parts: OnceCell<Parts>,
        #[property(get, set)]
        pub selected: Cell<u32>,
        pub dropdown: OnceCell<gtk::DropDown>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ComboRow {
        const NAME: &'static str = "NdlComboRow";
        type Type = super::ComboRow;
        type ParentType = gtk::ListBoxRow;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ComboRow {
        fn signals() -> &'static [Signal] {
            activated_signal!()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.add_css_class("ndl-combo-row");
            let dropdown = gtk::DropDown::builder().valign(gtk::Align::Center).build();
            dropdown.add_css_class("flat");
            obj.bind_property("selected", &dropdown, "selected")
                .bidirectional()
                .sync_create()
                .build();
            obj.add_suffix(&dropdown);
            let _ = self.dropdown.set(dropdown);
            let weak = obj.downgrade();
            on_row_activated(obj.upcast_ref(), move || {
                if let Some(row) = weak.upgrade() {
                    if let Some(d) = row.imp().dropdown.get() {
                        d.activate();
                    }
                    row.emit_by_name::<()>("activated", &[]);
                }
            });
        }
    }

    impl WidgetImpl for ComboRow {}
    impl ListBoxRowImpl for ComboRow {}
}

glib::wrapper! {
    /// A row that picks one item out of a list.
    pub struct ComboRow(ObjectSubclass<combo_imp::ComboRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

row_common!(ComboRow);
/// Anything that is a list model, for [`ComboRowBuilder::model`].
pub struct ModelArg(gtk::gio::ListModel);

impl<T: IsA<gtk::gio::ListModel>> From<&T> for ModelArg {
    fn from(model: &T) -> Self {
        Self(model.as_ref().clone())
    }
}

row_builder!(ComboRowBuilder, ComboRow, { selected: u32, model: ModelArg }, |row, b| {
    if let Some(m) = &b.model {
        row.set_model(Some(&m.0));
    }
    if let Some(s) = b.selected {
        row.set_selected(s);
    }
});

impl Default for ComboRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ComboRow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> ComboRowBuilder {
        ComboRowBuilder::default()
    }

    pub fn set_model(&self, model: Option<&impl IsA<gtk::gio::ListModel>>) {
        if let Some(d) = self.imp().dropdown.get() {
            let selected = self.selected();
            d.set_model(model);
            d.set_selected(selected);
        }
    }

    pub fn model(&self) -> Option<gtk::gio::ListModel> {
        self.imp().dropdown.get().and_then(|d| d.model())
    }
}

// ---------------------------------------------------------------------------------------------
// SpinRow

mod spin_imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::SpinRow)]
    pub struct SpinRow {
        pub(crate) parts: OnceCell<Parts>,
        #[property(get, set)]
        pub value: Cell<f64>,
        pub spin: OnceCell<gtk::SpinButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SpinRow {
        const NAME: &'static str = "NdlSpinRow";
        type Type = super::SpinRow;
        type ParentType = gtk::ListBoxRow;
    }

    #[glib::derived_properties]
    impl ObjectImpl for SpinRow {
        fn signals() -> &'static [Signal] {
            activated_signal!()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_activatable(false);
            obj.add_css_class("ndl-spin-row");
            let spin = gtk::SpinButton::builder()
                .valign(gtk::Align::Center)
                .build();
            obj.add_suffix(&spin);
            let _ = self.spin.set(spin);
        }
    }

    impl WidgetImpl for SpinRow {}
    impl ListBoxRowImpl for SpinRow {}
}

glib::wrapper! {
    /// A row holding a number with buttons to step it.
    pub struct SpinRow(ObjectSubclass<spin_imp::SpinRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

row_common!(SpinRow);

impl SpinRow {
    /// A row stepping through `min..=max` by `step`, showing as many decimals as `step` needs.
    pub fn with_range(min: f64, max: f64, step: f64) -> Self {
        let row: Self = glib::Object::new();
        if let Some(spin) = row.imp().spin.get() {
            let adjustment = gtk::Adjustment::new(min, min, max, step, step * 10.0, 0.0);
            spin.set_adjustment(&adjustment);
            spin.set_digits(digits_for(step));
            row.set_value(min);
            row.bind_property("value", &adjustment, "value")
                .bidirectional()
                .sync_create()
                .build();
        }
        row
    }

    pub fn spin_button(&self) -> Option<gtk::SpinButton> {
        self.imp().spin.get().cloned()
    }
}

/// Decimal places needed to show every multiple of `step`.
fn digits_for(step: f64) -> u32 {
    let mut digits = 0;
    let mut s = step.abs();
    while digits < 6 && s.is_finite() && (s - s.round()).abs() > 1e-9 {
        s *= 10.0;
        digits += 1;
    }
    digits
}

// ---------------------------------------------------------------------------------------------
// ButtonRow

mod button_imp {
    use super::*;

    #[derive(Default)]
    pub struct ButtonRow {
        pub(crate) parts: OnceCell<Parts>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ButtonRow {
        const NAME: &'static str = "NdlButtonRow";
        type Type = super::ButtonRow;
        type ParentType = gtk::ListBoxRow;
    }

    impl ObjectImpl for ButtonRow {
        fn signals() -> &'static [Signal] {
            activated_signal!()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.add_css_class("ndl-button-row");
            let parts = obj.parts();
            parts.title.set_xalign(0.5);
            parts.titles.set_halign(gtk::Align::Center);
            let weak = obj.downgrade();
            on_row_activated(obj.upcast_ref(), move || {
                if let Some(row) = weak.upgrade() {
                    row.emit_by_name::<()>("activated", &[]);
                }
            });
        }
    }

    impl WidgetImpl for ButtonRow {}
    impl ListBoxRowImpl for ButtonRow {}
}

glib::wrapper! {
    /// A row that is a button: its title centred, the whole row the target.
    pub struct ButtonRow(ObjectSubclass<button_imp::ButtonRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

row_common!(ButtonRow);
row_builder!(ButtonRowBuilder, ButtonRow, {}, |_row, _b| {});

impl Default for ButtonRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonRow {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn builder() -> ButtonRowBuilder {
        ButtonRowBuilder::default()
    }
}

// ---------------------------------------------------------------------------------------------
// EntryRow and PasswordEntryRow

mod entry_imp {
    use super::*;

    #[derive(Default)]
    pub struct EntryRow {
        pub(crate) parts: OnceCell<Parts>,
        pub editable: OnceCell<gtk::Editable>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EntryRow {
        const NAME: &'static str = "NdlEntryRow";
        type Type = super::EntryRow;
        type ParentType = gtk::ListBoxRow;
    }

    impl ObjectImpl for EntryRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("activated").build(),
                    Signal::builder("changed").build(),
                ]
            })
        }
    }

    impl WidgetImpl for EntryRow {}
    impl ListBoxRowImpl for EntryRow {}

    #[derive(Default)]
    pub struct PasswordEntryRow {
        pub(crate) parts: OnceCell<Parts>,
        pub editable: OnceCell<gtk::Editable>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PasswordEntryRow {
        const NAME: &'static str = "NdlPasswordEntryRow";
        type Type = super::PasswordEntryRow;
        type ParentType = gtk::ListBoxRow;
    }

    impl ObjectImpl for PasswordEntryRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("activated").build(),
                    Signal::builder("changed").build(),
                ]
            })
        }
    }

    impl WidgetImpl for PasswordEntryRow {}
    impl ListBoxRowImpl for PasswordEntryRow {}
}

/// Puts the title above an editable that spans the row, and forwards its `changed`.
fn build_entry_row(row: &gtk::ListBoxRow, parts: &Parts, editable: &gtk::Editable) {
    row.set_activatable(false);
    row.add_css_class("ndl-entry-row");
    parts.title.add_css_class("caption");
    parts.titles.append(editable);
    editable.set_hexpand(true);
    let weak = row.downgrade();
    editable.connect_changed(move |_| {
        if let Some(row) = weak.upgrade() {
            row.emit_by_name::<()>("changed", &[]);
        }
    });
    let weak = row.downgrade();
    let focus = gtk::GestureClick::new();
    let target = editable.downgrade();
    focus.connect_released(move |_, _, _, _| {
        if let (Some(_), Some(e)) = (weak.upgrade(), target.upgrade()) {
            e.grab_focus();
        }
    });
    row.add_controller(focus);
    keep_title_in_step(parts, editable);
}

/// An empty field that is not being edited shows its title as the placeholder, centred in the
/// row, instead of a small title above nothing. The title moves up once there is something to
/// label: when the field has text or focus.
fn keep_title_in_step(parts: &Parts, editable: &gtk::Editable) {
    use std::rc::Rc;
    let has_placeholder = editable.find_property("placeholder-text").is_some();
    let sync: Rc<dyn Fn()> = {
        let title = parts.title.downgrade();
        let editable = editable.downgrade();
        Rc::new(move || {
            let (Some(title), Some(editable)) = (title.upgrade(), editable.upgrade()) else {
                return;
            };
            let focused = editable
                .upcast_ref::<gtk::Widget>()
                .state_flags()
                .contains(gtk::StateFlags::FOCUS_WITHIN);
            let resting = editable.text().is_empty() && !focused;
            let shown = !resting || !has_placeholder;
            if title.is_visible() != shown {
                title.set_visible(shown);
            }
            if has_placeholder {
                let text = title.text();
                editable.set_property("placeholder-text", text.as_str());
            }
        })
    };
    let again = sync.clone();
    editable.connect_changed(move |_| again());
    let again = sync.clone();
    editable.connect_state_flags_changed(move |_, _| again());
    let again = sync.clone();
    parts.title.connect_label_notify(move |_| again());
    // Setting a title makes the label visible; the row decides again.
    let again = sync.clone();
    parts.title.connect_visible_notify(move |_| again());
    sync();
}

macro_rules! entry_common {
    ($ty:ty, $builder:ident) => {
        impl $ty {
            fn editable(&self) -> Option<&gtk::Editable> {
                self.imp().editable.get()
            }

            pub fn text(&self) -> glib::GString {
                self.editable().map(|e| e.text()).unwrap_or_default()
            }

            pub fn set_text(&self, text: &str) {
                if let Some(e) = self.editable() {
                    e.set_text(text);
                }
            }

            pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
                self.connect_local("changed", false, move |args| {
                    if let Some(row) = args.first().and_then(|v| v.get::<Self>().ok()) {
                        f(&row);
                    }
                    None
                })
            }

            pub fn builder() -> $builder {
                $builder::default()
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

glib::wrapper! {
    /// A row whose value is a line of text, with its title above it.
    pub struct EntryRow(ObjectSubclass<entry_imp::EntryRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

row_common!(EntryRow);
entry_common!(EntryRow, EntryRowBuilder);
row_builder!(EntryRowBuilder, EntryRow, { text: String }, |row, b| {
    if let Some(t) = &b.text {
        row.set_text(t);
    }
});

impl EntryRow {
    pub fn new() -> Self {
        let row: Self = glib::Object::new();
        let text = gtk::Text::new();
        let editable: gtk::Editable = text.upcast();
        build_entry_row(row.upcast_ref(), row.parts(), &editable);
        let _ = row.imp().editable.set(editable);
        row
    }
}

glib::wrapper! {
    /// An [`EntryRow`] that hides what is typed, with a button to show it.
    pub struct PasswordEntryRow(ObjectSubclass<entry_imp::PasswordEntryRow>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

row_common!(PasswordEntryRow);
entry_common!(PasswordEntryRow, PasswordEntryRowBuilder);
row_builder!(PasswordEntryRowBuilder, PasswordEntryRow, { text: String }, |row, b| {
    if let Some(t) = &b.text {
        row.set_text(t);
    }
});

impl PasswordEntryRow {
    pub fn new() -> Self {
        let row: Self = glib::Object::new();
        let entry = gtk::PasswordEntry::builder().show_peek_icon(true).build();
        entry.add_css_class("flat");
        let editable: gtk::Editable = entry.upcast();
        build_entry_row(row.upcast_ref(), row.parts(), &editable);
        let _ = row.imp().editable.set(editable);
        row
    }
}

#[cfg(test)]
mod tests {
    use super::digits_for;

    #[test]
    fn digits_follow_the_step() {
        assert_eq!(digits_for(1.0), 0);
        assert_eq!(digits_for(0.5), 1);
        assert_eq!(digits_for(0.25), 2);
        assert_eq!(digits_for(f64::NAN), 0);
    }
}
