use std::{convert::Infallible, fmt::Display};

use log::error;

#[easy_ext::ext(ResultLogExt)]
#[allow(
    clippy::return_self_not_must_use,
    reason = "easy ext prevent must use usage"
)]
pub impl<T, E: Display> Result<T, E> {
    #[inline]
    fn log_err(self) -> Self {
        self.inspect_err(|e| error!("{e}"))
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
