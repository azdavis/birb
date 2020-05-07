//! Interpretation.

use crate::cst::{Block, Expr, Field, Pat, Stmt, TopDefn};
use crate::ident::Ident;
use crate::util::SliceDisplay;
use std::collections::HashMap;
use std::fmt;

/// Steps the expression `main()` in the given context to a value. Requires that the context be
/// statically checked and have a main function.
pub fn get(cx: HashMap<Ident, TopDefn>) -> Value {
  let main = &cx[&Ident::new("main")];
  let main = match main {
    TopDefn::Fn_(x) => x,
    TopDefn::Struct(..) | TopDefn::Enum(..) => unreachable!(),
  };
  block_eval(&main.body, HashMap::new(), &cx)
}

fn block_eval(b: &Block, mut m: HashMap<Ident, Value>, cx: &HashMap<Ident, TopDefn>) -> Value {
  for s in b.stmts.iter() {
    let (pat, expr) = match s {
      Stmt::Let(p, _, e) => (p, e),
    };
    let val = expr_eval(expr, &m, cx);
    let mm = pat_match(pat, &val);
    let mm = mm.unwrap();
    m.extend(mm);
  }
  expr_eval(b.expr.as_ref().unwrap(), &m, cx)
}

fn pat_match(p: &Pat, v: &Value) -> Option<HashMap<Ident, Value>> {
  match (p, v) {
    (Pat::Wildcard, _) => Some(HashMap::new()),
    (Pat::String_(x), Value::String_(y)) => {
      if x == y {
        Some(HashMap::new())
      } else {
        None
      }
    }
    (Pat::Number(x), Value::Number(y)) => {
      if x == y {
        Some(HashMap::new())
      } else {
        None
      }
    }
    (Pat::Tuple(xs), Value::Tuple(ys)) => {
      let mut m = HashMap::new();
      for (x, y) in xs.iter().zip(ys) {
        match pat_match(x, y) {
          None => return None,
          Some(x) => m.extend(x),
        }
      }
      Some(m)
    }
    (Pat::Struct(..), _) => todo!("struct pat"),
    (Pat::Ctor(x, p), Value::Ctor(y, q)) => {
      if x == y {
        pat_match(&*p, &*q)
      } else {
        None
      }
    }
    (Pat::Ident(i), _) => {
      let mut m = HashMap::new();
      m.insert(i.clone(), v.clone());
      Some(m)
    }
    (Pat::Or(..), _) => todo!("or pat"),
    _ => None,
  }
}

fn expr_eval(e: &Expr, m: &HashMap<Ident, Value>, cx: &HashMap<Ident, TopDefn>) -> Value {
  match e {
    Expr::String_(x) => Value::String_(x.clone()),
    Expr::Number(x) => Value::Number(x.clone()),
    Expr::Tuple(xs) => {
      let mut t = Vec::with_capacity(xs.len());
      for x in xs {
        t.push(expr_eval(x, m, cx));
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
        t.push(Field::IdentAnd(i.clone(), expr_eval(&a, m, cx)));
      }
      Value::Struct(w.clone(), t)
    }
    Expr::Ident(i) => m[i].clone(),
    Expr::FnCall(i, _, xs) => {
      let mut vs = Vec::with_capacity(xs.len());
      for x in xs {
        vs.push(expr_eval(x, m, cx));
      }
      match cx.get(i) {
        Some(TopDefn::Fn_(f)) => {
          let mut m = m.clone();
          for (p, v) in f.params.iter().zip(vs) {
            m.insert(p.ident.clone(), v);
          }
          block_eval(&f.body, m, cx)
        }
        Some(TopDefn::Struct(..)) | Some(TopDefn::Enum(..)) => unreachable!(),
        None => {
          let v = vs.pop().unwrap();
          assert!(vs.is_empty());
          Value::Ctor(i.clone(), v.into())
        }
      }
    }
    Expr::FieldGet(..) => todo!(),
    Expr::MethodCall(..) => todo!(),
    Expr::Return(..) => todo!("eval return"),
    Expr::Match(..) => todo!(),
    Expr::Block(b) => block_eval(&*b, m.clone(), cx),
  }
}

/// A value.
#[derive(Debug, PartialEq, Eq, Clone)]
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
