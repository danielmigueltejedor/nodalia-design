//! Builds every widget and checks the layout contracts apps rely on.
//!
//! Needs a display (CI runs it under `xvfb-run`); without one it passes without checking,
//! like the rest of the suite that only parses CSS.

use ndl::gtk;
use ndl::gtk::glib;
use ndl::gtk::prelude::*;
use nodalia_gtk as ndl;

/// One test, not several: GTK must stay on the thread that initialised it.
#[test]
fn widgets() {
    if gtk::init().is_err() {
        eprintln!("no display: widget checks skipped");
        return;
    }
    every_row_type_can_be_built();
    dialog_takes_its_content_size();
    avatar_takes_its_size();
    sidebar_fills_the_pane();
    toolbar_reports_its_bars();
    navigation_pops_back_to_the_root();
    window_title_stacks_its_subtitle();
    window_controls_are_not_stretched();
    alert_answers_after_its_handle_is_dropped();
    alert_is_centred_over_the_window();
}

fn find_buttons(widget: &gtk::Widget, found: &mut Vec<gtk::Button>) {
    let mut child = widget.first_child();
    while let Some(current) = child {
        if let Ok(button) = current.clone().downcast::<gtk::Button>() {
            found.push(button);
        }
        find_buttons(&current, found);
        child = current.next_sibling();
    }
}

/// Apps present an alert and drop it; its buttons still have to answer. They did nothing
/// when the alert lived only in the caller's local.
fn alert_answers_after_its_handle_is_dropped() {
    let window = ndl::Window::new();
    window.set_content(Some(&gtk::Label::new(Some("content"))));
    let answered = std::rc::Rc::new(std::cell::RefCell::new(None::<String>));
    let dialog = {
        let alert = ndl::AlertDialog::new(Some("Update"), Some("Body"));
        alert.add_response("later", "Later");
        alert.add_response("install", "Install");
        let seen = answered.clone();
        alert.connect_response(None, move |_, id| {
            seen.replace(Some(id.to_owned()));
        });
        alert.present(Some(&window));
        alert.dialog().clone()
    };
    let mut buttons = Vec::new();
    find_buttons(dialog.upcast_ref(), &mut buttons);
    assert_eq!(buttons.len(), 2);
    buttons[1].emit_clicked();
    assert_eq!(answered.borrow().as_deref(), Some("install"));
    assert!(dialog.parent().is_none(), "the alert closes once answered");
}

/// The alert floats in the middle of the window, not over its header bar.
fn alert_is_centred_over_the_window() {
    let window = ndl::Window::new();
    window.set_content(Some(&gtk::Label::new(Some("content"))));
    let alert = ndl::AlertDialog::new(Some("Update"), Some("Body"));
    alert.add_response("later", "Later");
    alert.present(Some(&window));
    let dialog = alert.dialog().clone();
    let Some(host) = dialog.parent() else {
        panic!("presented on the window's dialog layer");
    };
    host.measure(gtk::Orientation::Vertical, 800);
    host.allocate(800, 600, -1, None);
    let height = dialog.height();
    assert!(height > 0 && height < 600);
    let Some(bounds) = dialog.compute_bounds(&host) else {
        panic!("the dialog has bounds in its host");
    };
    let top = bounds.y() as i32;
    let bottom = 600 - (top + height);
    assert!((top - bottom).abs() <= 2, "top {top}, bottom {bottom}");
    alert.dialog().force_close();
}

/// A header taller than the window controls centres them: stretched, the circles became pills.
fn window_controls_are_not_stretched() {
    let header = ndl::HeaderBar::new();
    let tall = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    tall.set_size_request(10, 60);
    header.pack_start(&tall);
    header.measure(gtk::Orientation::Vertical, 400);
    header.allocate(400, 80, -1, None);
    let mut controls = Vec::new();
    collect_window_controls(header.upcast_ref(), &mut controls);
    assert_eq!(controls.len(), 2);
    for control in controls {
        assert_eq!(control.valign(), gtk::Align::Center);
        assert!(
            control.height() < 60,
            "window controls stretched to {}",
            control.height()
        );
    }
}

fn collect_window_controls(widget: &gtk::Widget, found: &mut Vec<gtk::WindowControls>) {
    let mut child = widget.first_child();
    while let Some(current) = child {
        if let Ok(controls) = current.clone().downcast::<gtk::WindowControls>() {
            found.push(controls);
        }
        collect_window_controls(&current, found);
        child = current.next_sibling();
    }
}

/// A title and subtitle read top to bottom, never "Cola1 canción" on one line.
fn window_title_stacks_its_subtitle() {
    let title = ndl::WindowTitle::new("Cola", "1 canción");
    let one = ndl::WindowTitle::new("Cola", "");
    assert!(
        natural(&title, gtk::Orientation::Vertical) > natural(&one, gtk::Orientation::Vertical)
    );
}

/// Each row type registers its own `activated` signal; a shared one panicked on the second.
fn every_row_type_can_be_built() {
    let _ = ndl::ActionRow::new();
    let _ = ndl::SwitchRow::new();
    let _ = ndl::ComboRow::new();
    let _ = ndl::ButtonRow::new();
    let _ = glib::Object::new::<ndl::EntryRow>();
    let _ = glib::Object::new::<ndl::PasswordEntryRow>();
    let _ = glib::Object::new::<ndl::SpinRow>();
}

fn natural(widget: &impl IsA<gtk::Widget>, orientation: gtk::Orientation) -> i32 {
    widget.as_ref().measure(orientation, -1).1
}

fn dialog_takes_its_content_size() {
    let dialog = ndl::Dialog::builder()
        .child(&gtk::Label::new(Some("x")))
        .content_width(420)
        .content_height(300)
        .build();
    assert_eq!(natural(&dialog, gtk::Orientation::Horizontal), 420);
    assert_eq!(natural(&dialog, gtk::Orientation::Vertical), 300);
}

fn avatar_takes_its_size() {
    let avatar = ndl::Avatar::new();
    avatar.set_size(64);
    assert_eq!(natural(&avatar, gtk::Orientation::Horizontal), 64);
}

/// A scroller in the sidebar must get the pane's height, not its own natural height of ~0.
fn sidebar_fills_the_pane() {
    let scroller = gtk::ScrolledWindow::new();
    let split = ndl::OverlaySplitView::new();
    split.set_sidebar(Some(&scroller));
    split.set_content(Some(&gtk::Label::new(Some("content"))));
    split.measure(gtk::Orientation::Horizontal, -1);
    split.allocate(800, 600, -1, None);
    assert_eq!(scroller.height(), 600);
}

fn toolbar_reports_its_bars() {
    let view = ndl::ToolbarView::new();
    let bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    bar.set_size_request(-1, 40);
    view.add_top_bar(&bar);
    view.set_content(Some(&gtk::Label::new(Some("content"))));
    view.measure(gtk::Orientation::Vertical, 400);
    view.allocate(400, 300, -1, None);
    assert_eq!(bar.height(), 40);
    // Reported from an idle, after allocation.
    let context = glib::MainContext::default();
    while context.iteration(false) {}
    assert_eq!(view.top_bar_height(), 40);
}

fn navigation_pops_back_to_the_root() {
    let nav = ndl::NavigationView::new();
    nav.add(&ndl::NavigationPage::new(&gtk::Label::new(None), "root"));
    nav.push(&ndl::NavigationPage::new(&gtk::Label::new(None), "detail"));
    assert!(nav.pop());
    assert!(!nav.pop());
}
