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
                let s = tokio::time::sleep(duration);
                //tokio::pin!(s);
                let hs = HasSleep { sleep: Box::pin(s) };
                return Box::new(hs);
            }
            Tim::Timer(ref t) => t.sleep(duration),
        }
    }
    fn sleep_until(&self, deadline: Instant) -> Box<dyn Sleep + Unpin> {
        match *self {
            Tim::Default => {
                return Box::new(HasSleep {
                    sleep: Box::pin(tokio::time::sleep_until(deadline.into())),
                })
            }
            Tim::Timer(ref t) => t.sleep_until(deadline),
        }
    }

    fn interval(&self, period: Duration) -> Box<dyn Interval> {
        match *self {
            Tim::Default => Box::new(tokio::time::interval(period)),
            Tim::Timer(ref t) => t.interval(period),
        }
    }

    fn pause(&self) {
        match *self {
            Tim::Default => tokio::time::pause(),
            Tim::Timer(ref t) => t.pause(),
        }
    }

    fn advance(&self, duration: Duration) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        match *self {
            Tim::Default => Box::pin(tokio::time::advance(duration)),
            Tim::Timer(ref t) => t.advance(duration),
        }
    }
}

impl Interval for tokio::time::Interval {
    fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<Instant> {
        tokio::time::Interval::poll_tick(self, cx).map(|a| a.into())
    }
}

/*
pub(crate) struct HasInterval {
    pub(crate) interval: Pin<Box<tokio::time::Interval>>,
}

impl Future for HasInterval {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        return self.interval.as_mut().poll(cx);
    }
}
*/

// Use HasSleep to get tokio::time::Sleep to implement Unpin.
// see https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html
pub(crate) struct HasSleep {
    pub(crate) sleep: Pin<Box<tokio::time::Sleep>>,
}

impl Future for HasSleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.sleep.as_mut().poll(cx)
    }
}

/*
Box<dyn Sleep>

the trait `rt::Sleep` cannot be made into an object
`rt::Sleep` cannot be made into an objectrustcE0038
rt.rs(37, 54): for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

*/
