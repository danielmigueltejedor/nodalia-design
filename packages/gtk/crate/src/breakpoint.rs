//! Size breakpoints: a condition on the space a window or a [`BreakpointBin`] has, and property
//! values that hold while it is met.
//!
//! When several breakpoints match, the one added last wins, and only one is in force at a time.
//! Leaving a breakpoint puts back the values its setters replaced.

use std::cell::{Cell, RefCell};
use std::sync::OnceLock;

use gtk::glib;
use gtk::glib::subclass::Signal;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

/// One comparison: `max-width: 600px` and the like.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Term {
    MinWidth(f64),
    MaxWidth(f64),
    MinHeight(f64),
    MaxHeight(f64),
}

impl Term {
    fn holds(self, width: f64, height: f64) -> bool {
        match self {
            Self::MinWidth(v) => width >= v,
            Self::MaxWidth(v) => width <= v,
            Self::MinHeight(v) => height >= v,
            Self::MaxHeight(v) => height <= v,
        }
    }
}

/// A condition: terms joined by `and` (all) or `or` (any). Mixing the two is not supported.
#[derive(Debug, Clone, PartialEq)]
pub struct BreakpointCondition {
    terms: Vec<Term>,
    any: bool,
}

/// Why a condition string was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionError(pub String);

impl std::fmt::Display for ConditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid breakpoint condition: {}", self.0)
    }
}

impl std::error::Error for ConditionError {}

impl BreakpointCondition {
    /// Parses `max-width: 600px`, `min-width: 400px and max-height: 300px`, `… or …`.
    /// Units: `px`, or `pt` (1pt = 4/3 px); a bare number is px.
    pub fn parse(text: &str) -> Result<Self, ConditionError> {
        let lower = text.trim().to_ascii_lowercase();
        let (parts, any): (Vec<&str>, bool) = if lower.contains(" or ") {
            if lower.contains(" and ") {
                return Err(ConditionError("mixes `and` and `or`".into()));
            }
            (lower.split(" or ").collect(), true)
        } else {
            (lower.split(" and ").collect(), false)
        };
        let mut terms = Vec::new();
        for part in parts {
            let part = part.trim().trim_start_matches('(').trim_end_matches(')');
            let Some((name, value)) = part.split_once(':') else {
                return Err(ConditionError(format!("`{part}` has no `:`")));
            };
            let value = value.trim();
            let (number, scale) = if let Some(n) = value.strip_suffix("px") {
                (n, 1.0)
            } else if let Some(n) = value.strip_suffix("pt") {
                (n, 4.0 / 3.0)
            } else {
                (value, 1.0)
            };
            let number: f64 = number
                .trim()
                .parse()
                .map_err(|_| ConditionError(format!("`{value}` is not a length")))?;
            if !number.is_finite() || number < 0.0 {
                return Err(ConditionError(format!("`{value}` is not a length")));
            }
            let v = number * scale;
            terms.push(match name.trim() {
                "min-width" => Term::MinWidth(v),
                "max-width" => Term::MaxWidth(v),
                "min-height" => Term::MinHeight(v),
                "max-height" => Term::MaxHeight(v),
                other => return Err(ConditionError(format!("unknown feature `{other}`"))),
            });
        }
        if terms.is_empty() {
            return Err(ConditionError("empty".into()));
        }
        Ok(Self { terms, any })
    }

    /// Whether the condition holds for this much space.
    pub fn holds(&self, width: f64, height: f64) -> bool {
        if self.any {
            self.terms.iter().any(|t| t.holds(width, height))
        } else {
            self.terms.iter().all(|t| t.holds(width, height))
        }
    }
}

