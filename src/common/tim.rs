use std::{
    fmt,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use futures_core::Future;

use crate::rt::{Interval, Sleep, Timer};

// Either the user provides a timer for background tasks, or we use
// `tokio::timer`.
#[derive(Clone)]
pub enum Tim {
    Default,
    Timer(Arc<dyn Timer + Send + Sync>),
}

impl fmt::Debug for Tim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tim").finish()
    }
}

impl Timer for Tim {
    fn sleep(&self, duration: Duration) -> Box<dyn Sleep + Unpin> {
        match *self {
            Tim::Default => {
                panic!("You must supply a timer.")
            }
            Tim::Timer(ref t) => t.sleep(duration),
        }
    }
    fn sleep_until(&self, deadline: Instant) -> Box<dyn Sleep + Unpin> {
        match *self {
            Tim::Default => {
                panic!("You must supply a timer.")
            }
            Tim::Timer(ref t) => t.sleep_until(deadline),
        }
    }

    fn interval(&self, period: Duration) -> Box<dyn Interval> {
        match *self {
            Tim::Default => {
                panic!("You must supply a timer.")
            },
            Tim::Timer(ref t) => t.interval(period),
        }
    }
}

/*
impl Interval for tokio::time::Interval {
    fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<std::time::Instant> {
        println!("interval polled");
        let raw = tokio::time::Interval::poll_tick(self, cx);
        println!("{:?}", raw);

        raw.map(|a| a.into_std())
    }
}
*/
