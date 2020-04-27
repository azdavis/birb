//! Errors.

use crate::cst::{Kind, Kinded};
use crate::ident::Ident;
use crate::parse::Found;
use std::fmt;

/// An error.
#[derive(Debug)]
pub enum Error {
  /// An invalid byte was found.
  InvalidByte(u8),
  /// An unclosed string literal was found.
  UnclosedString,
  /// Invalid UTF-8 was found in a string literal.
  InvalidUTF8(std::str::Utf8Error),
  /// A number literal was invalid.
  InvalidNumber(std::num::ParseIntError),
  /// A parse error occurred, where we expected one thing but found another thing.
  Parse(&'static str, Found),
  /// There were empty kinded params, like `struct Foo[] { x: Int }`.
  EmptyKindedParams,
  /// There were empty kinded arguments, like `Foo[] { x: 3 }`.
  EmptyKindedArgs,
  /// There was an undefined identifier.
  UndefinedIdentifier(Ident),
  /// There was a kind mismatch, where we expected something to have the left Kind but it had the
  /// right Kind instead.
  MismatchedKinds(Kind, Kind),
  /// There was an incorrect number of Kinded arguments.
  WrongNumArgs(Ident, usize, usize),
  /// There was a application of a Kinded where the Kinded did not have Arrow kind.
  InvalidKindedApp(Ident, Kind),
  /// There was a duplicated field in a struct.
  DuplicateField(Ident, Ident),
  /// There was a duplicated identifier.
  DuplicateIdentifier(Ident),
  /// There was an undefined field in an struct expression or field get.
  NoSuchField(Ident, Ident),
  /// There was a type mismatch, where we expected something to have the left Kinded but it had the
  /// right Kinded instead.
  MismatchedTypes(Kinded, Kinded),
  /// There was a field get on something not of struct type.
  NotStruct(Ident),
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
      Self::EmptyKindedParams => write!(f, "empty type/effect params"),
      Self::EmptyKindedArgs => write!(f, "empty type/effect args"),
      Self::UndefinedIdentifier(id) => write!(f, "undefined identifier: {}", id),
      Self::MismatchedKinds(expected, found) => write!(
        f,
        "mismatched kinds: expected {}, found {}",
        expected, found
      ),
      Self::WrongNumArgs(id, expected, found) => write!(
        f,
        "wrong number of arguments for {}: expected {}, found {}",
        id, expected, found
      ),
      Self::InvalidKindedApp(bi, found) => write!(
        f,
        "invalid kind for {}: expected an arrow kind, found {}",
        bi, found
      ),
      Self::DuplicateField(struct_, field) => {
        write!(f, "duplicate field for {}: {}", struct_, field)
      }
      Self::DuplicateIdentifier(id) => write!(f, "duplicate identifier: {}", id),
      Self::NoSuchField(struct_, field) => write!(f, "no such field for {}: {}", struct_, field),
      Self::MismatchedTypes(expected, found) => write!(
        f,
        "mismatched types: expected {}, found {}",
        expected, found
      ),
      Self::NotStruct(field) => write!(f, "cannot get field {} of non-struct type", field),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::InvalidUTF8(e) => Some(e),
      Self::InvalidNumber(e) => Some(e),
      Self::InvalidByte(..)
      | Self::UnclosedString
      | Self::Parse(..)
      | Self::EmptyKindedParams
      | Self::EmptyKindedArgs
      | Self::UndefinedIdentifier(..)
      | Self::MismatchedKinds(..)
      | Self::WrongNumArgs(..)
      | Self::InvalidKindedApp(..)
      | Self::DuplicateField(..)
      | Self::DuplicateIdentifier(..)
      | Self::NoSuchField(..)
      | Self::MismatchedTypes(..)
      | Self::NotStruct(..) => None,
    }
  }
}

/// A shorthand for a Result where the error is our Error.
pub type Result<T> = std::result::Result<T, Error>;
