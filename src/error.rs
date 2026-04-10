use std::fmt;

/// Errors returned by opentui operations.
#[derive(Debug)]
pub enum Error {
  /// The Zig core returned a null pointer where a valid object was expected.
  CreationFailed(&'static str),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::CreationFailed(what) => write!(f, "failed to create {what}"),
    }
  }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
