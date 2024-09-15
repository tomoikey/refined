use std::fmt::{Debug, Display, Formatter};

/// A type indicating a failure to convert to `Refined`
#[derive(Debug)]
pub struct Error<T> {
    value: T,
    message: String,
}

impl<T> Error<T> {
    pub fn new(value: T, message: impl Into<String>) -> Self {
        Self {
            value,
            message: message.into(),
        }
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
