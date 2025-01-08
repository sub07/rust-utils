use std::fmt::Display;

use log::error;

pub trait OptionToResultExt<T, E> {
    fn unwrap_or_fallible<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>;
}

impl<T, E> OptionToResultExt<T, E> for Option<T> {
    fn unwrap_or_fallible<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        match self {
            Some(value) => Ok(value),
            None => f(),
        }
    }
}

pub trait ResultLogExt<T, E> {
    #[must_use]
    fn log_err(self) -> Self;

    /// # Errors
    ///
    /// This function will return an error if self is an error.
    fn log_and_map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F>;

    #[must_use]
    fn log_ok(self) -> Option<T>;
}

impl<T, E: Display> ResultLogExt<T, E> for Result<T, E> {
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
