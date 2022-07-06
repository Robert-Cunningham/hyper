use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use crate::rt::{Interval, Sleep, Timer};

/// A user-provided timer to time background tasks.
pub(crate) type Tim = Option<Arc<dyn Timer + Send + Sync>>;

impl Timer for Tim {
    fn sleep(&self, duration: Duration) -> Box<dyn Sleep + Unpin> {
        match *self {
            None => {
                panic!("You must supply a timer.")
            }
            Some(ref t) => t.sleep(duration),
        }
    }
    fn sleep_until(&self, deadline: Instant) -> Box<dyn Sleep + Unpin> {
        match *self {
            None => {
                panic!("You must supply a timer.")
            }
            Some(ref t) => t.sleep_until(deadline),
        }
    }

    fn interval(&self, period: Duration) -> Box<dyn Interval> {
        match *self {
            None => {
                panic!("You must supply a timer.")
            }
            Some(ref t) => t.interval(period),
        }
    }
}