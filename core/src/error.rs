use std::fmt;

#[derive(Debug)]
pub enum Error {
  InvalidByte(u8),
  UnclosedString,
  InvalidUTF8(std::str::Utf8Error),
  InvalidNumber(std::num::ParseIntError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::InvalidByte(b) => write!(f, "invalid byte: {:x}", b),
      Self::UnclosedString => write!(f, "unclosed string literal"),
      Self::InvalidUTF8(ref e) => write!(f, "invalid utf-8: {}", e),
      Self::InvalidNumber(ref e) => write!(f, "invalid number: {}", e),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match *self {
      Self::InvalidUTF8(ref e) => Some(e),
      Self::InvalidNumber(ref e) => Some(e),
      Self::InvalidByte(_) | Self::UnclosedString => None,
    }
  }
}

pub type Result<T> = std::result::Result<T, Error>;
