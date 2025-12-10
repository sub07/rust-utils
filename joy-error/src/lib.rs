use std::convert::Infallible;

#[cfg(any(feature = "log-crate", feature = "tracing-crate"))]
pub mod log;

#[cfg(all(not(feature = "log-crate"), not(feature = "tracing-crate")))]
macro_rules! error {
    ($t:tt) => {{}};
}

#[cfg(feature = "anyhow-crate")]
#[easy_ext::ext(AnyhowResultLogExt)]
#[allow(
    clippy::return_self_not_must_use,
    reason = "easy ext prevent must use usage"
)]
pub impl<T, E: std::fmt::Display> Result<T, E> {
    fn into_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| anyhow::Error::msg(e.to_string()))
    }
}

#[easy_ext::ext(ResultInfallibleExt)]
pub impl<T> Result<T, Infallible> {
    fn unwrap_infallible(self) -> T {
        self.expect("Infallible")
    }
}
