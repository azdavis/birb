//! Interpretation.

use crate::cst::{Field, Kinded, QualIdent, TopDefn};
use crate::ident::BigIdent;

/// Steps the expression `main()` in the given context to a value. Requires that the context be
/// statically checked and have a main function
pub fn get(cx: &[TopDefn]) -> Value {
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
  Struct(BigIdent, Vec<Kinded>, Vec<Field<Value>>),
  /// A qualified identifier, like `a` or `Bar::b`.
  QualIdent(QualIdent),
}
