use crate::cst::{Kind, Type};
use crate::ident::{BigIdent, Ident};
use crate::parse::Found;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  InvalidByte(u8),
  UnclosedString,
  InvalidUTF8(std::str::Utf8Error),
  InvalidNumber(std::num::ParseIntError),
  Parse(&'static str, Found),
  UndefinedKind(BigIdent),
  UndefinedType(BigIdent),
  UndefinedEffect(BigIdent),
  MismatchedTypeKinds(Type, Kind),
  MismatchedEffectKinds(BigIdent, Kind),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidByte(b) => write!(f, "invalid byte: {:x}", b),
      Self::UnclosedString => write!(f, "unclosed string literal"),
      Self::InvalidUTF8(e) => write!(f, "invalid utf-8: {}", e),
      Self::InvalidNumber(e) => write!(f, "invalid number: {}", e),
      Self::Parse(expected, found) => {
        write!(f, "parse error: expected {}, found {}", expected, found)
      }
      Self::UndefinedKind(bi) => write!(f, "undefined kind: {}", bi),
      Self::UndefinedType(bi) => write!(f, "undefined type: {}", bi),
      Self::UndefinedEffect(bi) => write!(f, "undefined effect: {}", bi),
      Self::MismatchedTypeKinds(t, found) => write!(
        f,
        "mismatched kinds for {}: expected Type, found {}",
        t, found
      ),
      Self::MismatchedEffectKinds(bi, found) => write!(
        f,
        "mismatched kinds for {}: expected Effect, found {}",
        bi, found
      ),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::InvalidUTF8(e) => Some(e),
      Self::InvalidNumber(e) => Some(e),
      Self::InvalidByte(_)
      | Self::UnclosedString
      | Self::Parse(..)
      | Self::UndefinedKind(..)
      | Self::UndefinedType(..)
      | Self::UndefinedEffect(..)
      | Self::MismatchedTypeKinds(..)
      | Self::MismatchedEffectKinds(..) => None,
    }
  }
}

pub type Result<T> = std::result::Result<T, Error>;
