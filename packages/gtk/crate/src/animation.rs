//! A value eased from one number to another over a fixed time, on the widget's frame clock.
//!
//! The system's reduced-motion preference (`gtk-enable-animations` off) makes every animation
//! jump to its end on the next frame.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;

/// Easing curves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Easing {
    Linear,
    EaseOutQuad,
    #[default]
    EaseOutCubic,
    EaseInOutCubic,
}

impl Easing {
    /// The curve at `t` in `0..=1`; out-of-range input is clamped.
    pub fn ease(self, t: f64) -> f64 {
        let t = if t.is_nan() { 1.0 } else { t.clamp(0.0, 1.0) };
        match self {
            Self::Linear => t,
            Self::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
        }
    }
}

/// Where an animation sends its values.
#[derive(Clone)]
pub struct CallbackAnimationTarget(Rc<dyn Fn(f64)>);

impl CallbackAnimationTarget {
    pub fn new(f: impl Fn(f64) + 'static) -> Self {
        Self(Rc::new(f))
    }
}

/// Whether the animation has run, is running, or has finished.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Playing,
    Paused,
    Finished,
}

struct Inner {
    widget: glib::WeakRef<gtk::Widget>,
    from: Cell<f64>,
    to: Cell<f64>,
    duration_ms: Cell<u32>,
    easing: Cell<Easing>,
    target: CallbackAnimationTarget,
    value: Cell<f64>,
    state: Cell<AnimationState>,
    tick: RefCell<Option<gtk::TickCallbackId>>,
    start_us: Cell<Option<i64>>,
    done: RefCell<Vec<Box<dyn Fn()>>>,
}

/// An animation from one value to another.
#[derive(Clone)]
pub struct TimedAnimation(Rc<Inner>);

impl TimedAnimation {
    pub fn new(
        widget: &impl IsA<gtk::Widget>,
        from: f64,
        to: f64,
        duration_ms: u32,
        target: CallbackAnimationTarget,
    ) -> Self {
        Self(Rc::new(Inner {
            widget: widget.as_ref().downgrade(),
            from: Cell::new(from),
            to: Cell::new(to),
            duration_ms: Cell::new(duration_ms),
            easing: Cell::new(Easing::EaseOutCubic),
            target,
            value: Cell::new(from),
            state: Cell::new(AnimationState::Idle),
            tick: RefCell::new(None),
            start_us: Cell::new(None),
            done: RefCell::new(Vec::new()),
        }))
    }

    pub fn set_easing(&self, easing: Easing) {
        self.0.easing.set(easing);
    }
    pub fn set_value_from(&self, v: f64) {
        self.0.from.set(v);
    }
    pub fn set_value_to(&self, v: f64) {
        self.0.to.set(v);
    }
    pub fn value_from(&self) -> f64 {
        self.0.from.get()
    }
    pub fn value_to(&self) -> f64 {
        self.0.to.get()
    }
    pub fn set_duration(&self, ms: u32) {
        self.0.duration_ms.set(ms);
    }
    pub fn value(&self) -> f64 {
        self.0.value.get()
    }
    pub fn state(&self) -> AnimationState {
        self.0.state.get()
    }

    pub fn connect_done(&self, f: impl Fn() + 'static) {
        self.0.done.borrow_mut().push(Box::new(f));
    }

    fn set(&self, v: f64) {
        self.0.value.set(v);
        (self.0.target.0)(v);
    }

    fn stop_ticking(&self) {
        if let Some(id) = self.0.tick.borrow_mut().take() {
            id.remove();
        }
    }

    /// Starts from the beginning, interrupting a run in progress.
    pub fn play(&self) {
        self.stop_ticking();
        let animations = gtk::Settings::default().is_none_or(|s| s.is_gtk_enable_animations());
        let Some(widget) = self.0.widget.upgrade() else {
            self.skip();
            return;
        };
        if !animations || self.0.duration_ms.get() == 0 || !widget.is_mapped() {
            self.skip();
            return;
        }
        self.0.state.set(AnimationState::Playing);
        self.0.start_us.set(None);
        self.set(self.0.from.get());
        let this = self.clone();
        let id = widget.add_tick_callback(move |_, clock| {
            let now = clock.frame_time();
            let start = this.0.start_us.get().unwrap_or(now);
            this.0.start_us.set(Some(start));
            let elapsed_ms = (now.saturating_sub(start)) as f64 / 1000.0;
            let t = elapsed_ms / f64::from(this.0.duration_ms.get().max(1));
            let (from, to) = (this.0.from.get(), this.0.to.get());
            let eased = this.0.easing.get().ease(t);
            this.set(from + (to - from) * eased);
            if t >= 1.0 {
                this.0.tick.borrow_mut().take();
                this.finish();
                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
        self.0.tick.replace(Some(id));
    }

    /// Stops where it is, without finishing. `play` starts again from the beginning.
    pub fn pause(&self) {
        self.stop_ticking();
        if self.0.state.get() == AnimationState::Playing {
            self.0.state.set(AnimationState::Paused);
        }
    }

    /// Jumps to the end.
    pub fn skip(&self) {
        self.stop_ticking();
        self.set(self.0.to.get());
        self.finish();
    }

    /// Back to the start, without telling anyone it finished.
    pub fn reset(&self) {
        self.stop_ticking();
        self.0.state.set(AnimationState::Idle);
        self.set(self.0.from.get());
    }

    fn finish(&self) {
        self.0.state.set(AnimationState::Finished);
        for f in self.0.done.borrow().iter() {
            f();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curves_start_at_zero_and_end_at_one() {
        for e in [
            Easing::Linear,
            Easing::EaseOutQuad,
            Easing::EaseOutCubic,
            Easing::EaseInOutCubic,
        ] {
            assert_eq!(e.ease(0.0), 0.0);
            assert!((e.ease(1.0) - 1.0).abs() < 1e-12);
            assert_eq!(e.ease(-3.0), 0.0);
            assert!((e.ease(7.0) - 1.0).abs() < 1e-12);
            assert!((e.ease(f64::NAN) - 1.0).abs() < 1e-12);
        }
    }
}
