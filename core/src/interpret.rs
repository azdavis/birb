//! Interpretation.

use crate::cst::{Field, TopDefn};
use crate::ident::Ident;
use crate::util::SliceDisplay;
use std::collections::HashMap;
use std::fmt;

/// Steps the expression `main()` in the given context to a value. Requires that the context be
/// statically checked and have a main function.
pub fn get<S>(_: HashMap<Ident, TopDefn, S>) -> Value
where
  S: std::hash::BuildHasher,
{
  todo!()
}

/// A value.
#[derive(Debug, PartialEq, Eq)]
pub enum Value {
  /// A string literal, like `"x"`.
  String_(String),
  /// A number (integer) literal, like `3`.
  Number(u64),
  /// A tuple, like `(1, "e")`.
  Tuple(Vec<Value>),
  /// A struct expression, like `Foo { x: 3 }`.
  Struct(Ident, Vec<Field<Value>>),
  /// A constructor, like `some(3)`.
  Ctor(Ident, Box<Value>),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::String_(s) => s.fmt(f),
      Self::Number(n) => n.fmt(f),
      Self::Tuple(vs) => SliceDisplay::new("(", vs, ")").fmt(f),
      Self::Struct(name, fs) => write!(f, "{} {{ {} }}", name, SliceDisplay::new("", fs, ""),),
      Self::Ctor(name, v) => write!(f, "{}({})", name, v),
    }
  }
}
