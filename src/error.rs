use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents an `NSError`-style failure returned by a `MetalKit` API.
pub struct MetalKitError {
    message: String,
}

impl MetalKitError {
    #[must_use]
    /// Creates a new `MetalKitError` message wrapper.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for MetalKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for MetalKitError {}
