//! Runtime components
//!
//! By default, hyper includes the [tokio](https://tokio.rs) runtime.
//!
//! If the `runtime` feature is disabled, the types in this module can be used
//! to plug in other runtimes.

use std::{
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use futures_core::Future;

/// An executor of futures.
pub trait Executor<Fut> {
    /// Place the future into the executor to be run.
    fn execute(&self, fut: Fut);
}

pub trait Timer : std::fmt::Debug {
    fn sleep(&self, duration: Duration) -> Box<dyn Sleep + Unpin>;
    fn sleep_until(&self, deadline: Instant) -> Box<dyn Sleep + Unpin>;
    fn interval(&self, period: Duration) -> Box<dyn Interval>;
}

/*
impl Sleep for HasSleep {
    fn is_elapsed(&self) -> bool {
        self.sleep.is_elapsed()
    }
    fn deadline(&self) -> Instant {
        self.sleep.deadline().into()
    }
    fn reset(mut self: Pin<&mut Self>, deadline: Instant) {
        self.sleep.as_mut().reset(deadline.into())
    }
}
*/

// The generic version of tokio::time::Sleep, which itself is the output of tokio::time::sleep
pub trait Sleep: Send + Sync + Unpin + Future<Output = ()> {
    fn deadline(&self) -> Instant;
    fn reset(self: Pin<&mut Self>, deadline: Instant);
    fn is_elapsed(&self) -> bool;
}

// The generic version of tokio::time::Interval, which itself is the output of tokio::time::sleep
pub trait Interval: Send + Sync {
    fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<Instant>;
}
