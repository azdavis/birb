//! Identifiers.

use std::fmt;

/// An identifier.
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
    write!(f, "{}", self.0)
  }
}
