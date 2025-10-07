use std::{convert::Infallible, fmt::Display};

#[cfg(feature = "tracing-crate")]
use tracing::error;

#[cfg(feature = "log-crate")]
use log::error;

#[cfg(all(not(feature = "log-crate"), not(feature = "tracing-crate")))]
macro_rules! error {
    ($t:tt) => {{}};
}

#[easy_ext::ext(ResultLogExt)]
#[allow(
    clippy::return_self_not_must_use,
    reason = "easy ext prevent must use usage"
)]
pub impl<T, E: Display> Result<T, E> {
    #[inline]
    fn log_err(self) -> Self {
        if cfg!(any(feature = "log-crate", feature = "tracing-crate")) {
            self.inspect_err(|e| error!("{e}"))
        } else {
            self
        }
    }

    #[inline]
    fn log_and_map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
        self.log_err().map_err(op)
    }

    #[inline]
    fn log_ok(self) -> Option<T> {
        self.log_err().ok()
    }
}

#[easy_ext::ext(ResultInfallibleExt)]
pub impl<T> Result<T, Infallible> {
    fn unwrap_infallible(self) -> T {
        self.expect("Infallible")
    }
}

#[easy_ext::ext(ResultExt)]
pub impl<T, E> Result<T, E> {
    #[cfg(feature = "anyhow-crate")]
    fn into_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| anyhow::Error::msg(e.to_string()))
    }
}
