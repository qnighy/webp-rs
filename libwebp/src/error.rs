use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WebpUnknownError;

impl fmt::Display for WebpUnknownError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for WebpUnknownError {
    fn description(&self) -> &str {
        "webp library error"
    }
}