pub(crate) struct Setter {
    target: glib::WeakRef<glib::Object>,
    property: String,
    value: glib::Value,
    saved: Option<glib::Value>,
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct Breakpoint {
        pub condition: RefCell<Option<BreakpointCondition>>,
        pub(crate) setters: RefCell<Vec<Setter>>,
        pub applied: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Breakpoint {
        const NAME: &'static str = "NdlBreakpoint";
        type Type = super::Breakpoint;
    }

    impl ObjectImpl for Breakpoint {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("apply").build(),
                    Signal::builder("unapply").build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    pub struct Breakpoint(ObjectSubclass<imp::Breakpoint>);
}

impl Breakpoint {
    pub fn new(condition: BreakpointCondition) -> Self {
        let this: Self = glib::Object::new();
        this.imp().condition.replace(Some(condition));
        this
    }

    /// While this breakpoint is in force, `property` of `target` is `value`.
    pub fn add_setter(
        &self,
        target: &impl IsA<glib::Object>,
        property: &str,
        value: Option<&glib::Value>,
    ) {
        let Some(value) = value.cloned() else { return };
        self.imp().setters.borrow_mut().push(Setter {
            target: target.upcast_ref::<glib::Object>().downgrade(),
            property: property.to_owned(),
            value,
            saved: None,
        });
    }

    pub fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("apply", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }

    pub fn connect_unapply<F: Fn(&Self) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_local("unapply", false, move |args| {
            if let Some(Ok(this)) = args.first().map(|v| v.get::<Self>()) {
                f(&this);
            }
            None
        })
    }

    pub(crate) fn matches(&self, width: f64, height: f64) -> bool {
        self.imp()
            .condition
            .borrow()
            .as_ref()
            .is_some_and(|c| c.holds(width, height))
    }

    pub(crate) fn apply(&self) {
        let imp = self.imp();
        if imp.applied.replace(true) {
            return;
        }
        for setter in imp.setters.borrow_mut().iter_mut() {
            let Some(target) = setter.target.upgrade() else {
                continue;
            };
            if target.find_property(&setter.property).is_none() {
                continue;
            }
            setter.saved = Some(target.property_value(&setter.property));
            target.set_property_from_value(&setter.property, &setter.value);
        }
        self.emit_by_name::<()>("apply", &[]);
    }

    pub(crate) fn unapply(&self) {
        let imp = self.imp();
        if !imp.applied.replace(false) {
            return;
        }
        for setter in imp.setters.borrow_mut().iter_mut() {
            let (Some(target), Some(saved)) = (setter.target.upgrade(), setter.saved.take()) else {
                continue;
            };
            target.set_property_from_value(&setter.property, &saved);
        }
        self.emit_by_name::<()>("unapply", &[]);
    }
}

/// The breakpoints of one container and the one in force.
#[derive(Default)]
pub(crate) struct Breakpoints {
    list: RefCell<Vec<Breakpoint>>,
    current: RefCell<Option<Breakpoint>>,
    pending: Cell<Option<(i32, i32)>>,
    scheduled: Cell<bool>,
}

impl Breakpoints {
    pub fn add(&self, breakpoint: Breakpoint) {
        self.list.borrow_mut().push(breakpoint);
    }

    /// Records the size and re-evaluates on idle: setters change layout, which must not happen
    /// in the middle of the allocation that reported the size.
    pub fn size_changed(&self, owner: &gtk::Widget, width: i32, height: i32) {
        if self.list.borrow().is_empty() {
            return;
        }
        self.pending.set(Some((width, height)));
        if self.scheduled.replace(true) {
            return;
        }
        let weak = owner.downgrade();
        glib::idle_add_local_once(move || {
            if let Some(owner) = weak.upgrade() {
                crate::window::evaluate_breakpoints(&owner);
            }
        });
    }

    pub fn evaluate(&self) {
        self.scheduled.set(false);
        let Some((w, h)) = self.pending.take() else {
            return;
        };
        let winner = self
            .list
            .borrow()
            .iter()
            .rev()
            .find(|b| b.matches(f64::from(w), f64::from(h)))
            .cloned();
        if *self.current.borrow() == winner {
            return;
        }
        if let Some(old) = self.current.replace(winner.clone()) {
            old.unapply();
        }
        if let Some(new) = winner {
            new.apply();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_widths_and_heights() {
        let c = BreakpointCondition::parse("max-width: 600px").expect("parse");
        assert!(c.holds(600.0, 10.0));
        assert!(!c.holds(601.0, 10.0));
        let c =
            BreakpointCondition::parse("min-width: 400px and max-height: 300px").expect("parse");
        assert!(c.holds(500.0, 200.0));
        assert!(!c.holds(500.0, 400.0));
        let c = BreakpointCondition::parse("max-width: 100px or max-height: 100px").expect("parse");
        assert!(c.holds(500.0, 50.0));
    }

    #[test]
    fn points_are_four_thirds_of_a_pixel() {
        let c = BreakpointCondition::parse("max-width: 300pt").expect("parse");
        assert!(c.holds(400.0, 0.0));
        assert!(!c.holds(401.0, 0.0));
    }

    #[test]
    fn refuses_what_it_cannot_read() {
        for bad in [
            "",
            "width: 3px",
            "max-width 3px",
            "max-width: -3px",
            "max-width: wide",
            "max-width: 1px and min-width: 1px or max-height: 2px",
            "max-width: NaN",
        ] {
            assert!(BreakpointCondition::parse(bad).is_err(), "{bad}");
        }
    }
}
