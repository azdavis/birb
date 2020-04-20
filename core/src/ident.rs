use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }
}

impl fmt::Display for Ident {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigIdent(String);

impl BigIdent {
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }
}

impl fmt::Display for BigIdent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}
