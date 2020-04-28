//! Interpretation.

use crate::cst::{Field, Kinded, TopDefn};
use crate::ident::Ident;
use std::collections::HashMap;

/// Steps the expression `main()` in the given context to a value. Requires that the context be
/// statically checked and have a main function.
pub fn get(cx: HashMap<Ident, TopDefn>) -> Value {
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
  Struct(Ident, Vec<Kinded>, Vec<Field<Value>>),
  /// A constructor, like `some(3)`.
  Ctor(Ident, Vec<Kinded>, Box<Value>),
  /// An identifier, like `a`.
  Ident(Ident),
}
