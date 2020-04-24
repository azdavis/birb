//! Identifiers.

use std::fmt;

/// A regular identifier, which starts with a lowercase letter and is used for function and variable
/// names.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl Ident {
  /// Construct a new Ident.
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }
}

impl fmt::Display for Ident {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

/// A big identifier, which starts with an uppercase letter and is used for type, effect, and kind
/// names.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BigIdent(String);

impl BigIdent {
  /// Construct a new BigIdent.
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }
}

impl fmt::Display for BigIdent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}
