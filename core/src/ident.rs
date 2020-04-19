#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigIdent(String);

impl BigIdent {
  pub fn new(s: &str) -> Self {
    Self(s.to_owned())
  }
}
