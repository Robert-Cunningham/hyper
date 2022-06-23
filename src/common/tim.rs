use std::{fmt, sync::Arc, time::Duration};

use crate::rt::{Sleep, Timer};

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
    fn sleep(&self, duration: Duration) -> &mut dyn Sleep {
        match *self {
            Tim::Default => &mut tokio::time::sleep(duration),
            Tim::Timer(ref t) => t.sleep(duration),
        }
    }
}

/*
Box<dyn Sleep>

the trait `rt::Sleep` cannot be made into an object
`rt::Sleep` cannot be made into an objectrustcE0038
rt.rs(37, 54): for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>

*/
