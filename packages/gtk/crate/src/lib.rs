//! `nodalia-gtk`: the Nodalia Design Language for GTK 4, without libadwaita.
//!
//! Two halves. [`StyleManager`] installs NDL's stylesheets (colours per mode, metrics per
//! density, and one sheet styling GTK's widgets and this crate's) and follows the system's
//! light/dark and contrast preferences. The widgets cover what an adaptive GNOME-style app
//! needs and GTK alone does not have: navigation, split views, bottom sheets, in-window
//! dialogs, breakpoints, preference rows and toasts. Their names and methods follow
//! libadwaita's where the job is the same, so porting an app is mostly a change of prefix.
//!
//! Call [`init`] once GTK is initialised and before the first window is shown.

#![forbid(unsafe_code)]

mod about;
mod animation;
mod breakpoint;
mod dialog;
mod fonts;
mod global_menu;
mod icons;
mod layout;
mod menu;
mod misc;
mod navigation;
mod prefs;
mod rows;
mod theme;
mod toolbar;
mod window;

pub use gtk;

pub use icons::{has_icon, THEME as ICON_THEME};

pub use about::{
    AboutDialog, AboutDialogBuilder, ShortcutsDialog, ShortcutsItem, ShortcutsSection,
};
pub use animation::{AnimationState, CallbackAnimationTarget, Easing, TimedAnimation};
pub use breakpoint::{Breakpoint, BreakpointCondition, ConditionError};
pub use dialog::{AlertDialog, Dialog, DialogBuilder, ResponseAppearance};
pub use global_menu::{global_menu, MenuEntry, TopMenu};
pub use layout::{BottomSheet, OverlaySplitView};
pub use menu::{menu_item, menu_item_danger, menu_rows};
pub use misc::{
    Avatar, AvatarBuilder, Clamp, ClampBuilder, ClampScrollable, Spinner, SpinnerBuilder,
    StatusPage, StatusPageBuilder, Toast, ToastOverlay,
};
pub use navigation::{NavigationPage, NavigationPageBuilder, NavigationView};
pub use prefs::{PreferencesDialog, PreferencesGroup, PreferencesGroupBuilder, PreferencesPage};
pub use rows::{
    ActionRow, ActionRowBuilder, ButtonRow, ButtonRowBuilder, ComboRow, ComboRowBuilder, EntryRow,
    EntryRowBuilder, PasswordEntryRow, PasswordEntryRowBuilder, SpinRow, SwitchRow,
    SwitchRowBuilder,
};
pub use theme::{
    set_window_controls, window_controls, ColorScheme, Density, StyleManager, WindowControls,
    WIDGETS_CSS,
};
pub use toolbar::{HeaderBar, HeaderBarBuilder, ToolbarView, ToolbarViewBuilder, WindowTitle};
pub use window::{ApplicationWindow, BreakpointBin, Window, WindowBuilder};

/// GTK's prelude, re-exported so an app needs one import for both.
pub mod prelude {
    pub use gtk::prelude::*;
}

/// Installs NDL's stylesheets on the default display. Safe to call more than once; does nothing
/// until GTK has a display.
pub fn init() {
    let _ = StyleManager::default();
}
