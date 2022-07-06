use std::{
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};

use futures_core::Future;

use crate::rt::{Interval, Sleep, Timer};

// Either the user provides a timer for background tasks, or we use
// `tokio::timer`.
#[derive(Clone)]
pub enum Tim { // might swap this into an option<arc<dyn timer + send _ sync>>
    None,
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
            Tim::None => {
                panic!("You must supply a timer.")
            }
            Tim::Timer(ref t) => t.sleep(duration),
        }
    }
    fn sleep_until(&self, deadline: Instant) -> Box<dyn Sleep + Unpin> {
        match *self {
            Tim::None => {
                panic!("You must supply a timer.")
            }
            Tim::Timer(ref t) => t.sleep_until(deadline),
        }
    }

    fn interval(&self, period: Duration) -> Box<dyn Interval> {
        match *self {
            Tim::None => {
                panic!("You must supply a timer.")
            }
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

// #[derive(Clone, Debug)]
// pub struct TokioTimer;
//
// impl Timer for TokioTimer {
//     fn sleep(&self, duration: Duration) -> Box<dyn Sleep + Unpin> {
//         let s = tokio::time::sleep(duration);
//         let hs = TokioSleep { inner: Box::pin(s) };
//         return Box::new(hs);
//     }
//
//     fn sleep_until(&self, deadline: Instant) -> Box<dyn Sleep + Unpin> {
//         return Box::new(TokioSleep {
//             inner: Box::pin(tokio::time::sleep_until(deadline.into())),
//         });
//     }
//
//     fn interval(&self, period: Duration) -> Box<dyn Interval> {
//         Box::new(TokioInterval {
//             inner: tokio::time::interval(period),
//         })
//     }
//
//     /*
//     fn timeout<O, T: Future<Output = O>>(duration: Duration, future: T) -> Box<dyn Timeout<O> + Unpin> {
//         tokio::time::timeout(duration, future)
//     }
//     */
// }
//
// /// An Interval object that uses the tokio runtime.
// pub struct TokioInterval {
//     inner: tokio::time::Interval,
// }
//
// impl Interval for TokioInterval {
//     fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<std::time::Instant> {
//         let raw = tokio::time::Interval::poll_tick(&mut self.inner, cx);
//         raw.map(|a| a.into_std())
//     }
// }
//
// struct TokioTimeout<T> {
//     inner: Pin<Box<tokio::time::Timeout<T>>>,
// }
//
// impl<T> Future for TokioTimeout<T> where T: Future {
//     type Output = Result<T::Output, tokio::time::error::Elapsed>;
//
//     fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
//         self.inner.as_mut().poll(context)
//     }
//
// }
//
// /*
// impl<T> Timeout<T> for TokioTimeout<T>
// where
//     T: Future + Send + Sync,
// {
//     //type Output = ;
//
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<T, Elapsed>> {
//         self.inner.as_mut().poll(cx)
//     }
// }
// */
//
// // Use TokioSleep to get tokio::time::Sleep to implement Unpin.
// // see https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html
// pub(crate) struct TokioSleep {
//     pub(crate) inner: Pin<Box<tokio::time::Sleep>>,
// }
//
// impl Future for TokioSleep {
//     type Output = ();
//
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         self.inner.as_mut().poll(cx)
//     }
// }
//
// // Use HasSleep to get tokio::time::Sleep to implement Unpin.
// // see https://docs.rs/tokio/latest/tokio/time/struct.Sleep.html
//
// impl Sleep for TokioSleep {
//     fn is_elapsed(&self) -> bool {
//         self.inner.is_elapsed()
//     }
//     fn deadline(&self) -> Instant {
//         self.inner.deadline().into()
//     }
//     fn reset(mut self: Pin<&mut Self>, deadline: Instant) {
//         self.inner.as_mut().reset(deadline.into())
//     }
// }
//
