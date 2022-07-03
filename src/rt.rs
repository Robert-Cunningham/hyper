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

use futures_core::{Future, future::BoxFuture};

/// An executor of futures.
pub trait Executor<Fut> {
    /// Place the future into the executor to be run.
    fn execute(&self, fut: Fut);
}

pub trait Timer: Send + Sync + std::fmt::Debug + Clone + 'static {
    fn sleep(duration: Duration) -> Box<dyn Sleep + Unpin>;
    fn sleep_until(deadline: Instant) -> Box<dyn Sleep + Unpin>;
    fn interval(period: Duration) -> Box<dyn Interval>;
    //fn timeout<O, T: Future<Output = O>>(duration: Duration, future: T) -> Box<dyn Timeout<O> + Unpin>;
    //fn timeout<T: Future>(duration: Duration, future: T) -> Box<dyn Timeout<T> + Unpin>;
}

// TokioTimer::timeout should work just like tokio::time::timeout.
// Timer must require the same function signature as tokio::time::timeout.

#[derive(Clone, Debug)]
pub struct UnimplemenetedTimer;

impl Timer for UnimplemenetedTimer {
    fn sleep(_duration: Duration) -> Box<dyn Sleep + Unpin> {
        panic!("Need to configure a timer.")
    }
    fn sleep_until(_deadline: Instant) -> Box<dyn Sleep + Unpin> {
        panic!("Need to configure a timer.")
    }
    fn interval(_period: Duration) -> Box<dyn Interval> {
        panic!("Need to configure a timer.")
    }
    /*
    fn timeout<O, T: Future<Output = O>>(_duration: Duration, _future: T) -> Box<dyn Timeout<O> + Unpin> {
        panic!("Need to configure a timer.")
    }
    */
}

pub trait Timer2 {
    fn sleep(duration: Duration) -> Box<dyn Sleep + Unpin>;
    //fn timeout<T>(&self, duration: Duration, future: T) -> Box<dyn Timeout<T>>;
}

// The generic version of tokio::time::Sleep, which itself is the output of tokio::time::sleep
pub trait Sleep: Send + Sync + Future<Output = ()> {
    fn deadline(&self) -> Instant;
    fn reset(self: Pin<&mut Self>, deadline: Instant);
    fn is_elapsed(&self) -> bool;
}

// The generic version of tokio::time::Interval, which itself is the output of tokio::time::sleep
pub trait Interval: Send + Sync {
    fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<Instant>;
}

// pub trait Timeout<O>: Send + Sync + Future<Output = Result<O, tokio::time::error::Elapsed>> {
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<O>;
// }

/*
pub trait Timeout: Send + Sync + Future {
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll< Result<O::Output, tokio::time::error::Elapsed> >;
}
*/