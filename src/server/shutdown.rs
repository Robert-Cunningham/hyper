use std::error::Error as StdError;

use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::debug;

use super::accept::Accept;
use super::conn::UpgradeableConnection;
use super::server::{Server, Watcher};
use crate::body::{Body, HttpBody};
use crate::common::drain::{self, Draining, Signal, Watch, Watching};
use crate::common::exec::{ConnStreamExec, NewSvcExec};
use crate::common::{task, Future, Pin, Poll, Unpin};
use crate::service::{HttpService, MakeServiceRef};

use crate::rt::Timer;

pin_project! {
    #[allow(missing_debug_implementations)]
    pub struct Graceful<I, S, F, M, E> {
        #[pin]
        state: State<I, S, F, M, E>,
    }
}

pin_project! {
    #[project = StateProj]
    pub(super) enum State<I, S, F, M, E> {
        Running {
            drain: Option<(Signal, Watch)>,
            #[pin]
            server: Server<I, S, M, E>,
            #[pin]
            signal: F,
        },
        Draining { draining: Draining },
    }
}

impl<I, S, F, M, E> Graceful<I, S, F, M, E> {
    pub(super) fn new(server: Server<I, S, M, E>, signal: F) -> Self {
        let drain = Some(drain::channel());
        Graceful {
            state: State::Running {
                drain,
                server,
                signal,
            },
        }
    }
}

impl<I, IO, IE, S, B, F, M, E> Future for Graceful<I, S, F, M, E>
where
    I: Accept<Conn = IO, Error = IE>,
    IE: Into<Box<dyn StdError + Send + Sync>>,
    IO: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: MakeServiceRef<IO, Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    F: Future<Output = ()>,
    E: ConnStreamExec<<S::Service as HttpService<Body>>::Future, B>,
    E: NewSvcExec<IO, S::Future, S::Service, M, E, GracefulWatcher>,
    M: Timer + Send + Sync
{
    type Output = crate::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();
        loop {
            let next = {
                match me.state.as_mut().project() {
                    StateProj::Running {
                        drain,
                        server,
                        signal,
                    } => match signal.poll(cx) {
                        Poll::Ready(()) => {
                            debug!("signal received, starting graceful shutdown");
                            let sig = drain.take().expect("drain channel").0;
                            State::Draining {
                                draining: sig.drain(),
                            }
                        }
                        Poll::Pending => {
                            let watch = drain.as_ref().expect("drain channel").1.clone();
                            return server.poll_watch(cx, &GracefulWatcher(watch));
                        }
                    },
                    StateProj::Draining { ref mut draining } => {
                        return Pin::new(draining).poll(cx).map(Ok);
                    }
                }
            };
            me.state.set(next);
        }
    }
}

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct GracefulWatcher(Watch);

impl<I, S, M, E> Watcher<I, S, M, E> for GracefulWatcher
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: HttpService<Body>,
    E: ConnStreamExec<S::Future, S::ResBody>,
    S::ResBody: 'static,
    <S::ResBody as HttpBody>::Error: Into<Box<dyn StdError + Send + Sync>>,
    M: Timer + Send + Sync
{
    type Future =
        Watching<UpgradeableConnection<I, S, M, E>, fn(Pin<&mut UpgradeableConnection<I, S, M, E>>)>;

    fn watch(&self, conn: UpgradeableConnection<I, S, M, E>) -> Self::Future {
        self.0.clone().watch(conn, on_drain)
    }
}

fn on_drain<I, S, M, E>(conn: Pin<&mut UpgradeableConnection<I, S, M, E>>)
where
    S: HttpService<Body>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    I: AsyncRead + AsyncWrite + Unpin,
    S::ResBody: HttpBody + 'static,
    <S::ResBody as HttpBody>::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: ConnStreamExec<S::Future, S::ResBody>,
    M: Timer + Send + Sync,
{
    conn.graceful_shutdown()
}
