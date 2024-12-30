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
