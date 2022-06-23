//! Runtime components
//!
//! By default, hyper includes the [tokio](https://tokio.rs) runtime.
//!
//! If the `runtime` feature is disabled, the types in this module can be used
//! to plug in other runtimes.

use std::{
    panic::Location,
    time::{Duration, Instant},
};

use futures_core::Future;

/// An executor of futures.
pub trait Executor<Fut> {
    /// Place the future into the executor to be run.
    fn execute(&self, fut: Fut);
}

pub trait Timer {
    fn sleep(&self, duration: Duration) -> &mut dyn Sleep;
}

impl Sleep for tokio::time::Sleep {
    fn new_timeout(
        deadline: Instant,
        location: Option<&'static Location<'static>>,
    ) -> tokio::time::Sleep {
        // cannot implement since new_timeout doesn't take &self as its first option?
        tokio::time::Sleep::new_timeout(deadline, location)
    }
}

// If Sleep is Sized, it's not object safe.

// If Sleep is not Sized, we can't await
// (*Tim::Default.sleep(timeout)).await;
// the size for values of type `dyn rt::Sleep<Output = ()>` cannot be known at compilation time
// the trait `Sized` is not implemented for `dyn rt::Sleep<Output = ()>`
// required because of the requirements on the impl of `std::future::IntoFuture` for `dyn rt::Sleep<Output = ()>`rustcE0277

// The generic version of tokio::time::Sleep, which itself is the output of tokio::time::sleep
pub trait Sleep: Send + Sync + Future<Output = ()> {
    fn new_timeout(deadline: Instant, location: Option<&'static Location<'static>>) -> Self
    where
        Self: Sized;
}
pub trait Interval: Send + Sync {}
