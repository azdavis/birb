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

/// An identifier, big or non-big.
#[derive(Debug)]
pub enum Identifier {
  /// A regular (small) identifier.
  Ident(Ident),
  /// A big identifier.
  BigIdent(BigIdent),
}

impl fmt::Display for Identifier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Ident(id) => id.fmt(f),
      Self::BigIdent(bi) => bi.fmt(f),
    }
  }
}

impl From<Ident> for Identifier {
  fn from(val: Ident) -> Self {
    Self::Ident(val)
  }
}

impl From<BigIdent> for Identifier {
  fn from(val: BigIdent) -> Self {
    Self::BigIdent(val)
  }
}
