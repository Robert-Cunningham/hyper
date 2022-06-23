use std::{fmt, sync::Arc, time::Duration};

use crate::rt::Timer;

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

impl Tim {
    pub(crate) fn sleep(duration: Duration) {
        match *self {
            Tim::Default => tokio::time::sleep(duration),
            Tim::Timer(ref t) => t.sleep(duration),
        }
    }
}
