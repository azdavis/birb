//! Errors.

use crate::cst::{Kind, Kinded};
use crate::ident::Ident;
use crate::parse::Found;
use std::fmt;

/// An error.
#[derive(Debug)]
pub enum Error {
  /// Invalid byte.
  InvalidByte(u8),
  /// Unclosed string literal.
  UnclosedString,
  /// Invalid UTF-8 in a string literal.
  InvalidUTF8(std::str::Utf8Error),
  /// Invalid number literal.
  InvalidNumber(std::num::ParseIntError),
  /// Parse error, where we expected one thing but found another thing.
  Parse(&'static str, Found),
  /// Empty kinded params, like `struct Foo[] { x: Nat }`.
  EmptyKindedParams,
  /// Empty kinded arguments, like `Foo[] { x: 3 }`.
  EmptyKindedArgs,
  /// Undefined identifier.
  UndefinedIdentifier(Ident),
  /// Kind mismatch, where we expected something to have the left Kind but it had the right Kind
  /// instead.
  MismatchedKinds(Kind, Kind),
  /// Incorrect number of arguments.
  WrongNumArgs(Ident, usize, usize),
  /// Application of a Kinded where the Kinded did not have Arrow kind.
  InvalidKindedApp(Ident, Kind),
  /// Duplicated field in a struct.
  DuplicateField(Ident, Ident),
  /// Duplicated identifier.
  DuplicateIdentifier(Ident),
  /// Undefined field in an struct expression or field get.
  NoSuchField(Ident, Ident),
  /// Type mismatch, where we expected something to have the left Kinded but it had the right Kinded
  /// instead.
  MismatchedTypes(Kinded, Kinded),
  /// Field get on something not of struct type.
  NotStruct(Ident),
  /// A pattern didn't make sense for this match.
  InvalidPattern(Kinded),
  /// No main function.
  NoMain,
  /// Invalid use of an effect in a function.
  InvalidEffectUse(Ident, Kinded),
  /// No expression given at the end of a block.
  NoExprForBlock,
  /// An empty match expression.
  EmptyMatch,
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
      Self::InvalidPattern(typ) => write!(f, "invalid pattern for type {}", typ),
      Self::NoMain => write!(f, "no main function"),
      Self::InvalidEffectUse(fn_, ef) => write!(f, "invalid use of effect {} in {}", ef, fn_),
      Self::NoExprForBlock => write!(f, "no expression at the end of the block"),
      Self::EmptyMatch => write!(f, "empty match expression"),
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
      | Self::NotStruct(..)
      | Self::InvalidPattern(..)
      | Self::NoMain
      | Self::InvalidEffectUse(..)
      | Self::NoExprForBlock
      | Self::EmptyMatch => None,
    }
  }
}

/// A shorthand for a Result where the error is our Error.
pub type Result<T> = std::result::Result<T, Error>;
