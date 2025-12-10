use std::fmt::Display;

use tracing::Level;

pub struct ResultLogWrapper<T, E>(Level, Result<T, E>);

impl<T, E: Display> ResultLogWrapper<T, E> {
    pub fn log_err(self) -> Result<T, E> {
        match &self.1 {
            Ok(_) => {}
            Err(e) => match self.0 {
                Level::ERROR => tracing::error!("{}", e),
                Level::WARN => tracing::warn!("{}", e),
                Level::INFO => tracing::info!("{}", e),
                Level::DEBUG => tracing::debug!("{}", e),
                Level::TRACE => tracing::trace!("{}", e),
            },
        }
        self.1
    }

    pub fn log_and_map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
        self.log_err().map_err(op)
    }

    pub fn log_ok(self) -> Option<T> {
        self.log_err().ok()
    }
}

#[easy_ext::ext(ResultLogExt)]
pub impl<T, E: Display> Result<T, E> {
    fn error(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::ERROR, self)
    }

    fn warn(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::WARN, self)
    }

    fn info(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::INFO, self)
    }

    fn debug(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::DEBUG, self)
    }

    fn trace(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::TRACE, self)
    }
}
