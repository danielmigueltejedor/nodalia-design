//! The About and keyboard-shortcuts dialogs.

use std::rc::Rc;

use gtk::prelude::*;

use crate::dialog::Dialog;
use crate::misc::Clamp;
use crate::prefs::PreferencesGroup;
use crate::rows::ActionRow;
use crate::toolbar::{HeaderBar, ToolbarView};

/// The release notes as plain lines: paragraphs as they are, list items with a bullet. Accepts
/// the small subset of HTML AppStream release notes use (`p`, `ul`, `ol`, `li`, `em`, `code`).
pub(crate) fn release_notes_lines(html: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut rest = html;
    let flush = |current: &mut String, lines: &mut Vec<String>| {
        let text = current.split_whitespace().collect::<Vec<_>>().join(" ");
        if !text.is_empty() {
            lines.push(text);
        }
        current.clear();
    };
    while let Some(open) = rest.find('<') {
        current.push_str(&rest[..open]);
        let Some(close) = rest[open..].find('>') else {
            current.push_str(&rest[open..]);
            rest = "";
            break;
        };
        let tag = rest[open + 1..open + close].trim().to_ascii_lowercase();
        match tag.as_str() {
            "p" | "/p" | "/li" | "ul" | "/ul" | "ol" | "/ol" => flush(&mut current, &mut lines),
            "li" => {
                flush(&mut current, &mut lines);
                current.push_str("• ");
            }
            _ => {}
        }
        rest = &rest[open + close + 1..];
    }
    current.push_str(rest);
    flush(&mut current, &mut lines);
    lines
        .into_iter()
        .map(|l| {
            l.replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
        })
        .collect()
}

fn license_name(license: gtk::License) -> &'static str {
    match license {
        gtk::License::Gpl20 => "GNU GPL 2.0",
        gtk::License::Gpl30 => "GNU GPL 3.0",
        gtk::License::Gpl20Only => "GNU GPL 2.0 only",
        gtk::License::Gpl30Only => "GNU GPL 3.0 only",
        gtk::License::Lgpl21 => "GNU LGPL 2.1",
        gtk::License::Lgpl30 => "GNU LGPL 3.0",
        gtk::License::MitX11 => "MIT",
        gtk::License::Apache20 => "Apache 2.0",
        gtk::License::Bsd => "BSD",
        gtk::License::Bsd3 => "BSD 3-Clause",
        gtk::License::Mpl20 => "MPL 2.0",
        gtk::License::Agpl30 => "GNU AGPL 3.0",
        gtk::License::Artistic => "Artistic 2.0",
        _ => "",
    }
}

/// A dialog about the application: its icon, name, version, who made it and what changed.
#[derive(Clone)]
pub struct AboutDialog(Rc<Dialog>);

#[derive(Default)]
#[must_use]
pub struct AboutDialogBuilder {
    application_icon: Option<String>,
    application_name: Option<String>,
    developer_name: Option<String>,
    version: Option<String>,
    comments: Option<String>,
    website: Option<String>,
    release_notes: Option<String>,
    release_notes_version: Option<String>,
    license_type: Option<gtk::License>,
}

macro_rules! text_setter {
    ($($name:ident),*) => {
        $(
            pub fn $name(mut self, value: impl Into<String>) -> Self {
                self.$name = Some(value.into());
                self
            }
        )*
    };
}

impl AboutDialogBuilder {
    text_setter!(
        application_icon,
        application_name,
        developer_name,
        version,
        comments,
        website,
        release_notes,
        release_notes_version
    );

    pub fn license_type(mut self, license: gtk::License) -> Self {
        self.license_type = Some(license);
        self
    }

    pub fn build(self) -> AboutDialog {
        let column = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_top(12)
            .margin_bottom(24)
            .margin_start(12)
            .margin_end(12)
            .build();
        column.add_css_class("ndl-about");
        if let Some(icon) = &self.application_icon {
            let image = gtk::Image::builder()
                .icon_name(icon.as_str())
                .pixel_size(128)
                .css_classes(["icon"])
                .build();
            column.append(&image);
        }
        let label = |text: &str, classes: &[&str]| {
            gtk::Label::builder()
                .label(text)
                .wrap(true)
                .justify(gtk::Justification::Center)
                .css_classes(classes.iter().map(|c| (*c).to_owned()).collect::<Vec<_>>())
                .build()
        };
        if let Some(name) = &self.application_name {
            column.append(&label(name, &["title-1"]));
        }
        if let Some(dev) = &self.developer_name {
            column.append(&label(dev, &["dim-label"]));
        }
        if let Some(version) = &self.version {
            let pill = label(version, &["ndl-pill", "caption"]);
            pill.set_halign(gtk::Align::Center);
            column.append(&pill);
        }
        if let Some(comments) = &self.comments {
            column.append(&label(comments, &["body"]));
        }

        let details = PreferencesGroup::new();
        if let Some(website) = &self.website {
            let row = ActionRow::builder()
                .title(website.as_str())
                .activatable(true)
                .build();
            row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));
            let url = website.clone();
            row.connect_activated(move |row| {
                let window = row.root().and_downcast::<gtk::Window>();
                gtk::UriLauncher::new(&url).launch(
                    window.as_ref(),
                    gtk::gio::Cancellable::NONE,
                    |_| {},
                );
            });
            details.add(&row);
        }
        if let Some(license) = self.license_type {
            let name = license_name(license);
            if !name.is_empty() {
                let row = ActionRow::builder().title(name).build();
                row.add_prefix(&gtk::Image::from_icon_name("emblem-documents-symbolic"));
                details.add(&row);
            }
        }
        column.append(&details);

        if let Some(notes) = &self.release_notes {
            let lines = release_notes_lines(notes);
            if !lines.is_empty() {
                let heading = self
                    .release_notes_version
                    .as_deref()
                    .map_or_else(|| "What’s New".to_owned(), |v| format!("What’s New in {v}"));
                let group = PreferencesGroup::builder().title(heading).build();
                let body = gtk::Box::new(gtk::Orientation::Vertical, 6);
                body.add_css_class("ndl-release-notes");
                for line in &lines {
                    let l = gtk::Label::builder()
                        .label(line.as_str())
                        .wrap(true)
                        .xalign(0.0)
                        .build();
                    body.append(&l);
                }
                group.add(&body);
                column.append(&group);
            }
        }

        let clamp = Clamp::builder().maximum_size(420).child(&column).build();
        let scroller = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .propagate_natural_height(true)
            .vexpand(true)
            .child(&clamp)
            .build();
        let view = ToolbarView::new();
        let header = HeaderBar::new();
        header.set_title_widget(Some(&gtk::Label::new(None)));
        header.add_css_class("flat");
        view.add_top_bar(&header);
        view.set_content(Some(&scroller));
        let dialog = Dialog::builder()
            .title(self.application_name.clone().unwrap_or_default())
            .child(&view)
            .content_width(420)
            .content_height(560)
            .build();
        dialog.add_css_class("about");
        AboutDialog(Rc::new(dialog))
    }
}

