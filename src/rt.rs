//! Runtime components
//!
//! The types in this module can be used to supply a runtime, like tokio.

use std::{
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant}, any::Any,
};

use futures_core::Future;

/// An executor of futures.
pub trait Executor<Fut> {
    /// Place the future into the executor to be run.
    fn execute(&self, fut: Fut);
}

/// A timer which provides timer-like functions, similar to tokio::time::*.
pub trait Timer : std::fmt::Debug {
    /// An analogue of tokio::time::sleep.
    fn sleep(&self, duration: Duration) -> Box<dyn Sleep + Unpin>;

    /// An analogue of tokio::time::sleep_until.
    fn sleep_until(&self, deadline: Instant) -> Box<dyn Sleep + Unpin>;

    /// An analogue of tokio::time::interval.
    fn interval(&self, period: Duration) -> Box<dyn Interval>;

    // An analogue of tokio::time::timeout.
    //fn timeout(&self, duration: Duration, future: Box<dyn Future<Output = Box<dyn Any>> + Unpin>) -> Box<tokio::time::Timeout<Box<dyn Any>>>;
    // I don't love the Box<dyn Any>, but I don't know a better way to implement this while keeping Timer object-safe.
}

/// The generic version of tokio::time::Sleep, which itself is the output of tokio::time::sleep
pub trait Sleep: Send + Sync + Unpin + Future<Output = ()> {

    /// An analogue of tokio::time::Sleep::deadline.
    fn deadline(&self) -> Instant;

    /// An analogue of tokio::time::Sleep::reset.
    fn reset(self: Pin<&mut Self>, deadline: Instant);

    /// An analogue of tokio::time::Sleep::is_elapsed.
    fn is_elapsed(&self) -> bool;
}

/// The generic version of tokio::time::Interval, which itself is the output of tokio::time::sleep
pub trait Interval: Send + Sync {

    /// An analogue of tokio::time::Interval::is_elapsed.
    fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<Instant>;
}
