//! Interpretation.

use crate::cst::{Expr, Field, Kinded, Stmt, TopDefn};
use crate::ident::Ident;
use crate::util::SliceDisplay;
use std::collections::HashMap;
use std::fmt;

/// Steps the expression `main()` in the given context to a value. Requires that the context be
/// statically checked and have a main function.
pub fn get<S>(cx: HashMap<Ident, TopDefn, S>) -> Value
where
  S: std::hash::BuildHasher,
{
  let main = &cx[&Ident::new("main")];
  let main = match main {
    TopDefn::Fn_(x) => x,
    TopDefn::Struct(..) | TopDefn::Enum(..) => unreachable!(),
  };
  let main_body = &main.body;
  for stmt in &main_body.stmts {
    let (pat, expr) = match stmt {
      Stmt::Let(p, _, e) => (p, e),
    };
    let val = helper(expr);
  }
  todo!()
}

fn helper(e: &Expr) -> Value {
  match e {
    Expr::String_(x) => Value::String_(x.clone()),
    Expr::Number(x) => Value::Number(x.clone()),
    Expr::Tuple(xs) => {
      let mut t = Vec::with_capacity(xs.len());
      for x in xs {
        t.push(helper(x));
      }
      Value::Tuple(t)
    }
    Expr::Struct(w, _, xs) => {
      let mut t = Vec::with_capacity(xs.len());
      for x in xs {
        let (i, a) = match x {
          Field::Ident(i) => (i, Expr::Ident(i.clone())),
          Field::IdentAnd(i, j) => (i, j.clone()),
        };
        t.push(Field::IdentAnd(i.clone(), helper(&a)));
      }
      Value::Struct(w.clone(), t)
    }
    Expr::Ident(i) => todo!(),
    Expr::FnCall(..) => todo!(),
    Expr::FieldGet(..) => todo!(),
    Expr::MethodCall(..) => todo!(),
    Expr::Return(..) => todo!(),
    Expr::Match(..) => todo!(),
    Expr::Block(..) => todo!(),
  }
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