impl AboutDialog {
    pub fn builder() -> AboutDialogBuilder {
        AboutDialogBuilder::default()
    }

    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.0.present(parent);
    }

    pub fn dialog(&self) -> &Dialog {
        &self.0
    }
}

/// One shortcut: what it does, and its keys drawn as keycaps.
#[derive(Clone)]
pub struct ShortcutsItem(ActionRow);

impl ShortcutsItem {
    /// `accelerator` is in `gtk_accelerator_parse` form, e.g. `<Control>k`.
    pub fn new(title: &str, accelerator: &str) -> Self {
        let row = ActionRow::builder().title(title).build();
        let keys = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        keys.set_valign(gtk::Align::Center);
        for part in accelerator_parts(accelerator) {
            let cap = gtk::Label::builder()
                .label(part.as_str())
                .css_classes(["keycap"])
                .build();
            keys.append(&cap);
        }
        row.add_suffix(&keys);
        Self(row)
    }
}

/// The keys of an accelerator as the platform labels them, one per cap.
pub(crate) fn accelerator_parts(accelerator: &str) -> Vec<String> {
    let Some((key, mods)) = gtk::accelerator_parse(accelerator) else {
        return vec![accelerator.to_owned()];
    };
    let label = gtk::accelerator_get_label(key, mods);
    split_accel_label(&label)
}

/// "Ctrl+Shift++" → ["Ctrl", "Shift", "+"]: a trailing `+` is the key, not a separator.
pub(crate) fn split_accel_label(label: &str) -> Vec<String> {
    let (body, plus_key) = match label.strip_suffix("++") {
        Some(body) => (body, true),
        None if label == "+" => ("", true),
        None => (label, false),
    };
    let mut parts: Vec<String> = body
        .split('+')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect();
    if plus_key {
        parts.push("+".to_owned());
    }
    parts
}

/// A titled group of [`ShortcutsItem`]s.
#[derive(Clone)]
pub struct ShortcutsSection(PreferencesGroup);

impl ShortcutsSection {
    pub fn new(title: Option<&str>) -> Self {
        let group = PreferencesGroup::new();
        if let Some(t) = title {
            group.set_title(t);
        }
        Self(group)
    }

    pub fn add(&self, item: ShortcutsItem) {
        self.0.add(&item.0);
    }
}

/// A dialog listing an application's keyboard shortcuts by section.
#[derive(Clone)]
pub struct ShortcutsDialog(Rc<(Dialog, gtk::Box)>);

impl Default for ShortcutsDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutsDialog {
    pub fn new() -> Self {
        let column = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(24)
            .margin_top(12)
            .margin_bottom(24)
            .margin_start(12)
            .margin_end(12)
            .build();
        let clamp = Clamp::builder().maximum_size(560).child(&column).build();
        let scroller = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vexpand(true)
            .child(&clamp)
            .build();
        let view = ToolbarView::new();
        view.add_top_bar(&HeaderBar::new());
        view.set_content(Some(&scroller));
        let dialog = Dialog::builder()
            .child(&view)
            .content_width(560)
            .content_height(620)
            .build();
        dialog.add_css_class("shortcuts");
        Self(Rc::new((dialog, column)))
    }

    pub fn set_title(&self, title: &str) {
        self.0 .0.set_title(title);
    }

    pub fn add(&self, section: ShortcutsSection) {
        self.0 .1.append(&section.0);
    }

    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.0 .0.present(parent);
    }
}

#[cfg(test)]
mod tests {
    use super::{release_notes_lines, split_accel_label};

    #[test]
    fn release_notes_become_lines() {
        let lines = release_notes_lines(
            "<p>Vinilo 2.0</p><ul><li>One &amp; two.</li>\n<li>Three</li></ul>",
        );
        assert_eq!(lines, ["Vinilo 2.0", "• One & two.", "• Three"]);
    }

    #[test]
    fn unclosed_tags_do_not_lose_text() {
        assert_eq!(release_notes_lines("plain <b"), ["plain <b"]);
    }

    #[test]
    fn accelerator_labels_split_on_plus() {
        assert_eq!(split_accel_label("Ctrl+K"), ["Ctrl", "K"]);
        assert_eq!(split_accel_label("Ctrl+Shift++"), ["Ctrl", "Shift", "+"]);
        assert_eq!(split_accel_label("F9"), ["F9"]);
        assert_eq!(split_accel_label("+"), ["+"]);
    }
}
