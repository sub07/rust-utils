#[cfg(feature = "log-crate")]
mod log_crate;

#[cfg(feature = "tracing-crate")]
mod tracing_crate;

#[cfg(feature = "log-crate")]
pub use log_crate::*;

#[cfg(feature = "tracing-crate")]
pub use tracing_crate::*;
