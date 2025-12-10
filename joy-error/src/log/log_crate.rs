use std::fmt::Display;

use log::Level;

pub struct ResultLogWrapper<T, E>(Level, Result<T, E>);

impl<T, E: Display> ResultLogWrapper<T, E> {
    pub fn log_err(self) -> Result<T, E> {
        match &self.1 {
            Ok(_) => {}
            Err(e) => {
                log::log!(self.0, "{}", e);
            }
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
// #[allow(
//     clippy::return_self_not_must_use,
//     reason = "easy ext prevent must use usage"
// )]
pub impl<T, E: Display> Result<T, E> {
    fn error(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::Error, self)
    }

    fn warn(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::Warn, self)
    }

    fn info(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::Info, self)
    }

    fn debug(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::Debug, self)
    }

    fn trace(self) -> ResultLogWrapper<T, E> {
        ResultLogWrapper(Level::Trace, self)
    }
}
